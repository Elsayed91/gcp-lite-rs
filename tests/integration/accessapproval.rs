//! Integration tests for Access Approval API.
//!
//! Needed for GCP CIS benchmark check:
//!   - CIS 2.15 (logging_access_approval): verify Access Approval is enrolled
//!     for at least one service on the org/project.
//!
//! **Important API behavior**: `GetAccessApprovalSettings` returns 404 (NotFound)
//! when Access Approval has never been configured on the resource — NOT an empty
//! settings object. CIS check consumers must treat NotFound as "not enrolled".
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration accessapproval -- --ignored --nocapture
//! ```

use gcp_lite::{GcpError, GcpHttpClient};
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── Access Approval Settings ──────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_access_approval_get_project_settings() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let aa = client.access_approval();

    println!("\n=== Access Approval Integration Test ===");
    println!("Project: {}", project);

    // [1/3] Get project settings.
    // The API returns 404 (NotFound) when Access Approval has never been configured
    // on this project — it does NOT return an empty settings object.
    // Both outcomes are valid for this test:
    //   - Ok(settings)  → Access Approval is configured; verify field structure
    //   - Err(NotFound) → Not configured; this is the normal state for most projects
    println!("\n[1/3] Getting Access Approval settings for project...");
    match aa.get_project_settings(&project).await {
        Ok(settings) => {
            println!("  Access Approval IS configured on this project.");
            println!("  name:              {}", settings.name);
            println!("  enrolled_services: {:?}", settings.enrolled_services);
            println!("  enrolled_ancestor: {:?}", settings.enrolled_ancestor);
            println!("  notification_emails: {:?}", settings.notification_emails);

            // [2/3] Verify field structure when settings are present
            println!("\n[2/3] Verifying settings structure...");
            assert!(
                settings.name.contains("accessApprovalSettings"),
                "name should contain 'accessApprovalSettings', got: {}",
                settings.name
            );
            for svc in &settings.enrolled_services {
                println!(
                    "    enrolled: {:?} at level {:?}",
                    svc.cloud_product, svc.enrollment_level
                );
            }
            println!(
                "  Structure OK. Enrolled service count: {}",
                settings.enrolled_services.len()
            );
        }
        Err(GcpError::NotFound { .. }) => {
            // This is the expected outcome for projects where Access Approval
            // has never been set up. CIS check should interpret this as non-compliant.
            println!(
                "  Access Approval is NOT configured (NotFound). \
                 This is normal — treat as non-enrolled for CIS 2.15."
            );
            println!("\n[2/3] Skipping structure verification (no settings to verify).");
        }
        Err(e) => return Err(e.into()),
    }

    // [3/3] Error case — get settings for a clearly non-existent project
    println!("\n[3/3] Getting settings for non-existent project (expect error)...");
    let result = aa
        .get_project_settings("nonexistent-project-id-00000000")
        .await;
    assert!(result.is_err(), "Should error on non-existent project");
    println!("  Got expected error: {:?}", result.unwrap_err());

    println!("\nAll Access Approval tests passed!");
    Ok(())
}
