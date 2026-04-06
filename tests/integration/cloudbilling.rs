//! Integration tests for Cloud Billing API
//!
//! Run with:
//!   GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!     cargo test --test integration cloudbilling -- --ignored --nocapture

use gcp_lite::GcpHttpClient;
use std::env;

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_cloudbilling_info_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Cloud Billing Info Lifecycle Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;

    // Ensure Cloud Billing API is enabled
    println!("\n[0/2] Ensuring Cloud Billing API is enabled...");
    let service_usage = client.service_usage();
    let api_name = "cloudbilling.googleapis.com";
    let enabled = service_usage
        .is_service_enabled(&project_id, api_name)
        .await?;
    if !enabled {
        println!("  Enabling {}...", api_name);
        service_usage.enable_service(&project_id, api_name).await?;
        // Wait for propagation
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    } else {
        println!("  Already enabled");
    }

    let billing = client.billing();

    // === 1. GET Billing Info ===
    println!("\n[1/2] Getting billing info...");
    let info = billing.get_billing_info(&project_id).await?;
    println!("  Billing Enabled: {:?}", info.billing_enabled);
    println!("  Billing Account: {:?}", info.billing_account_name);

    assert_eq!(info.project_id, Some(project_id.clone()));
    println!("  Confirmed project ID matches");

    // === 2. Update Billing Info (Link/Unlink) ===
    // CAUTION: This modifies project billing. We will restore it.

    if let Some(ref account_name) = info.billing_account_name {
        if account_name.is_empty() {
            println!(
                "\n[2/2] Project has no billing account. Skipping update test to avoid needing a known billing account ID."
            );
            return Ok(());
        }

        let original_id = account_name
            .strip_prefix("billingAccounts/")
            .expect("billing account name should start with billingAccounts/");

        println!("\n[2/2] Testing unlink/relink cycle...");
        println!("  Original account: {}", original_id);

        // A. Unlink (Disable Billing)
        println!("  Disabling billing...");
        let disabled = billing.update_billing_info(&project_id, None).await?;
        assert_eq!(disabled.billing_enabled, Some(false));
        assert!(
            disabled
                .billing_account_name
                .as_deref()
                .unwrap_or("")
                .is_empty()
        );
        println!("  Billing disabled successfully");

        // B. Relink (Restore Original)
        println!("  Restoring billing...");
        let restored = billing
            .update_billing_info(&project_id, Some(original_id))
            .await?;
        assert_eq!(restored.billing_enabled, Some(true));
        assert_eq!(
            restored.billing_account_name,
            Some(format!("billingAccounts/{}", original_id))
        );
        println!("  Billing restored successfully");
    } else {
        println!("\n[2/3] Project has no billing account. Skipping update test.");
    }

    // === 3. ERROR: Get billing for non-existent project ===
    println!("\n[3/3] Getting billing info for non-existent project (expect error)...");
    let result = billing
        .get_billing_info("nonexistent-project-xyz-99999")
        .await;
    assert!(result.is_err(), "Non-existent project should return error");
    println!("  Non-existent project billing: error (correct)");

    Ok(())
}
