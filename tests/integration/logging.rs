//! Integration tests for Cloud Logging API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 2.2 (logging_sinks_configured): ListSinks, CreateSink, DeleteSink
//!   - CIS 2.3 (logging_bucket_retention_locked): ListSinks (find destination bucket)
//!   - CIS 2.4-2.11 (alert policies): ListMetrics, CreateMetric, DeleteMetric
//!
//! Resources created and cleaned up by each test:
//!   Sink: cloud-lite-test-ralph-logging-sink (destination: _Default log bucket)
//!   Metric: cloud-lite-test-ralph-logging-metric (filter: severity >= ERROR)
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration logging -- --ignored --nocapture
//! ```

use gcp_lite::{
    GcpHttpClient,
    types::logging::{LogMetric, LogSink},
};
use std::env;

const TEST_SINK: &str = "cloud-lite-test-ralph-logging-sink";
const TEST_METRIC: &str = "cloud-lite-test-ralph-logging-metric";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// =============================================================================
// Integration Tests
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_logging_sinks_lifecycle() {
    let project = project_id();

    println!("\n=== Cloud Logging Sinks Lifecycle Test ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let logging = client.logging();

    // Always cleanup before and after
    let _ = logging.delete_sink(&project, TEST_SINK).await;

    let result = run_sinks_lifecycle(&logging, &project).await;

    // Cleanup
    let _ = logging.delete_sink(&project, TEST_SINK).await;

    result.expect("Cloud Logging sinks lifecycle tests failed");
    println!("\nAll Cloud Logging sinks tests passed!");
}

async fn run_sinks_lifecycle(
    logging: &gcp_lite::api::LoggingClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Destination: the project's default log bucket (always exists, no IAM setup needed)
    let destination = format!(
        "logging.googleapis.com/projects/{}/locations/global/buckets/_Default",
        project
    );

    // === 1. LIST SINKS (baseline) ===
    println!("\n[1/5] Listing sinks (baseline)...");
    let sinks_before = logging.list_sinks(project).await?;
    println!("  Found {} existing sinks", sinks_before.len());

    // === 2. CREATE SINK ===
    println!("\n[2/5] Creating sink '{}'...", TEST_SINK);
    let new_sink = LogSink {
        name: TEST_SINK.to_string(),
        destination: destination.clone(),
        filter: Some("severity >= ERROR".to_string()),
        description: Some("cloud-lite integration test sink".to_string()),
        ..Default::default()
    };
    let created = logging.create_sink(project, &new_sink).await?;
    assert_eq!(
        created.name, TEST_SINK,
        "Created sink name should be '{}', got: {}",
        TEST_SINK, created.name
    );
    assert_eq!(
        created.destination, destination,
        "Created sink destination mismatch"
    );
    assert!(
        created.filter.as_deref() == Some("severity >= ERROR"),
        "Created sink filter should be 'severity >= ERROR', got: {:?}",
        created.filter
    );
    println!("  Created sink: {}", created.name);
    println!("  Destination: {}", created.destination);
    println!("  Writer identity: {:?}", created.writer_identity);

    // === 3. GET SINK ===
    println!("\n[3/5] Getting sink '{}'...", TEST_SINK);
    let got = logging.get_sink(project, TEST_SINK).await?;
    assert_eq!(
        got.name, TEST_SINK,
        "Got sink name should be '{}', got: {}",
        TEST_SINK, got.name
    );
    assert_eq!(
        got.destination, destination,
        "Got sink destination mismatch"
    );
    println!("  Got sink: {}", got.name);
    println!("  Create time: {:?}", got.create_time);

    // === 4. LIST SINKS (verify inclusion) ===
    println!("\n[4/5] Listing sinks (expect test sink in list)...");
    let sinks_after = logging.list_sinks(project).await?;
    assert!(
        sinks_after.len() >= sinks_before.len() + 1,
        "Should have at least one more sink after creation, before={} after={}",
        sinks_before.len(),
        sinks_after.len()
    );
    let found = sinks_after.iter().any(|s| s.name == TEST_SINK);
    assert!(
        found,
        "Should find test sink '{}' in list, got: {:?}",
        TEST_SINK,
        sinks_after.iter().map(|s| &s.name).collect::<Vec<_>>()
    );
    println!("  Found {} sinks, test sink present", sinks_after.len());

    // === 5. DELETE SINK ===
    println!("\n[5/5] Deleting sink '{}'...", TEST_SINK);
    logging.delete_sink(project, TEST_SINK).await?;
    println!("  Deleted sink successfully");

    // Verify deletion
    let err = logging.get_sink(project, TEST_SINK).await;
    assert!(err.is_err(), "Getting deleted sink should return an error");
    println!("  Confirmed sink is gone (get returns error)");

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_logging_metrics_lifecycle() {
    let project = project_id();

    println!("\n=== Cloud Logging Metrics Lifecycle Test ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let logging = client.logging();

    // Always cleanup before and after
    let _ = logging.delete_metric(&project, TEST_METRIC).await;

    let result = run_metrics_lifecycle(&logging, &project).await;

    // Cleanup
    let _ = logging.delete_metric(&project, TEST_METRIC).await;

    result.expect("Cloud Logging metrics lifecycle tests failed");
    println!("\nAll Cloud Logging metrics tests passed!");
}

async fn run_metrics_lifecycle(
    logging: &gcp_lite::api::LoggingClient<'_>,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // === 1. LIST METRICS (baseline) ===
    println!("\n[1/5] Listing metrics (baseline)...");
    let metrics_before = logging.list_metrics(project).await?;
    println!("  Found {} existing metrics", metrics_before.len());

    // === 2. CREATE METRIC ===
    println!("\n[2/5] Creating metric '{}'...", TEST_METRIC);
    let new_metric = LogMetric {
        name: TEST_METRIC.to_string(),
        filter: "severity >= ERROR".to_string(),
        description: Some("cloud-lite integration test metric".to_string()),
        ..Default::default()
    };
    let created = logging.create_metric(project, &new_metric).await?;
    assert_eq!(
        created.name, TEST_METRIC,
        "Created metric name should be '{}', got: {}",
        TEST_METRIC, created.name
    );
    assert_eq!(
        created.filter, "severity >= ERROR",
        "Created metric filter should be 'severity >= ERROR', got: {}",
        created.filter
    );
    println!("  Created metric: {}", created.name);
    println!("  Filter: {}", created.filter);
    println!("  Create time: {:?}", created.create_time);

    // === 3. GET METRIC ===
    println!("\n[3/5] Getting metric '{}'...", TEST_METRIC);
    let got = logging.get_metric(project, TEST_METRIC).await?;
    assert_eq!(
        got.name, TEST_METRIC,
        "Got metric name should be '{}', got: {}",
        TEST_METRIC, got.name
    );
    assert_eq!(
        got.filter, "severity >= ERROR",
        "Got metric filter mismatch"
    );
    println!("  Got metric: {}", got.name);
    println!("  Filter: {}", got.filter);

    // === 4. LIST METRICS (verify inclusion) ===
    println!("\n[4/5] Listing metrics (expect test metric in list)...");
    let metrics_after = logging.list_metrics(project).await?;
    assert!(
        metrics_after.len() >= metrics_before.len() + 1,
        "Should have at least one more metric after creation, before={} after={}",
        metrics_before.len(),
        metrics_after.len()
    );
    let found = metrics_after.iter().any(|m| m.name == TEST_METRIC);
    assert!(
        found,
        "Should find test metric '{}' in list, got: {:?}",
        TEST_METRIC,
        metrics_after.iter().map(|m| &m.name).collect::<Vec<_>>()
    );
    println!(
        "  Found {} metrics, test metric present",
        metrics_after.len()
    );

    // === 5. DELETE METRIC ===
    println!("\n[5/5] Deleting metric '{}'...", TEST_METRIC);
    logging.delete_metric(project, TEST_METRIC).await?;
    println!("  Deleted metric successfully");

    // Verify deletion
    let err = logging.get_metric(project, TEST_METRIC).await;
    assert!(
        err.is_err(),
        "Getting deleted metric should return an error"
    );
    println!("  Confirmed metric is gone (get returns error)");

    Ok(())
}
