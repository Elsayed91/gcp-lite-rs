//! Integration tests for Cloud Monitoring API
//!
//! Tests metric descriptors, monitored resource descriptors, and time series queries.
//!
//! Run with:
//!   GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!     cargo test --test integration monitoring -- --ignored --test-threads=1 --nocapture

use gcp_lite::GcpHttpClient;
use gcp_lite::api::monitoring::TimeSeriesParams;
use gcp_lite::types::monitoring::*;
use std::collections::HashMap;
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// ── Metric Descriptors ───────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_metric_descriptors() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== List Metric Descriptors ===");

    // 1. List without filter — should return many descriptors
    println!("\n[1/3] Listing all metric descriptors (first page)...");
    let response = monitoring
        .list_metric_descriptors(&project, None, None)
        .await?;
    println!("  Got {} descriptors", response.metric_descriptors.len());
    assert!(
        !response.metric_descriptors.is_empty(),
        "project should have at least one metric descriptor"
    );

    // Verify basic structure of first descriptor
    let first = &response.metric_descriptors[0];
    assert!(
        !first.name.is_empty(),
        "descriptor name should not be empty"
    );
    assert!(
        first.metric_type.is_some(),
        "descriptor should have a metric type"
    );
    println!(
        "  First: {} ({})",
        first.name,
        first.metric_type.as_deref().unwrap_or("?")
    );

    // 2. List with filter — narrow to compute metrics
    println!("\n[2/3] Listing compute metric descriptors...");
    let filtered = monitoring
        .list_metric_descriptors(
            &project,
            Some("metric.type = starts_with(\"compute.googleapis.com/\")"),
            None,
        )
        .await?;
    println!(
        "  Got {} compute descriptors",
        filtered.metric_descriptors.len()
    );
    // Compute descriptors may or may not exist depending on project resources
    for desc in filtered.metric_descriptors.iter().take(3) {
        let mt = desc.metric_type.as_deref().unwrap_or("?");
        assert!(
            mt.starts_with("compute.googleapis.com/"),
            "filtered descriptor should be a compute metric, got: {}",
            mt
        );
        println!("  - {}", mt);
    }

    // 3. List with filter — serviceruntime metrics (always present if API is enabled)
    println!("\n[3/3] Listing serviceruntime metric descriptors...");
    let sr = monitoring
        .list_metric_descriptors(
            &project,
            Some("metric.type = starts_with(\"serviceruntime.googleapis.com/\")"),
            None,
        )
        .await?;
    println!(
        "  Got {} serviceruntime descriptors",
        sr.metric_descriptors.len()
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_get_metric_descriptor() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Get Metric Descriptor ===");

    // Get a well-known metric descriptor
    let metric_type = "compute.googleapis.com/instance/cpu/utilization";
    println!("\n[1/2] Getting descriptor for {}...", metric_type);
    let desc = monitoring
        .get_metric_descriptor(&project, metric_type)
        .await?;
    assert_eq!(desc.metric_type.as_deref(), Some(metric_type));
    assert!(desc.metric_kind.is_some());
    assert!(desc.value_type.is_some());
    println!("  Kind: {:?}", desc.metric_kind);
    println!("  Value type: {:?}", desc.value_type);
    println!("  Labels: {:?}", desc.labels.len());
    println!("  Unit: {:?}", desc.unit);

    // 2. Test non-existent metric descriptor
    println!("\n[2/2] Getting non-existent descriptor...");
    let result = monitoring
        .get_metric_descriptor(&project, "custom.googleapis.com/nonexistent/metric")
        .await;
    assert!(
        result.is_err(),
        "non-existent descriptor should return error"
    );
    println!("  Got expected error");

    Ok(())
}

