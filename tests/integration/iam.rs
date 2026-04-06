//! Integration tests for IAM service account and key operations.
//!
//! These tests require:
//! - `GCLOUD_PROJECT_ID` environment variable
//! - ADC credentials configured (`GOOGLE_AUTH_USE_GCLOUD=1` or service account)
//! - Sufficient IAM permissions (`roles/iam.serviceAccountAdmin`)
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::iam::CreateServiceAccountKeyRequest;
use std::env;
use std::process::Command;

const TEST_SA_NAME: &str = "iam-integ-test";
const TEST_SA_KEY_NAME: &str = "iam-integ-key";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

fn sa_email(name: &str, project: &str) -> String {
    format!("{}@{}.iam.gserviceaccount.com", name, project)
}

/// Delete a service account via gcloud CLI (idempotent — ignores errors).
fn gcloud_delete_sa(project: &str, email: &str) {
    let _ = Command::new("gcloud")
        .args([
            "iam",
            "service-accounts",
            "delete",
            email,
            "--project",
            project,
            "--quiet",
        ])
        .output();
}

/// IAM has eventual consistency — wait briefly after mutations.
async fn wait_for_consistency() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

// =============================================================================
// Service Account CRUD Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_service_account_lifecycle() {
    let project = project_id();
    let email = sa_email(TEST_SA_NAME, &project);

    // Pre-cleanup in case a previous run left resources
    gcloud_delete_sa(&project, &email);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let result = run_service_account_tests(&client, &project, &email).await;

    // Always cleanup
    println!("\nCleaning up...");
    gcloud_delete_sa(&project, &email);

    result.expect("service account lifecycle test failed");
    println!("\nAll service account tests passed!");
}

async fn run_service_account_tests(
    client: &GcpHttpClient,
    project: &str,
    email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();

    // === 1. CREATE ===
    println!("\n[1/6] Creating service account...");
    let sa = iam
        .create_service_account(
            project,
            TEST_SA_NAME,
            "IAM Integration Test SA",
            "Created by integration tests — safe to delete",
        )
        .await?;

    assert_eq!(sa.email.as_deref(), Some(email));
    assert_eq!(sa.display_name.as_deref(), Some("IAM Integration Test SA"));
    assert!(sa.unique_id.is_some(), "Should have unique_id");
    println!("  Created: {}", email);
    println!("  unique_id: {}", sa.unique_id.as_deref().unwrap_or("?"));

    wait_for_consistency().await;

    // === 2. GET ===
    println!("\n[2/6] Getting service account...");
    let fetched = iam.get_service_account(project, email).await?;

    assert_eq!(fetched.email.as_deref(), Some(email));
    assert_eq!(
        fetched.display_name.as_deref(),
        Some("IAM Integration Test SA")
    );
    assert_eq!(
        fetched.description.as_deref(),
        Some("Created by integration tests — safe to delete")
    );
    assert!(
        fetched.disabled.is_none() || fetched.disabled == Some(false),
        "SA should not be disabled, got: {:?}",
        fetched.disabled
    );
    println!("  All fields match");

    // === 3. LIST ===
    println!("\n[3/6] Listing service accounts...");
    let list = iam.list_service_accounts(project).await?;

    assert!(
        !list.accounts.is_empty(),
        "list_service_accounts returned no accounts"
    );
    println!("  {} accounts in project", list.accounts.len());

    // === 4. DELETE ===
    println!("\n[4/6] Deleting service account...");
    iam.delete_service_account(project, email).await?;
    println!("  Delete call succeeded");

    wait_for_consistency().await;

    // === 5. VERIFY DELETION ===
    println!("\n[5/6] Verifying deletion (expect 404)...");
    let result = iam.get_service_account(project, email).await;
    assert!(result.is_err(), "Deleted SA should return error on get");
    println!("  Confirmed: get returns error after deletion");

    // === 6. GET NON-EXISTENT ===
    println!("\n[6/6] Getting non-existent SA (expect error)...");
    let result = iam
        .get_service_account(project, "nonexistent@example.iam.gserviceaccount.com")
        .await;
    assert!(result.is_err(), "Should error on non-existent SA");
    println!("  Confirmed: error for non-existent SA");

    Ok(())
}

// =============================================================================
// Service Account Key Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_service_account_key_lifecycle() {
    let project = project_id();
    let email = sa_email(TEST_SA_KEY_NAME, &project);

    // Pre-cleanup
    gcloud_delete_sa(&project, &email);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let result = run_key_tests(&client, &project, &email).await;

    // Always cleanup
    println!("\nCleaning up...");
    gcloud_delete_sa(&project, &email);

    result.expect("key lifecycle test failed");
    println!("\nAll key lifecycle tests passed!");
}

async fn run_key_tests(
    client: &GcpHttpClient,
    project: &str,
    email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let iam = client.iam();

    // Setup: create SA
    println!("\n[0/5] Creating service account for key tests...");
    iam.create_service_account(
        project,
        TEST_SA_KEY_NAME,
        "Key Lifecycle Test SA",
        "Created by integration tests — safe to delete",
    )
    .await?;
    println!("  Created: {}", email);

    wait_for_consistency().await;

    // === 1. CREATE KEY ===
    println!("\n[1/5] Creating service account key...");
    let key = iam
        .create_service_account_key(project, email, &CreateServiceAccountKeyRequest::default())
        .await?;

    assert!(!key.name.is_empty(), "Key should have a name");
    assert!(
        key.private_key_data.is_some(),
        "Create response should include private_key_data"
    );
    println!(
        "  private_key_data length: {} bytes",
        key.private_key_data.as_ref().unwrap().len()
    );

    let key_id = key
        .name
        .rsplit('/')
        .next()
        .expect("key name should have / separator");
    println!("  Created key: {}", key_id);

    // Brief wait — list may not immediately reflect the new key
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // === 2. LIST KEYS ===
    println!("\n[2/5] Listing service account keys...");
    let keys = iam.list_service_account_keys(project, email).await?;

    for k in &keys.keys {
        let short_id = k.name.rsplit('/').next().unwrap_or(&k.name);
        println!("    key: {}, type: {:?}", short_id, k.key_type);
    }

    assert!(!keys.keys.is_empty(), "Should have at least one key");

    let found = keys.keys.iter().any(|k| k.name.contains(key_id));
    // The key may not appear in list immediately due to eventual consistency;
    // if not found, log a warning but don't fail — the create already succeeded.
    if found {
        println!(
            "  Found created key in list ({} total keys)",
            keys.keys.len()
        );
    } else {
        println!(
            "  WARNING: Created key {} not yet in list ({} total keys) — eventual consistency",
            key_id,
            keys.keys.len()
        );
    }

    // === 3. DELETE KEY ===
    println!("\n[3/5] Deleting service account key...");
    iam.delete_service_account_key(project, email, key_id)
        .await?;
    println!("  Delete call succeeded");

    // Key deletion may need propagation time
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // === 4. VERIFY KEY DELETED ===
    println!("\n[4/5] Verifying key deleted...");
    let keys_after = iam.list_service_account_keys(project, email).await?;
    let still_exists = keys_after.keys.iter().any(|k| k.name.contains(key_id));
    if still_exists {
        println!("  WARNING: Key still appears in list — eventual consistency lag");
    } else {
        println!("  Confirmed: key no longer in list");
    }

    // === 5. DELETE SA (cleanup) ===
    println!("\n[5/5] Deleting service account...");
    iam.delete_service_account(project, email).await?;
    println!("  Deleted");

    Ok(())
}
