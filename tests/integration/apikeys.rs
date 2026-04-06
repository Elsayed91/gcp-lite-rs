//! Integration tests for API Keys API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 1.12 (iam_api_keys_active_services): inspect restrictions.apiTargets
//!   - CIS 1.13 (iam_api_keys_restricted_hosts): inspect browser/server key restrictions
//!   - CIS 1.14 (iam_api_keys_restricted_apis): inspect restrictions.apiTargets
//!   - CIS 1.15 (iam_api_keys_rotation): inspect createTime (key age)
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration apikeys -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use std::env;
use std::process::Command;
use std::time::Duration;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

const TEST_KEY_DISPLAY_NAME: &str = "cloud-lite-test-ralph-apikeys-key";

/// Create a test API key using gcloud CLI.
/// gcloud prints an operation ID, not the key name — caller should discover
/// the key afterwards via list_keys.
fn gcloud_create_key(project: &str) {
    let output = Command::new("gcloud")
        .args([
            "services",
            "api-keys",
            "create",
            "--display-name",
            TEST_KEY_DISPLAY_NAME,
            "--project",
            project,
            "--quiet",
        ])
        .output();
    if let Ok(out) = output {
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            panic!("gcloud create key failed: {}", err);
        }
    }
}

/// Delete a test API key by key UUID using gcloud CLI.
fn gcloud_delete_key(project: &str, key_id: &str) {
    let _ = Command::new("gcloud")
        .args([
            "services",
            "api-keys",
            "delete",
            key_id,
            "--project",
            project,
            "--quiet",
        ])
        .output();
}

/// Extract the key UUID from a full resource name.
/// e.g. "projects/123456/locations/global/keys/abc-def-uuid" → "abc-def-uuid"
fn key_id_from_name(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

// ── API Keys list + get ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_apikeys_list_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let apikeys = client.apikeys();

    println!("\n=== API Keys Integration Test ===");
    println!("Project: {}", project);

    // Pre-cleanup: remove any leftover test keys from failed prior runs
    let existing = apikeys.list_keys(&project).await?;
    for key in &existing {
        if key.display_name.as_deref() == Some(TEST_KEY_DISPLAY_NAME) {
            let kid = key_id_from_name(&key.name);
            println!("[Setup] Cleaning up leftover test key: {}", key.name);
            gcloud_delete_key(&project, kid);
        }
    }

    // [1/5] Create test key via gcloud CLI
    // Note: gcloud outputs an operation ID on create, not the key name.
    // We discover the created key via list_keys.
    println!("\n[1/5] Creating test API key via gcloud CLI...");
    gcloud_create_key(&project);
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Discover the created key by display_name
    let all_keys = apikeys.list_keys(&project).await?;
    let test_key = all_keys
        .iter()
        .find(|k| k.display_name.as_deref() == Some(TEST_KEY_DISPLAY_NAME))
        .expect(
            "Test key should appear in list_keys after gcloud creation. \
             Ensure apikeys.googleapis.com is enabled.",
        );
    let key_id = key_id_from_name(&test_key.name).to_string();
    println!("  Discovered key: {} (id={})", test_key.name, key_id);

    // Run test body; always clean up regardless of outcome
    let result = run_apikeys_tests(&apikeys, &project, &key_id).await;

    println!("\n[Cleanup] Deleting test key id={}...", key_id);
    gcloud_delete_key(&project, &key_id);
    println!("  Deleted.");

    result?;
    println!("\nAll API Keys tests passed!");
    Ok(())
}

async fn run_apikeys_tests(
    apikeys: &gcp_lite::api::ApikeysClient<'_>,
    project: &str,
    key_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [2/5] list_keys — verify test key appears and check field coverage
    println!("\n[2/5] Listing API keys...");
    let keys = apikeys.list_keys(project).await?;
    println!("  Found {} key(s)", keys.len());
    let found = keys
        .iter()
        .find(|k| key_id_from_name(&k.name) == key_id)
        .expect("Created test key should appear in list_keys");
    println!("  Test key found: {}", found.name);
    assert!(!found.name.is_empty(), "Key name must not be empty");

    // [3/5] get_key — fetch by project + key_id and verify fields
    println!("\n[3/5] Getting key by ID '{}'...", key_id);
    let key = apikeys.get_key(project, key_id).await?;
    assert!(
        key.name.contains(key_id),
        "Key name should contain key_id, got: {}",
        key.name
    );
    assert_eq!(
        key.display_name.as_deref(),
        Some(TEST_KEY_DISPLAY_NAME),
        "display_name mismatch"
    );
    println!("  name:         {}", key.name);
    println!("  display_name: {:?}", key.display_name);
    println!("  uid:          {:?}", key.uid);
    println!("  create_time:  {:?}", key.create_time);
    println!("  restrictions: {:?}", key.restrictions);

    // CIS 1.15: create_time required to compute key age
    assert!(
        key.create_time.is_some(),
        "Key should have create_time (needed for CIS 1.15 key rotation check)"
    );

    // [4/5] Verify resource name contains expected path segments
    println!("\n[4/5] Verifying resource name format...");
    assert!(
        key.name.contains("locations/global/keys"),
        "Key name should contain 'locations/global/keys', got: {}",
        key.name
    );
    assert!(
        key.name.ends_with(key_id),
        "Key name should end with the key_id UUID, got: {}",
        key.name
    );
    println!("  Name format OK: {}", key.name);

    // [5/5] Error case — get a non-existent key
    println!("\n[5/5] Getting non-existent key (expect NotFound error)...");
    let result = apikeys
        .get_key(project, "nonexistent-key-id-00000000")
        .await;
    assert!(result.is_err(), "Should error on non-existent key");
    println!("  Got expected error: {:?}", result.unwrap_err());

    Ok(())
}