// ── Monitored Resource Descriptors ───────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_monitored_resource_descriptors() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== List Monitored Resource Descriptors ===");

    // 1. List all
    println!("\n[1/2] Listing all monitored resource descriptors...");
    let response = monitoring
        .list_monitored_resource_descriptors(&project, None, None)
        .await?;
    println!(
        "  Got {} resource descriptors",
        response.resource_descriptors.len()
    );
    assert!(
        !response.resource_descriptors.is_empty(),
        "should have at least one resource descriptor"
    );

    // Verify structure
    let first = &response.resource_descriptors[0];
    assert!(!first.name.is_empty());
    assert!(first.resource_type.is_some());
    println!(
        "  First: {} ({})",
        first.name,
        first.resource_type.as_deref().unwrap_or("?")
    );

    // Print a few well-known types
    let well_known = ["gce_instance", "gcs_bucket", "global"];
    for wk in &well_known {
        let found = response
            .resource_descriptors
            .iter()
            .any(|d| d.resource_type.as_deref() == Some(wk));
        println!("  '{}' present: {}", wk, found);
    }

    // 2. With filter
    println!("\n[2/2] Listing with filter for 'gce_' resource types...");
    let filtered = monitoring
        .list_monitored_resource_descriptors(
            &project,
            Some("resource.type = starts_with(\"gce_\")"),
            None,
        )
        .await?;
    println!(
        "  Got {} gce_* descriptors",
        filtered.resource_descriptors.len()
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_get_monitored_resource_descriptor() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Get Monitored Resource Descriptor ===");

    // 1. Get a well-known resource type
    println!("\n[1/2] Getting 'gce_instance' descriptor...");
    let desc = monitoring
        .get_monitored_resource_descriptor(&project, "gce_instance")
        .await?;
    assert_eq!(desc.resource_type.as_deref(), Some("gce_instance"));
    assert!(!desc.labels.is_empty(), "gce_instance should have labels");
    println!("  Display name: {:?}", desc.display_name);
    println!("  Labels:");
    for label in &desc.labels {
        println!("    - {} ({:?})", label.key, label.value_type);
    }

    // 2. Test non-existent resource type
    println!("\n[2/2] Getting non-existent resource type...");
    let result = monitoring
        .get_monitored_resource_descriptor(&project, "nonexistent_type_xyz")
        .await;
    assert!(
        result.is_err(),
        "non-existent resource type should return error"
    );
    println!("  Got expected error");

    Ok(())
}

