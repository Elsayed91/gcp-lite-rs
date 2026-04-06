//! Integration tests for Cloud Storage bucket and object operations.
//!
//! These tests require:
//! - `GCLOUD_PROJECT_ID` environment variable
//! - ADC credentials configured (`GOOGLE_AUTH_USE_GCLOUD=1` or service account)
//! - Sufficient permissions (`roles/storage.admin`)
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration storage -- --ignored --test-threads=1 --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::storage::*;
use std::env;
use std::process::Command;

const TEST_BUCKET_PREFIX: &str = "gcp-http-lite-integ-";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

fn test_bucket_name(suffix: &str) -> String {
    let project = project_id();
    format!("{TEST_BUCKET_PREFIX}{suffix}-{project}")
}

fn gcloud_delete_bucket(bucket: &str) {
    let _ = Command::new("gcloud")
        .args(["storage", "rm", "-r", &format!("gs://{bucket}"), "--quiet"])
        .output();
}

async fn wait_for_consistency() {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}

// =============================================================================
// Bucket CRUD Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_bucket_lifecycle() {
    let project = project_id();
    let bucket = test_bucket_name("bkt");

    gcloud_delete_bucket(&bucket);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let result = run_bucket_tests(&client, &project, &bucket).await;

    println!("\nCleaning up...");
    gcloud_delete_bucket(&bucket);

    result.expect("bucket lifecycle test failed");
    println!("\nAll bucket lifecycle tests passed!");
}

async fn run_bucket_tests(
    client: &GcpHttpClient,
    project: &str,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let storage = client.storage();

    // === 1. CREATE ===
    println!("\n[1/6] Creating bucket...");
    let created = storage
        .create_bucket(
            project,
            &Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                storage_class: Some("STANDARD".to_string()),
                ..Default::default()
            },
        )
        .await?;
    assert_eq!(created.name, bucket);
    assert_eq!(created.location.as_deref(), Some("US"));
    println!("  Created: {bucket}");

    wait_for_consistency().await;

    // === 2. GET ===
    println!("\n[2/6] Getting bucket...");
    let got = storage.get_bucket(bucket).await?;
    assert_eq!(got.name, bucket);
    assert_eq!(got.location.as_deref(), Some("US"));
    println!("  All fields match");

    // === 3. LIST with prefix ===
    println!("\n[3/7] Listing buckets with prefix...");
    let list = storage
        .list_buckets(project, Some(TEST_BUCKET_PREFIX), None)
        .await?;
    let found = list.items.iter().any(|b| b.name == bucket);
    assert!(found, "Created bucket should appear in list");
    println!("  Found bucket in list ({} total)", list.items.len());

    // List with None prefix (no filter)
    println!("  Listing buckets with no prefix (None)...");
    let list_all = storage.list_buckets(project, None, None).await?;
    let found_all = list_all.items.iter().any(|b| b.name == bucket);
    assert!(found_all, "Bucket should appear in unfiltered list");
    println!(
        "  Found bucket in unfiltered list ({} total)",
        list_all.items.len()
    );

    // === 4. PATCH ===
    println!("\n[4/6] Patching bucket storage class...");
    let patched = storage
        .patch_bucket(
            bucket,
            &Bucket {
                storage_class: Some("NEARLINE".to_string()),
                ..Default::default()
            },
        )
        .await?;
    assert_eq!(patched.storage_class.as_deref(), Some("NEARLINE"));
    println!("  Storage class updated to NEARLINE");

    // === 5. IAM ===
    println!("\n[5/6] Getting bucket IAM policy...");
    let policy = storage.get_bucket_iam_policy(bucket).await?;
    assert!(policy.version.is_some());
    println!("  IAM policy version: {:?}", policy.version);

    // === 6. DELETE ===
    println!("\n[6/7] Deleting bucket...");
    storage.delete_bucket(bucket).await?;
    println!("  Deleted");

    wait_for_consistency().await;

    let result = storage.get_bucket(bucket).await;
    assert!(result.is_err(), "Deleted bucket should return error");
    println!("  Confirmed: get returns error after deletion");

    // === 7. ERROR CASES ===
    println!("\n[7/7] Testing error cases...");

    // Get non-existent bucket
    let result = storage.get_bucket("nonexistent-bucket-xyz-99999").await;
    assert!(result.is_err(), "Non-existent bucket should return error");
    println!("  Non-existent bucket get: error (correct)");

    Ok(())
}

