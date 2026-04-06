//! Integration tests for Cloud KMS API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 1.9 (iam_kms_public_access): GetIamPolicy, SetIamPolicy on CryptoKey
//!   - CIS 1.10 (iam_kms_key_rotation): ListCryptoKeys, GetCryptoKey, UpdateCryptoKey rotation
//!   - CIS 1.11 (iam_kms_separation_of_duties): GetIamPolicy on KeyRing
//!
//! Prerequisites (created once, not cleaned up since KMS resources are permanent):
//!   Key ring: cloud-lite-test-ralph-kms-ring (location: global)
//!   Crypto key: cloud-lite-test-ralph-kms-key
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration cloudkms -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use std::env;
use std::process::Command;

const TEST_LOCATION: &str = "global";
const TEST_KEY_RING: &str = "cloud-lite-test-ralph-kms-ring";
const TEST_CRYPTO_KEY: &str = "cloud-lite-test-ralph-kms-key";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

/// Ensure the KMS key ring exists (idempotent — ignores ALREADY_EXISTS errors).
fn gcloud_ensure_key_ring(project: &str, location: &str, ring_name: &str) {
    let _ = Command::new("gcloud")
        .args([
            "kms",
            "keyrings",
            "create",
            ring_name,
            "--location",
            location,
            "--project",
            project,
        ])
        .output();
}

/// Ensure the KMS crypto key exists (idempotent — ignores ALREADY_EXISTS errors).
fn gcloud_ensure_crypto_key(project: &str, location: &str, ring: &str, key: &str) {
    let _ = Command::new("gcloud")
        .args([
            "kms",
            "keys",
            "create",
            key,
            "--keyring",
            ring,
            "--location",
            location,
            "--purpose",
            "encryption",
            "--project",
            project,
        ])
        .output();
}