// ── Time Series ──────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_time_series() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== List Time Series ===");

    // Use serviceruntime API request count — should always have data if any APIs are used
    let filter = "metric.type=\"serviceruntime.googleapis.com/api/request_count\"";

    // Query the last 1 hour
    let now = chrono::Utc::now();
    let one_hour_ago = now - chrono::Duration::hours(1);
    let interval = TimeInterval {
        start_time: Some(one_hour_ago.to_rfc3339()),
        end_time: Some(now.to_rfc3339()),
    };

    // 1. Basic query without aggregation
    println!("\n[1/3] Querying serviceruntime request_count (last 1h)...");
    let response = monitoring
        .list_time_series(&project, filter, &interval, None)
        .await?;
    println!("  Got {} time series", response.time_series.len());

    if response.time_series.is_empty() {
        println!(
            "  WARNING: No time series data. This can happen if no API calls were made recently."
        );
        println!("  Trying with a wider window (last 24h)...");

        let one_day_ago = now - chrono::Duration::hours(24);
        let wider_interval = TimeInterval {
            start_time: Some(one_day_ago.to_rfc3339()),
            end_time: Some(now.to_rfc3339()),
        };
        let wider = monitoring
            .list_time_series(&project, filter, &wider_interval, None)
            .await?;
        println!("  Got {} time series (24h window)", wider.time_series.len());
    }

    // Verify structure of returned time series
    for ts in response.time_series.iter().take(3) {
        if let Some(ref metric) = ts.metric {
            println!(
                "  Metric type: {}",
                metric.metric_type.as_deref().unwrap_or("?")
            );
            println!("    Labels: {:?}", metric.labels);
        }
        if let Some(ref resource) = ts.resource {
            println!(
                "    Resource: {}",
                resource.resource_type.as_deref().unwrap_or("?")
            );
        }
        println!("    Points: {}", ts.points.len());
    }

    // 2. Query with aggregation (align and reduce)
    println!("\n[2/3] Querying with aggregation (ALIGN_RATE, 60s)...");
    let params = TimeSeriesParams {
        aggregation_alignment_period: Some("60s".to_string()),
        aggregation_per_series_aligner: Some("ALIGN_RATE".to_string()),
        ..Default::default()
    };

    let wider_interval = TimeInterval {
        start_time: Some((now - chrono::Duration::hours(1)).to_rfc3339()),
        end_time: Some(now.to_rfc3339()),
    };
    let aggregated = monitoring
        .list_time_series(&project, filter, &wider_interval, Some(&params))
        .await?;
    println!(
        "  Got {} aggregated time series",
        aggregated.time_series.len()
    );

    // 3. Query with cross-series reduction
    println!("\n[3/3] Querying with cross-series reduction...");
    let reduce_params = TimeSeriesParams {
        aggregation_alignment_period: Some("3600s".to_string()),
        aggregation_per_series_aligner: Some("ALIGN_RATE".to_string()),
        aggregation_cross_series_reducer: Some("REDUCE_SUM".to_string()),
        aggregation_group_by_fields: Some(vec!["resource.labels.service".to_string()]),
        ..Default::default()
    };
    let reduced = monitoring
        .list_time_series(&project, filter, &wider_interval, Some(&reduce_params))
        .await?;
    println!("  Got {} reduced time series", reduced.time_series.len());

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_time_series_pagination() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Time Series Pagination ===");

    let filter = "metric.type=\"serviceruntime.googleapis.com/api/request_count\"";
    let now = chrono::Utc::now();
    let interval = TimeInterval {
        start_time: Some((now - chrono::Duration::hours(24)).to_rfc3339()),
        end_time: Some(now.to_rfc3339()),
    };

    // Request with small page size to force pagination
    let params = TimeSeriesParams {
        page_size: Some(2),
        ..Default::default()
    };

    println!("[1/1] Querying with page_size=2...");
    let page1 = monitoring
        .list_time_series(&project, filter, &interval, Some(&params))
        .await?;
    println!("  Page 1: {} time series", page1.time_series.len());
    println!("  Has next page: {}", page1.next_page_token.is_some());

    if let Some(ref token) = page1.next_page_token {
        let page2_params = TimeSeriesParams {
            page_size: Some(2),
            page_token: Some(token.clone()),
            ..Default::default()
        };
        let page2 = monitoring
            .list_time_series(&project, filter, &interval, Some(&page2_params))
            .await?;
        println!("  Page 2: {} time series", page2.time_series.len());
    }

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_time_series_error_cases() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Time Series Error Cases ===");

    // 1. Missing end time should fail
    println!("\n[1/2] Querying with no end time (empty interval)...");
    let empty_interval = TimeInterval {
        start_time: None,
        end_time: None,
    };
    let result = monitoring
        .list_time_series(
            &project,
            "metric.type=\"compute.googleapis.com/instance/cpu/utilization\"",
            &empty_interval,
            None,
        )
        .await;
    assert!(result.is_err(), "missing endTime should fail");
    println!("  Got expected error");

    // 2. Invalid filter
    println!("\n[2/2] Querying with invalid filter...");
    let now = chrono::Utc::now();
    let interval = TimeInterval {
        start_time: Some((now - chrono::Duration::hours(1)).to_rfc3339()),
        end_time: Some(now.to_rfc3339()),
    };
    let result = monitoring
        .list_time_series(&project, "this is not a valid filter!!!", &interval, None)
        .await;
    assert!(result.is_err(), "invalid filter should fail");
    println!("  Got expected error");

    Ok(())
}