// =============================================================================
// Object CRUD Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_object_lifecycle() {
    let project = project_id();
    let bucket = test_bucket_name("obj");

    gcloud_delete_bucket(&bucket);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let storage = client.storage();
    storage
        .create_bucket(
            &project,
            &Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create test bucket");

    wait_for_consistency().await;

    let result = run_object_tests(&client, &bucket).await;

    println!("\nCleaning up...");
    gcloud_delete_bucket(&bucket);

    result.expect("object lifecycle test failed");
    println!("\nAll object lifecycle tests passed!");
}

async fn run_object_tests(
    client: &GcpHttpClient,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let storage = client.storage();

    // === 1. CREATE ===
    println!("\n[1/5] Creating object...");
    let created = storage
        .create_object(
            bucket,
            &Object {
                name: "test-data/hello.txt".to_string(),
                content_type: Some("text/plain".to_string()),
                ..Default::default()
            },
        )
        .await?;
    assert_eq!(created.name, "test-data/hello.txt");
    println!("  Created: test-data/hello.txt");

    // === 2. GET ===
    println!("\n[2/5] Getting object metadata...");
    let got = storage.get_object(bucket, "test-data/hello.txt").await?;
    assert_eq!(got.name, "test-data/hello.txt");
    assert_eq!(got.bucket.as_deref(), Some(bucket));
    println!("  Object metadata retrieved");

    // === 3. LIST ===
    println!("\n[3/5] Listing objects...");
    let list = storage
        .list_objects(bucket, Some("test-data/"), None)
        .await?;
    let found = list.items.iter().any(|o| o.name == "test-data/hello.txt");
    assert!(found, "Created object should appear in list");
    println!("  Found object in list ({} total)", list.items.len());

    // === 4. COPY ===
    println!("\n[4/5] Copying object...");
    let copied = storage
        .copy_object(
            bucket,
            "test-data/hello.txt",
            bucket,
            "test-data/hello-copy.txt",
            &Object::default(),
        )
        .await?;
    assert_eq!(copied.name, "test-data/hello-copy.txt");
    println!("  Copied to test-data/hello-copy.txt");

    // === 5. LIST with stream ===
    println!("\n[5/7] Streaming objects with prefix...");
    use futures::StreamExt;

    // Create a few more objects for stream testing
    storage
        .create_object(
            bucket,
            &Object {
                name: "test-data/stream-a.txt".to_string(),
                content_type: Some("text/plain".to_string()),
                ..Default::default()
            },
        )
        .await?;
    storage
        .create_object(
            bucket,
            &Object {
                name: "test-data/stream-b.txt".to_string(),
                content_type: Some("text/plain".to_string()),
                ..Default::default()
            },
        )
        .await?;

    let stream = storage.list_objects_stream(bucket, Some("test-data/"));
    futures::pin_mut!(stream);

    let mut stream_names: Vec<String> = Vec::new();
    while let Some(result) = stream.next().await {
        let obj = result?;
        stream_names.push(obj.name.clone());
    }
    println!("  Streamed {} objects", stream_names.len());
    // Should include original + copy + 2 stream objects
    assert!(
        stream_names.len() >= 4,
        "Expected at least 4 objects via stream, got {}",
        stream_names.len()
    );
    assert!(stream_names.contains(&"test-data/hello.txt".to_string()));
    assert!(stream_names.contains(&"test-data/hello-copy.txt".to_string()));
    assert!(stream_names.contains(&"test-data/stream-a.txt".to_string()));
    assert!(stream_names.contains(&"test-data/stream-b.txt".to_string()));
    println!("  All expected objects found in stream");

    // === 6. REWRITE (same-bucket rename) ===
    println!("\n[6/7] Rewriting object (same-bucket rename)...");
    let rewrite = storage
        .rewrite_object(
            bucket,
            "test-data/stream-a.txt",
            bucket,
            "test-data/renamed-a.txt",
            None,
            &Object::default(),
        )
        .await?;
    assert_eq!(rewrite.done, Some(true));
    let rewritten = storage
        .get_object(bucket, "test-data/renamed-a.txt")
        .await?;
    assert_eq!(rewritten.name, "test-data/renamed-a.txt");
    println!("  Rewritten: stream-a.txt -> renamed-a.txt");

    // === 7. DELETE all ===
    println!("\n[7/7] Deleting all objects...");
    for name in [
        "test-data/hello.txt",
        "test-data/hello-copy.txt",
        "test-data/stream-b.txt",
        "test-data/renamed-a.txt",
    ] {
        storage.delete_object(bucket, name).await?;
    }
    println!("  Deleted all objects");

    let result = storage.get_object(bucket, "test-data/hello.txt").await;
    assert!(result.is_err(), "Deleted object should return error");
    println!("  Confirmed: get returns error after deletion");

    // Error: get non-existent object
    println!("  Getting non-existent object...");
    let result = storage
        .get_object(bucket, "nonexistent/path/to/object.txt")
        .await;
    assert!(result.is_err(), "Non-existent object should return error");
    println!("  Non-existent object get: error (correct)");

    // List with None prefix (no filter — should return empty after deletions)
    println!("  Listing objects with no prefix (None)...");
    let list_none = storage.list_objects(bucket, None, None).await?;
    println!("  Objects with no prefix filter: {}", list_none.items.len());

    Ok(())
}

