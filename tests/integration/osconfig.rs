//! Integration tests for OS Config API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 4.12 (compute_os_patches): verify OS patching is configured and
//!     VMs are reporting inventory (OS Config agent is installed and running).
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration osconfig -- --ignored --nocapture
//! ```

use gcp_lite::{GcpError, GcpHttpClient};
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── OS Config patch deployments + inventory ───────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_osconfig_list_patch_deployments_and_inventories()
-> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let os = client.osconfig();

    println!("\n=== OS Config Integration Test ===");
    println!("Project: {}", project);

    // [1/2] List patch deployments
    println!("\n[1/2] Listing patch deployments...");
    let deployments = os.list_patch_deployments(&project).await?;
    println!("  Found {} patch deployment(s)", deployments.len());
    for pd in &deployments {
        println!(
            "  - {} (state: {:?}, lastExecute: {:?})",
            pd.name, pd.state, pd.last_execute_time
        );
    }
    // CIS 4.12: projects may have zero patch deployments (non-compliant).
    // Verify structure of any returned deployments.
    for pd in &deployments {
        assert!(
            pd.name.contains(&project),
            "patch deployment name should contain project ID"
        );
    }

    // [2/2] List inventories (project-wide via zone wildcard, basic view)
    // NOTE: The OS Config API returns 501 for cross-location wildcard queries
    // (locations/-/instances/-) on projects where OS Config is not fully configured
    // or no VMs have the OS Config agent installed. This is a known API limitation.
    // For production CIS 4.12 checks, use list_inventories_in_zone per zone.
    println!("\n[2/2] Listing VM inventories (project-wide, BASIC view)...");
    match os.list_inventories(&project, "BASIC").await {
        Ok(inventories) => {
            println!("  Found {} inventory record(s)", inventories.len());
            for inv in inventories.iter().take(3) {
                let os_name = inv
                    .os_info
                    .as_ref()
                    .and_then(|o| o.long_name.as_deref())
                    .unwrap_or("<unknown>");
                println!(
                    "  - {} (OS: {}, updated: {:?})",
                    inv.name, os_name, inv.update_time
                );
            }
            for inv in &inventories {
                assert!(
                    inv.name.contains(&project),
                    "inventory name should contain project ID"
                );
            }
        }
        Err(GcpError::ServerError { status: 501, .. }) => {
            println!("  Project-wide inventory listing not supported (501 NotImplemented).");
            println!("  This occurs when OS Config agent is not installed on any VM.");
            println!("  For CIS 4.12, use list_inventories_in_zone per zone instead.");
        }
        Err(e) => return Err(e.into()),
    }

    println!("\nAll OS Config tests passed!");
    Ok(())
}