// ── Streaming ────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_metric_descriptors_stream() -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;

    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Metric Descriptors Stream ===");

    // Stream compute metric descriptors (limited set)
    println!("[1/1] Streaming compute metric descriptors...");
    let stream = monitoring.list_metric_descriptors_stream(
        &project,
        Some("metric.type = starts_with(\"compute.googleapis.com/\")"),
    );
    futures::pin_mut!(stream);

    let mut count = 0;
    while let Some(result) = stream.next().await {
        let desc = result?;
        count += 1;
        if count <= 5 {
            println!(
                "  [{}] {}",
                count,
                desc.metric_type.as_deref().unwrap_or("?")
            );
        }
    }
    println!("  Total compute descriptors streamed: {}", count);

    Ok(())
}

// ── Alert Policies ────────────────────────────────────────────────────────────

const TEST_ALERT_POLICY: &str = "cloud-lite-test-ralph-alert";
const TEST_NC: &str = "cloud-lite-test-ralph-nc";

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_alert_policies_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Alert Policy Lifecycle ===");
    println!("Project: {}", project);

    // Pre-cleanup: delete any leftover policy with our display name
    let existing = monitoring
        .list_alert_policies(
            &project,
            Some(&format!("displayName=\"{}\"", TEST_ALERT_POLICY)),
        )
        .await?;
    for p in &existing {
        if let Some(ref n) = p.name {
            let _ = monitoring.delete_alert_policy(n).await;
        }
    }

    // === 1. LIST (baseline) ===
    println!("\n[1/5] Listing alert policies (baseline)...");
    let before = monitoring.list_alert_policies(&project, None).await?;
    println!("  Found {} existing policies", before.len());

    // === 2. CREATE ===
    println!("\n[2/5] Creating alert policy '{}'...", TEST_ALERT_POLICY);
    let policy = AlertPolicy {
        display_name: Some(TEST_ALERT_POLICY.to_string()),
        enabled: Some(false),
        conditions: vec![Condition {
            display_name: Some("Test threshold condition".to_string()),
            condition_threshold: Some(serde_json::json!({
                "filter": "resource.type=\"consumed_api\" AND metric.type=\"serviceruntime.googleapis.com/api/request_count\"",
                "comparison": "COMPARISON_GT",
                "thresholdValue": 1000000,
                "duration": "60s",
                "aggregations": [{
                    "alignmentPeriod": "60s",
                    "perSeriesAligner": "ALIGN_RATE"
                }]
            })),
            ..Default::default()
        }],
        combiner: Some("OR".to_string()),
        ..Default::default()
    };
    let created = monitoring.create_alert_policy(&project, &policy).await?;
    let created_name = created
        .name
        .clone()
        .expect("created policy must have a name");
    assert_eq!(
        created.display_name.as_deref(),
        Some(TEST_ALERT_POLICY),
        "display name mismatch"
    );
    assert_eq!(created.enabled, Some(false), "policy should be disabled");
    println!("  Created: {}", created_name);
    println!("  Enabled: {:?}", created.enabled);

    // === 3. LIST (verify inclusion) ===
    println!("\n[3/5] Listing alert policies (after create)...");
    let after = monitoring.list_alert_policies(&project, None).await?;
    assert!(
        after.len() >= before.len() + 1,
        "should have one more policy after create, before={} after={}",
        before.len(),
        after.len()
    );
    let found = after
        .iter()
        .any(|p| p.name.as_deref() == Some(&created_name));
    assert!(found, "created policy should appear in list");
    println!("  Found {} policies, test policy present", after.len());

    // === 4. LIST with filter ===
    println!("\n[4/5] Listing with display name filter...");
    let filtered = monitoring
        .list_alert_policies(
            &project,
            Some(&format!("displayName=\"{}\"", TEST_ALERT_POLICY)),
        )
        .await?;
    assert!(!filtered.is_empty(), "filter should return the test policy");
    assert_eq!(filtered[0].display_name.as_deref(), Some(TEST_ALERT_POLICY));
    println!("  Filtered list returned {} policy(ies)", filtered.len());

    // === 5. DELETE ===
    println!("\n[5/5] Deleting alert policy...");
    monitoring.delete_alert_policy(&created_name).await?;
    println!("  Deleted: {}", created_name);

    // Verify deletion: list with filter should return empty
    let after_delete = monitoring
        .list_alert_policies(
            &project,
            Some(&format!("displayName=\"{}\"", TEST_ALERT_POLICY)),
        )
        .await?;
    assert!(
        after_delete.is_empty(),
        "deleted policy should not appear in list"
    );
    println!("  Confirmed: policy no longer in list");

    Ok(())
}