// =============================================================================
// Bucket Stream
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_bucket_stream() {
    let project = project_id();
    let bucket = test_bucket_name("strm");

    gcloud_delete_bucket(&bucket);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let storage = client.storage();
    storage
        .create_bucket(
            &project,
            &Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create test bucket");

    wait_for_consistency().await;

    let result = run_bucket_stream_tests(&client, &project, &bucket).await;

    println!("\nCleaning up...");
    gcloud_delete_bucket(&bucket);

    result.expect("bucket stream test failed");
    println!("\nAll bucket stream tests passed!");
}

async fn run_bucket_stream_tests(
    client: &GcpHttpClient,
    project: &str,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;

    let storage = client.storage();

    println!("\n[1/1] Streaming buckets with prefix...");
    let stream = storage.list_buckets_stream(project, Some(TEST_BUCKET_PREFIX));
    futures::pin_mut!(stream);

    let mut found = false;
    let mut count = 0;
    while let Some(result) = stream.next().await {
        let b = result?;
        count += 1;
        if b.name == bucket {
            found = true;
        }
    }
    println!("  Streamed {} buckets with prefix", count);
    assert!(found, "Test bucket should appear in stream");
    println!("  Confirmed: test bucket found in stream");

    Ok(())
}

// =============================================================================
// Public Access Prevention
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_public_access_prevention() {
    let project = project_id();
    let bucket = test_bucket_name("pap");

    gcloud_delete_bucket(&bucket);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    let storage = client.storage();
    storage
        .create_bucket(
            &project,
            &Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create test bucket");

    wait_for_consistency().await;

    let result = run_pap_tests(&client, &bucket).await;

    println!("\nCleaning up...");
    gcloud_delete_bucket(&bucket);

    result.expect("public access prevention test failed");
    println!("\nAll PAP tests passed!");
}

async fn run_pap_tests(
    client: &GcpHttpClient,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let storage = client.storage();

    // === 1. Basic PAP toggle: enforced ===
    println!("\n[1/4] Setting public access prevention to enforced...");
    let enforced = storage.set_public_access_prevention(bucket, true).await?;
    assert_eq!(
        enforced
            .iam_configuration
            .as_ref()
            .and_then(|c| c.public_access_prevention.as_deref()),
        Some("enforced")
    );
    println!("  PAP set to enforced");

    wait_for_consistency().await;

    // === 2. Basic PAP toggle: inherited ===
    println!("\n[2/4] Setting public access prevention to inherited...");
    let inherited = storage.set_public_access_prevention(bucket, false).await?;
    assert_eq!(
        inherited
            .iam_configuration
            .as_ref()
            .and_then(|c| c.public_access_prevention.as_deref()),
        Some("inherited")
    );
    println!("  PAP set to inherited");

    wait_for_consistency().await;

    // === 3. Verify PAP doesn't overwrite other bucket properties (race condition fix) ===
    // This is the critical test: set a bucket property (storage_class), then
    // call set_public_access_prevention, then verify storage_class wasn't reverted.
    // Before the fix, the method did GET→modify→PATCH with the full bucket,
    // which would overwrite concurrent changes. Now it sends a minimal PATCH.
    println!("\n[3/4] Verifying PAP doesn't overwrite other bucket fields...");

    // First, set storage_class to NEARLINE
    let patched = storage
        .patch_bucket(
            bucket,
            &Bucket {
                storage_class: Some("NEARLINE".to_string()),
                ..Default::default()
            },
        )
        .await?;
    assert_eq!(patched.storage_class.as_deref(), Some("NEARLINE"));
    println!("  Set storage_class to NEARLINE");

    wait_for_consistency().await;

    // Now set PAP to enforced — this should NOT affect storage_class
    let after_pap = storage.set_public_access_prevention(bucket, true).await?;
    assert_eq!(
        after_pap
            .iam_configuration
            .as_ref()
            .and_then(|c| c.public_access_prevention.as_deref()),
        Some("enforced")
    );

    // Verify the storage class survived the PAP change
    let verify = storage.get_bucket(bucket).await?;
    assert_eq!(
        verify.storage_class.as_deref(),
        Some("NEARLINE"),
        "storage_class should still be NEARLINE after set_public_access_prevention"
    );
    println!("  Confirmed: storage_class preserved after PAP change");

    wait_for_consistency().await;

    // === 4. Reset PAP back to inherited ===
    println!("\n[4/4] Resetting PAP to inherited...");
    storage.set_public_access_prevention(bucket, false).await?;
    println!("  PAP reset to inherited");

    Ok(())
}

// =============================================================================
// Bucket Retention Lock (CIS 2.3)
// NOTE: locking is permanent but the bucket itself can still be deleted when empty.
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_lock_bucket_retention_policy() {
    let project = project_id();
    let bucket = test_bucket_name("retention-lock");

    // Pre-cleanup
    gcloud_delete_bucket(&bucket);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let storage = client.storage();

    let result = run_retention_lock_test(&storage, &project, &bucket).await;

    println!("\nCleaning up retention lock bucket...");
    gcloud_delete_bucket(&bucket);

    result.expect("retention lock test failed");
}

async fn run_retention_lock_test(
    storage: &gcp_lite::api::storage::StorageClient<'_>,
    project: &str,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Bucket Retention Lock ===");
    println!("Project: {}", project);
    println!("Bucket:  {}", bucket);

    // === 1. CREATE BUCKET WITH RETENTION POLICY ===
    // Use a 1-second retention period so we aren't blocked by actual data retention.
    // The lock itself is what we're testing, not the retention enforcement.
    println!("\n[1/4] Creating bucket with 1-second retention policy...");
    let body = Bucket {
        name: bucket.to_string(),
        location: Some("US".to_string()),
        storage_class: Some("STANDARD".to_string()),
        retention_policy: Some(serde_json::json!({
            "retentionPeriod": "1"
        })),
        ..Default::default()
    };
    let created = storage.create_bucket(project, &body).await?;
    assert_eq!(created.name, bucket);
    println!("  Bucket created with retention_policy set");

    wait_for_consistency().await;

    // === 2. READ METAGENERATION ===
    // lockRetentionPolicy requires the current metageneration for optimistic concurrency.
    println!("\n[2/4] Reading bucket metageneration...");
    let current = storage.get_bucket(bucket).await?;
    let metageneration = current
        .metageneration
        .as_deref()
        .ok_or("metageneration missing from bucket response")?;
    println!("  metageneration = {}", metageneration);

    // Verify the retention policy is present but not yet locked
    let retention = current
        .retention_policy
        .as_ref()
        .ok_or("retentionPolicy missing from bucket response")?;
    let is_locked_before = retention
        .get("isLocked")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(
        !is_locked_before,
        "retention policy should NOT be locked before locking"
    );
    println!("  Confirmed: retention policy exists and is NOT yet locked");

    // === 3. LOCK RETENTION POLICY ===
    println!("\n[3/4] Locking retention policy...");
    let locked = storage
        .lock_bucket_retention_policy(bucket, metageneration)
        .await?;
    assert_eq!(locked.name, bucket);

    // Verify the lock is active
    let locked_retention = locked
        .retention_policy
        .as_ref()
        .ok_or("retentionPolicy missing after lock")?;
    let is_locked_after = locked_retention
        .get("isLocked")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(
        is_locked_after,
        "retention policy should be locked after lockRetentionPolicy"
    );
    println!("  Retention policy is now LOCKED (isLocked=true)");

    // === 4. VERIFY LOCK PERSISTS ===
    println!("\n[4/4] Verifying lock persists via get_bucket...");
    let verified = storage.get_bucket(bucket).await?;
    let verified_retention = verified
        .retention_policy
        .as_ref()
        .ok_or("retentionPolicy missing after verification get")?;
    let still_locked = verified_retention
        .get("isLocked")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(still_locked, "retention lock should persist");
    println!("  Lock confirmed: isLocked=true");

    Ok(())
}