// =============================================================================
// Integration Tests
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_cloudkms_list_and_get() {
    let project = project_id();

    println!("\n=== Cloud KMS List/Get Test ===");
    println!("Project: {}", project);
    println!("Location: {}", TEST_LOCATION);

    // Ensure fixtures exist
    println!("[Setup] Ensuring key ring and crypto key exist...");
    gcloud_ensure_key_ring(&project, TEST_LOCATION, TEST_KEY_RING);
    gcloud_ensure_crypto_key(&project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let kms = client.kms();

    let result = run_list_get_tests(&kms, &project).await;
    result.expect("Cloud KMS list/get tests failed");

    println!("\nAll Cloud KMS list/get tests passed!");
}

async fn run_list_get_tests(
    kms: &gcp_lite::api::KmsClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // === 1. LIST LOCATIONS ===
    println!("\n[1/6] Listing KMS locations...");
    let locations = kms.list_locations(project).await?;
    assert!(
        !locations.is_empty(),
        "Should have at least one KMS location"
    );
    let global_loc = locations
        .iter()
        .find(|l| l.location_id == "global")
        .expect("Should find 'global' location");
    assert!(
        global_loc.name.contains("global"),
        "Location name should contain 'global', got: {}",
        global_loc.name
    );
    println!("  Found {} locations, including 'global'", locations.len());

    // === 2. LIST KEY RINGS ===
    println!("\n[2/6] Listing key rings in 'global'...");
    let rings = kms.list_key_rings(project, TEST_LOCATION).await?;
    assert!(
        !rings.is_empty(),
        "Should have at least one key ring in global"
    );
    let test_ring = rings
        .iter()
        .find(|r| r.name.contains(TEST_KEY_RING))
        .expect("Should find test key ring");
    assert!(
        test_ring.name.contains(TEST_KEY_RING),
        "Key ring name should contain '{}', got: {}",
        TEST_KEY_RING,
        test_ring.name
    );
    println!("  Found {} key rings, including test ring", rings.len());

    // === 3. GET KEY RING ===
    println!("\n[3/6] Getting key ring '{}'...", TEST_KEY_RING);
    let ring = kms
        .get_key_ring(project, TEST_LOCATION, TEST_KEY_RING)
        .await?;
    assert!(
        ring.name.contains(TEST_KEY_RING),
        "Key ring name mismatch: {}",
        ring.name
    );
    assert!(
        ring.create_time.is_some(),
        "Key ring should have createTime"
    );
    println!("  Name: {}", ring.name);
    println!("  Created: {}", ring.create_time.as_deref().unwrap_or("?"));

    // === 4. LIST CRYPTO KEYS ===
    println!("\n[4/6] Listing crypto keys in '{}'...", TEST_KEY_RING);
    let keys = kms
        .list_crypto_keys(project, TEST_LOCATION, TEST_KEY_RING)
        .await?;
    assert!(
        !keys.is_empty(),
        "Should have at least one crypto key in the test key ring"
    );
    let test_key = keys
        .iter()
        .find(|k| k.name.contains(TEST_CRYPTO_KEY))
        .expect("Should find test crypto key");
    assert!(
        test_key.name.contains(TEST_CRYPTO_KEY),
        "Crypto key name mismatch: {}",
        test_key.name
    );
    println!("  Found {} crypto keys, including test key", keys.len());

    // === 5. GET CRYPTO KEY ===
    println!("\n[5/6] Getting crypto key '{}'...", TEST_CRYPTO_KEY);
    let key = kms
        .get_crypto_key(project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY)
        .await?;
    assert!(
        key.name.contains(TEST_CRYPTO_KEY),
        "Crypto key name mismatch: {}",
        key.name
    );
    assert_eq!(
        key.purpose.as_deref(),
        Some("ENCRYPT_DECRYPT"),
        "Key purpose should be ENCRYPT_DECRYPT, got: {:?}",
        key.purpose
    );
    assert!(key.primary.is_some(), "Key should have a primary version");
    println!("  Name: {}", key.name);
    println!("  Purpose: {:?}", key.purpose);
    println!(
        "  Primary state: {:?}",
        key.primary.as_ref().and_then(|p| p.state.as_deref())
    );

    // === 6. ERROR CASE: Get non-existent key ring ===
    println!("\n[6/6] Getting non-existent key ring (expect error)...");
    let err_result = kms
        .get_key_ring(project, TEST_LOCATION, "nonexistent-ring-xyz-99999")
        .await;
    assert!(err_result.is_err(), "Should error on non-existent key ring");
    println!("  Correctly received error: {:?}", err_result.unwrap_err());

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_cloudkms_iam_policy() {
    let project = project_id();

    println!("\n=== Cloud KMS IAM Policy Test ===");
    println!("Project: {}", project);

    // Ensure fixtures exist
    gcloud_ensure_key_ring(&project, TEST_LOCATION, TEST_KEY_RING);
    gcloud_ensure_crypto_key(&project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let kms = client.kms();

    let result = run_iam_tests(&kms, &project).await;
    result.expect("Cloud KMS IAM tests failed");

    println!("\nAll Cloud KMS IAM tests passed!");
}

async fn run_iam_tests(
    kms: &gcp_lite::api::KmsClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // === 1. GET IAM POLICY ===
    println!("\n[1/2] Getting IAM policy for crypto key...");
    let policy = kms
        .get_crypto_key_iam_policy(project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY)
        .await?;

    // Policy should exist (may be empty if no explicit bindings set)
    // etag is always present
    assert!(
        policy.etag.is_some(),
        "Policy should have an etag, got: {:?}",
        policy
    );
    let original_etag = policy.etag.clone();
    println!("  Policy etag: {:?}", original_etag);
    println!("  Bindings: {} bindings", policy.bindings.len());

    // === 2. SET IAM POLICY (no-op round-trip) ===
    println!("\n[2/2] Setting IAM policy (no-op round-trip)...");
    // Re-set the exact same policy — should succeed and return updated policy
    let updated_policy = kms
        .set_crypto_key_iam_policy(
            project,
            TEST_LOCATION,
            TEST_KEY_RING,
            TEST_CRYPTO_KEY,
            policy,
        )
        .await?;
    assert!(
        updated_policy.etag.is_some(),
        "Updated policy should have an etag"
    );
    println!("  Set IAM policy succeeded");
    println!("  New etag: {:?}", updated_policy.etag);

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_cloudkms_update_rotation() {
    let project = project_id();

    println!("\n=== Cloud KMS Rotation Update Test ===");
    println!("Project: {}", project);

    // Ensure fixtures exist
    gcloud_ensure_key_ring(&project, TEST_LOCATION, TEST_KEY_RING);
    gcloud_ensure_crypto_key(&project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let kms = client.kms();

    let result = run_rotation_tests(&kms, &project).await;
    result.expect("Cloud KMS rotation tests failed");

    println!("\nAll Cloud KMS rotation tests passed!");
}

// ── Key Ring IAM ─────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_cloudkms_key_ring_iam_policy() {
    let project = project_id();

    println!("\n=== Cloud KMS Key Ring IAM Policy Test ===");
    println!("Project: {}", project);

    // Ensure the key ring fixture exists
    gcloud_ensure_key_ring(&project, TEST_LOCATION, TEST_KEY_RING);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let kms = client.kms();

    let result = run_key_ring_iam_tests(&kms, &project).await;
    result.expect("Cloud KMS key ring IAM tests failed");

    println!("\nAll Cloud KMS key ring IAM tests passed!");
}

async fn run_key_ring_iam_tests(
    kms: &gcp_lite::api::KmsClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // === 1. GET KEY RING IAM POLICY ===
    println!(
        "\n[1/1] Getting IAM policy for key ring '{}'...",
        TEST_KEY_RING
    );
    let policy = kms
        .get_key_ring_iam_policy(project, TEST_LOCATION, TEST_KEY_RING)
        .await?;

    // IAM policy always has an etag, even when empty
    assert!(
        policy.etag.is_some(),
        "Key ring policy should have an etag, got: {:?}",
        policy
    );
    println!("  Policy etag: {:?}", policy.etag);
    println!("  Bindings: {} binding(s)", policy.bindings.len());
    for b in &policy.bindings {
        println!("    role={} members={:?}", b.role, b.members);
    }

    // CIS 1.11 check: no principal should have both cloudkms.admin AND
    // cloudkms.cryptoKeyEncrypterDecrypter on the same key ring.
    let admin_members: std::collections::HashSet<&str> = policy
        .bindings
        .iter()
        .filter(|b| b.role == "roles/cloudkms.admin")
        .flat_map(|b| b.members.iter().map(String::as_str))
        .collect();

    let encrypter_members: std::collections::HashSet<&str> = policy
        .bindings
        .iter()
        .filter(|b| b.role == "roles/cloudkms.cryptoKeyEncrypterDecrypter")
        .flat_map(|b| b.members.iter().map(String::as_str))
        .collect();

    let overlap: Vec<_> = admin_members.intersection(&encrypter_members).collect();
    if !overlap.is_empty() {
        println!(
            "  WARNING (CIS 1.11 violation): principals with both admin + encrypter on key ring: {:?}",
            overlap
        );
    } else {
        println!("  CIS 1.11: no separation-of-duties violation on this key ring.");
    }

    Ok(())
}

async fn run_rotation_tests(
    kms: &gcp_lite::api::KmsClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // === 1. GET CURRENT STATE ===
    println!("\n[1/3] Getting current crypto key state...");
    let key_before = kms
        .get_crypto_key(project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY)
        .await?;
    println!("  Current rotationPeriod: {:?}", key_before.rotation_period);
    println!(
        "  Current nextRotationTime: {:?}",
        key_before.next_rotation_time
    );

    // === 2. SET ROTATION SCHEDULE ===
    // 90 days = 7776000 seconds
    let rotation_period = "7776000s";
    // Set next rotation to 30 days from now (approximately)
    let next_rotation_time = "2026-03-30T00:00:00Z";

    println!("\n[2/3] Setting rotation schedule...");
    println!("  rotationPeriod: {}", rotation_period);
    println!("  nextRotationTime: {}", next_rotation_time);

    let updated_key = kms
        .update_crypto_key_rotation(
            project,
            TEST_LOCATION,
            TEST_KEY_RING,
            TEST_CRYPTO_KEY,
            rotation_period,
            next_rotation_time,
        )
        .await?;

    assert_eq!(
        updated_key.rotation_period.as_deref(),
        Some(rotation_period),
        "rotationPeriod should be set to {}, got: {:?}",
        rotation_period,
        updated_key.rotation_period
    );
    assert!(
        updated_key.next_rotation_time.is_some(),
        "nextRotationTime should be set, got: {:?}",
        updated_key.next_rotation_time
    );
    println!("  Rotation schedule set successfully");
    println!("  rotationPeriod: {:?}", updated_key.rotation_period);
    println!("  nextRotationTime: {:?}", updated_key.next_rotation_time);

    // === 3. VERIFY WITH GET ===
    println!("\n[3/3] Verifying rotation via get_crypto_key...");
    let verified_key = kms
        .get_crypto_key(project, TEST_LOCATION, TEST_KEY_RING, TEST_CRYPTO_KEY)
        .await?;
    assert_eq!(
        verified_key.rotation_period.as_deref(),
        Some(rotation_period),
        "Verified rotationPeriod should be {}, got: {:?}",
        rotation_period,
        verified_key.rotation_period
    );
    println!(
        "  Verified rotationPeriod: {:?}",
        verified_key.rotation_period
    );

    Ok(())
}
