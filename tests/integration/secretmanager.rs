//! Integration tests for Secret Manager API.
//!
//! These tests run against the real GCP Secret Manager API.
//! They require valid credentials and GCLOUD_PROJECT_ID environment variable.

use gcp_lite::GcpHttpClient;
use gcp_lite::types::secretmanager::{Automatic, Replication, Secret};
use std::collections::HashMap;

/// Helper to get the test project ID from the environment
fn test_project() -> String {
    std::env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

/// Ensure Secret Manager API is enabled
async fn ensure_api_enabled(client: &GcpHttpClient, project: &str) {
    println!("[Setup] Checking if Secret Manager API is enabled...");
    let service_usage = client.service_usage();
    let service_name = "secretmanager.googleapis.com";

    if service_usage
        .is_service_enabled(project, service_name)
        .await
        .unwrap_or(false)
    {
        println!("[Setup] ✓ Secret Manager API already enabled");
        return;
    }

    println!("[Setup] Enabling Secret Manager API (this may take a minute)...");
    service_usage
        .enable_service(project, service_name)
        .await
        .expect("Failed to enable Secret Manager API");
    println!("[Setup] ✓ Secret Manager API enabled");
}

/// Integration test for Secret CRUD lifecycle
#[tokio::test]
#[ignore]
async fn test_secret_lifecycle() {
    let client = GcpHttpClient::from_adc()
        .await
        .expect("Failed to create client");
    let project = test_project();
    let secret_id = "gcp-lite-test-secret";

    // Ensure API is enabled
    ensure_api_enabled(&client, &project).await;

    let secrets = client.secret_manager();

    // Cleanup: delete secret if it exists from previous failed run
    println!("[Setup] Cleaning up any existing test secret...");
    let _ = secrets.delete_secret(&project, secret_id, None).await;

    println!("[1/5] Creating secret: {}", secret_id);
    let mut labels = HashMap::new();
    labels.insert("test".to_string(), "gcp-lite".to_string());
    labels.insert("env".to_string(), "integration".to_string());

    // Replication is required for secret creation
    let replication = Replication {
        automatic: Some(Automatic::default()),
        ..Default::default()
    };

    let secret_request = Secret {
        labels: labels.clone(),
        replication: Some(replication),
        ..Default::default()
    };

    let created = secrets
        .create_secret(&project, secret_id, &secret_request)
        .await
        .expect("Failed to create secret");
    // GCP may return project number instead of project ID in resource names
    assert!(
        created.name.contains(secret_id),
        "Secret name should contain secret ID"
    );
    assert!(!created.labels.is_empty());
    assert_eq!(
        created.labels.get("test").map(|s| s.as_str()),
        Some("gcp-lite")
    );

    println!("[2/5] Getting secret: {}", secret_id);
    let fetched = secrets
        .get_secret(&project, secret_id)
        .await
        .expect("Failed to get secret");
    assert_eq!(fetched.name, created.name);
    assert_eq!(fetched.labels, created.labels);

    println!("[3/5] Listing secrets");
    let list_response = secrets
        .list_secrets(&project, None, None, None)
        .await
        .expect("Failed to list secrets");
    assert!(list_response.secrets.iter().any(|s| s.name == created.name));

    println!("[4/5] Updating secret labels");
    labels.insert("env".to_string(), "updated".to_string());
    let update_request = Secret {
        labels,
        ..Default::default()
    };
    let updated = secrets
        .patch_secret(&project, secret_id, "labels", &update_request)
        .await
        .expect("Failed to update secret");
    assert!(!updated.labels.is_empty());
    assert_eq!(
        updated.labels.get("env").map(|s| s.as_str()),
        Some("updated")
    );

    println!("[5/8] Deleting secret: {}", secret_id);
    secrets
        .delete_secret(&project, secret_id, None)
        .await
        .expect("Failed to delete secret");

    // Verify deletion
    let result = secrets.get_secret(&project, secret_id).await;
    assert!(result.is_err(), "Secret should not exist after deletion");

    // === ERROR CASES ===

    println!("[6/8] Getting non-existent secret (expect error)...");
    let result = secrets
        .get_secret(&project, "nonexistent-secret-xyz-99999")
        .await;
    assert!(result.is_err(), "Non-existent secret should return error");
    println!("  Non-existent secret: error (correct)");

    println!("[7/8] Deleting already-deleted secret (expect error)...");
    let result = secrets.delete_secret(&project, secret_id, None).await;
    assert!(
        result.is_err(),
        "Deleting already-deleted secret should error"
    );
    println!("  Already-deleted secret: error (correct)");

    println!("[8/8] Listing secrets with filter...");
    let filtered = secrets
        .list_secrets(&project, Some("labels.test=gcp-lite"), None, None)
        .await
        .expect("Filtered list should succeed");
    // After deletion, should not find any matching secrets with this label
    assert!(
        !filtered.secrets.iter().any(|s| s.name.contains(secret_id)),
        "Deleted secret should not appear in filtered list"
    );
    println!(
        "  Filtered list: {} secrets (deleted not present)",
        filtered.secrets.len()
    );

    println!("Secret lifecycle test completed successfully");
}
