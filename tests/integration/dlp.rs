//! Integration tests for Cloud DLP (Sensitive Data Protection) API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 7.4 (bq_data_classification): verify Sensitive Data Protection is
//!     configured and scanning BigQuery datasets for sensitive data.
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration dlp -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── DLP discovery configs + data profiles ─────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_dlp_list_configs_and_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let dlp = client.dlp();

    println!("\n=== Cloud DLP Integration Test ===");
    println!("Project: {}", project);

    // [1/2] List discovery configs
    println!("\n[1/2] Listing discovery configs (location: global)...");
    let configs = dlp.list_discovery_configs(&project, "global").await?;
    println!("  Found {} discovery config(s)", configs.len());
    for cfg in &configs {
        println!(
            "  - {} (status: {:?}, lastRun: {:?})",
            cfg.name, cfg.status, cfg.last_run_time
        );
    }
    // CIS 7.4: projects may have zero configs (non-compliant).
    // Verify structure of any returned configs.
    for cfg in &configs {
        assert!(
            cfg.name.contains(&project),
            "discovery config name should contain project ID"
        );
    }

    // [2/2] List project data profiles
    println!("\n[2/2] Listing project data profiles (location: global)...");
    let profiles = dlp.list_project_data_profiles(&project, "global").await?;
    println!("  Found {} project data profile(s)", profiles.len());
    for prof in profiles.iter().take(3) {
        let sens = prof
            .sensitivity_score
            .as_ref()
            .and_then(|s| s.score.as_deref())
            .unwrap_or("<unknown>");
        let risk = prof
            .data_risk_level
            .as_ref()
            .and_then(|r| r.score.as_deref())
            .unwrap_or("<unknown>");
        println!("  - {} (sensitivity: {}, risk: {})", prof.name, sens, risk);
    }
    // CIS 7.4: zero profiles means DLP is not scanning this project.
    for prof in &profiles {
        assert!(
            prof.name.contains(&project),
            "project data profile name should contain project ID"
        );
    }

    println!("\nAll DLP tests passed!");
    Ok(())
}
