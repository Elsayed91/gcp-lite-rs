//! Integration tests for Cloud DNS API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 3.3 (vpc_dnssec_enabled): patch managed zone DNSSEC config
//!   - CIS 3.4 (vpc_dnssec_no_rsasha1_ksk): inspect DNSSEC key specs
//!   - CIS 3.5 (vpc_dnssec_no_rsasha1_zsk): inspect DNSSEC key specs
//!   - CIS 2.12 (vpc_dns_logging): patch managed zone logging config
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration dns -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::dns::Policy;
use std::env;

const TEST_POLICY: &str = "cloud-lite-test-dns-policy";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── Managed Zones ────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_managed_zones_list() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let dns = client.dns();

    println!("\n=== Managed Zones List ===");
    println!("Project: {}", project);

    // 1. List all managed zones
    println!("\n[1/1] Listing all managed zones...");
    let zones = dns.list_managed_zones(&project).await?;
    println!("  Found {} managed zone(s)", zones.len());
    for z in zones.iter().take(5) {
        println!(
            "    - {} ({})",
            z.name,
            z.dns_name.as_deref().unwrap_or("?")
        );
        println!(
            "      DNSSEC state: {:?}",
            z.dnssec_config.as_ref().and_then(|d| d.state.as_deref())
        );
        println!(
            "      Logging: {:?}",
            z.cloud_logging_config
                .as_ref()
                .and_then(|l| l.enable_logging)
        );
    }
    if zones.len() > 5 {
        println!("  ... and {} more", zones.len() - 5);
    }

    Ok(())
}

// ── DNS Policies ─────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_dns_policies_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let dns = client.dns();

    println!("\n=== DNS Policy Lifecycle ===");
    println!("Project: {}", project);

    // Pre-cleanup: delete any leftover test policy
    let existing = dns.list_dns_policies(&project).await?;
    for p in &existing {
        if p.name == TEST_POLICY {
            let _ = dns.delete_dns_policy(&project, TEST_POLICY).await;
            println!("[Setup] Cleaned up leftover test policy: {}", TEST_POLICY);
        }
    }

    // === 1. LIST (baseline) ===
    println!("\n[1/6] Listing DNS policies (baseline)...");
    let before = dns.list_dns_policies(&project).await?;
    println!("  Found {} existing policy(ies)", before.len());

    // === 2. CREATE ===
    println!("\n[2/6] Creating DNS policy '{}'...", TEST_POLICY);
    let policy = Policy {
        name: TEST_POLICY.to_string(),
        description: Some("cloud-lite integration test policy".to_string()),
        enable_logging: Some(false),
        enable_inbound_forwarding: Some(false),
        ..Default::default()
    };
    let created = dns.create_dns_policy(&project, &policy).await?;
    assert_eq!(created.name, TEST_POLICY, "created policy name mismatch");
    assert_eq!(
        created.enable_logging,
        Some(false),
        "logging should be disabled"
    );
    println!("  Created: {}", created.name);
    println!("  Logging enabled: {:?}", created.enable_logging);

    // === 3. GET ===
    println!("\n[3/6] Getting DNS policy '{}'...", TEST_POLICY);
    let fetched = dns.get_dns_policy(&project, TEST_POLICY).await?;
    assert_eq!(fetched.name, TEST_POLICY, "fetched policy name mismatch");
    assert_eq!(
        fetched.enable_logging,
        Some(false),
        "logging should still be disabled"
    );
    println!(
        "  Got: {} (logging={:?})",
        fetched.name, fetched.enable_logging
    );

    // === 4. LIST (verify inclusion) ===
    println!("\n[4/6] Listing DNS policies (after create)...");
    let after = dns.list_dns_policies(&project).await?;
    assert!(
        after.len() >= before.len() + 1,
        "should have one more policy after create, before={} after={}",
        before.len(),
        after.len()
    );
    let found = after.iter().any(|p| p.name == TEST_POLICY);
    assert!(found, "created policy should appear in list");
    println!("  Found {} policy(ies), test policy present", after.len());

    // === 5. PATCH (enable logging) ===
    println!("\n[5/6] Patching DNS policy to enable logging...");
    let patch = Policy {
        name: TEST_POLICY.to_string(),
        enable_logging: Some(true),
        ..Default::default()
    };
    let patched = dns.patch_dns_policy(&project, TEST_POLICY, &patch).await?;
    assert_eq!(
        patched.enable_logging,
        Some(true),
        "logging should now be enabled"
    );
    println!(
        "  Patched: {} (logging={:?})",
        patched.name, patched.enable_logging
    );

    // === 6. DELETE ===
    println!("\n[6/6] Deleting DNS policy '{}'...", TEST_POLICY);
    dns.delete_dns_policy(&project, TEST_POLICY).await?;
    println!("  Deleted: {}", TEST_POLICY);

    // Verify deletion
    let after_delete = dns.list_dns_policies(&project).await?;
    let still_exists = after_delete.iter().any(|p| p.name == TEST_POLICY);
    assert!(!still_exists, "deleted policy should not appear in list");
    println!("  Confirmed: policy no longer in list");

    Ok(())
}
