//! Integration tests for App Engine API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 4.10 (compute_app_engine_https): verify services enforce HTTPS connections
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration appengine -- --ignored --nocapture
//! ```

use gcp_lite::{GcpError, GcpHttpClient};
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── App Engine get + list services ───────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_appengine_get_and_list() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let ae = client.appengine();

    println!("\n=== App Engine Integration Test ===");
    println!("Project: {}", project);

    // [1/3] Get the App Engine application
    println!("\n[1/3] Getting App Engine application for project...");
    match ae.get_app(&project).await {
        Ok(app) => {
            println!(
                "  App Engine IS deployed: {} (status: {:?})",
                app.name, app.serving_status
            );
            assert!(
                app.name.contains(&project),
                "app.name should contain project ID, got: {}",
                app.name
            );

            // [2/3] List all services (only if app exists)
            println!("\n[2/3] Listing services...");
            let services = ae.list_services(&project).await?;
            println!("  Found {} service(s)", services.len());
            for svc in &services {
                let ingress = svc
                    .network_settings
                    .as_ref()
                    .and_then(|ns| ns.ingress_traffic_allowed.as_deref())
                    .unwrap_or("<not set>");
                println!(
                    "  - {} (ingressTrafficAllowed: {})",
                    svc.id.as_deref().unwrap_or("?"),
                    ingress
                );
            }

            // [3/3] Get the default service if it exists
            println!("\n[3/3] Getting 'default' service (if present)...");
            let has_default = services.iter().any(|s| s.id.as_deref() == Some("default"));
            if has_default {
                let svc = ae.get_service(&project, "default").await?;
                println!("  Service: {}", svc.name);
                assert!(
                    svc.name.contains("default"),
                    "service name should contain 'default', got: {}",
                    svc.name
                );
            } else {
                println!("  No 'default' service found — skipping get_service check.");
            }
        }
        Err(GcpError::NotFound { .. }) => {
            println!("  App Engine NOT deployed on this project (NotFound).");
            println!("  Treat as non-compliant for CIS 4.10 if App Engine is expected.");
            println!("\n[2/3] Skipping list_services (no App Engine app).");
            println!("\n[3/3] Skipping get_service (no App Engine app).");
        }
        Err(e) => return Err(e.into()),
    }

    println!("\nAll App Engine tests passed!");
    Ok(())
}