// ── Notification Channels ─────────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_notification_channels_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let monitoring = client.monitoring();

    println!("\n=== Notification Channel Lifecycle ===");
    println!("Project: {}", project);

    // Pre-cleanup: delete any leftover channel with our display name
    let existing = monitoring
        .list_notification_channels(&project, Some(&format!("displayName=\"{}\"", TEST_NC)))
        .await?;
    for c in &existing {
        if let Some(ref n) = c.name {
            let _ = monitoring.delete_notification_channel(n).await;
        }
    }

    // === 1. LIST (baseline) ===
    println!("\n[1/5] Listing notification channels (baseline)...");
    let before = monitoring
        .list_notification_channels(&project, None)
        .await?;
    println!("  Found {} existing channels", before.len());

    // === 2. CREATE ===
    println!("\n[2/5] Creating notification channel '{}'...", TEST_NC);
    let mut labels = HashMap::new();
    labels.insert(
        "email_address".to_string(),
        "noreply@cloud-lite-test.invalid".to_string(),
    );
    let channel = NotificationChannel {
        channel_type: Some("email".to_string()),
        display_name: Some(TEST_NC.to_string()),
        description: Some("cloud-lite integration test channel".to_string()),
        enabled: Some(false),
        labels: Some(labels),
        ..Default::default()
    };
    let created = monitoring
        .create_notification_channel(&project, &channel)
        .await?;
    let created_name = created
        .name
        .clone()
        .expect("created channel must have a name");
    assert_eq!(
        created.display_name.as_deref(),
        Some(TEST_NC),
        "display name mismatch"
    );
    assert_eq!(
        created.channel_type.as_deref(),
        Some("email"),
        "channel type mismatch"
    );
    assert_eq!(created.enabled, Some(false), "channel should be disabled");
    println!("  Created: {}", created_name);
    println!("  Type: {:?}", created.channel_type);
    println!("  Enabled: {:?}", created.enabled);

    // === 3. LIST (verify inclusion) ===
    println!("\n[3/5] Listing notification channels (after create)...");
    let after = monitoring
        .list_notification_channels(&project, None)
        .await?;
    assert!(
        after.len() >= before.len() + 1,
        "should have one more channel after create, before={} after={}",
        before.len(),
        after.len()
    );
    let found = after
        .iter()
        .any(|c| c.name.as_deref() == Some(&created_name));
    assert!(found, "created channel should appear in list");
    println!("  Found {} channels, test channel present", after.len());

    // === 4. LIST with filter ===
    println!("\n[4/5] Listing with display name filter...");
    let filtered = monitoring
        .list_notification_channels(&project, Some(&format!("displayName=\"{}\"", TEST_NC)))
        .await?;
    assert!(
        !filtered.is_empty(),
        "filter should return the test channel"
    );
    assert_eq!(filtered[0].display_name.as_deref(), Some(TEST_NC));
    println!("  Filtered list returned {} channel(s)", filtered.len());

    // === 5. DELETE ===
    println!("\n[5/5] Deleting notification channel...");
    monitoring
        .delete_notification_channel(&created_name)
        .await?;
    println!("  Deleted: {}", created_name);

    // Verify deletion
    let after_delete = monitoring
        .list_notification_channels(&project, Some(&format!("displayName=\"{}\"", TEST_NC)))
        .await?;
    assert!(
        after_delete.is_empty(),
        "deleted channel should not appear in list"
    );
    println!("  Confirmed: channel no longer in list");

    Ok(())
}
