//! Integration tests for BigQuery API
//!
//! Run with:
//!   task test:integration:bigquery

use gcp_lite::GcpHttpClient;
use std::env;
use std::time::Duration;

const TEST_DATASET_ID: &str = "bq_integ_test";
const TEST_TABLE_ID: &str = "bq_integ_test_table";

/// Delete the test dataset using bq CLI (for cleanup).
fn bq_delete_dataset(project_id: &str, dataset_id: &str) {
    let _ = std::process::Command::new("bq")
        .args(["rm", "-r", "-f", &format!("{}:{}", project_id, dataset_id)])
        .output();
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_bigquery_dataset_and_table_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== BigQuery Dataset & Table Lifecycle Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    bq_delete_dataset(&project_id, TEST_DATASET_ID);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let result = run_dataset_table_tests(&client, &project_id).await;

    // Always cleanup
    bq_delete_dataset(&project_id, TEST_DATASET_ID);

    result?;
    println!("\nAll BigQuery dataset & table tests passed!");
    Ok(())
}

async fn run_dataset_table_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bq = client.bigquery();

    // [1/8] Create dataset via bq CLI (we don't have create_dataset in our API)
    println!("\n[1/8] Creating test dataset via bq CLI...");
    let output = std::process::Command::new("bq")
        .args([
            "mk",
            "--dataset",
            "--location=US",
            &format!("{}:{}", project_id, TEST_DATASET_ID),
        ])
        .output()?;
    assert!(
        output.status.success(),
        "Failed to create dataset: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [2/8] Get dataset
    println!("\n[2/8] Getting dataset...");
    let dataset = bq.get_dataset(project_id, TEST_DATASET_ID).await?;
    let ds_ref = dataset.dataset_reference.unwrap();
    assert_eq!(ds_ref.dataset_id, TEST_DATASET_ID);
    assert_eq!(dataset.location, Some("US".to_string()));
    println!("  Dataset: {}", dataset.id.unwrap_or_default());

    // [3/8] List datasets
    println!("\n[3/8] Listing datasets...");
    let datasets = bq.list_datasets(project_id).await?;
    assert!(
        datasets.iter().any(|d| d
            .dataset_reference
            .as_ref()
            .is_some_and(|r| r.dataset_id == TEST_DATASET_ID)),
        "Test dataset not found in list"
    );
    println!("  Found {} datasets", datasets.len());

    // [4/8] Create test table via bq CLI
    println!("\n[4/8] Creating test table via bq CLI...");
    let output = std::process::Command::new("bq")
        .args([
            "mk",
            "--table",
            &format!("{}:{}.{}", project_id, TEST_DATASET_ID, TEST_TABLE_ID),
            "name:STRING,value:INTEGER",
        ])
        .output()?;
    assert!(
        output.status.success(),
        "Failed to create table: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [5/8] Get table
    println!("\n[5/8] Getting table...");
    let table = bq
        .get_table(project_id, TEST_DATASET_ID, TEST_TABLE_ID)
        .await?;
    let tref = table.table_reference.unwrap();
    assert_eq!(tref.table_id, TEST_TABLE_ID);
    assert_eq!(tref.dataset_id, TEST_DATASET_ID);
    assert_eq!(table.type_value, Some("TABLE".to_string()));
    println!("  Table: {}", table.id.unwrap_or_default());

    // [6/8] List tables
    println!("\n[6/8] Listing tables...");
    let tables = bq.list_tables(project_id, TEST_DATASET_ID).await?;
    assert!(
        tables.iter().any(|t| t
            .table_reference
            .as_ref()
            .is_some_and(|r| r.table_id == TEST_TABLE_ID)),
        "Test table not found in list"
    );
    println!("  Found {} tables", tables.len());

    // [7/8] Patch table
    println!("\n[7/8] Patching table description...");
    let patch_body = gcp_lite::types::bigquery::Table {
        description: Some("Updated by integration test".to_string()),
        friendly_name: Some("Integration Test Table".to_string()),
        ..Default::default()
    };
    let patched = bq
        .patch_table(project_id, TEST_DATASET_ID, TEST_TABLE_ID, &patch_body)
        .await?;
    assert_eq!(
        patched.description,
        Some("Updated by integration test".to_string())
    );
    assert_eq!(
        patched.friendly_name,
        Some("Integration Test Table".to_string())
    );
    println!("  Patched description and friendlyName");

    // [8/8] Error case: get non-existent dataset
    println!("\n[8/8] Getting non-existent dataset (expect error)...");
    let result = bq.get_dataset(project_id, "nonexistent_dataset_xyz").await;
    assert!(result.is_err(), "Should error on non-existent dataset");
    println!("  Correctly received error");

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_bigquery_jobs() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== BigQuery Jobs Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;
    let bq = client.bigquery();

    // [1/5] Run a synchronous query
    println!("\n[1/5] Running synchronous query...");
    let req = gcp_lite::types::bigquery::QueryRequest {
        query: "SELECT 'hello' AS greeting, 42 AS answer".to_string(),
        use_legacy_sql: Some(false),
        ..Default::default()
    };
    let response = bq.query(&project_id, &req).await?;
    assert_eq!(response.job_complete, Some(true));
    assert_eq!(response.total_rows, Some("1".to_string()));
    assert!(!response.rows.is_empty());
    println!("  Query complete, {} rows returned", response.rows.len());

    // Verify schema
    let schema = response.schema.unwrap();
    let fields = schema.fields;
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "greeting");
    assert_eq!(fields[1].name, "answer");
    println!("  Schema: {} fields", fields.len());

    // [2/5] Insert an async query job
    println!("\n[2/5] Inserting async query job...");
    let job_body = gcp_lite::types::bigquery::Job {
        configuration: Some(gcp_lite::types::bigquery::JobConfiguration {
            query: Some(gcp_lite::types::bigquery::JobConfigurationQuery {
                query: "SELECT 1 AS num".to_string(),
                use_legacy_sql: Some(false),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let inserted = bq.insert_job(&project_id, &job_body).await?;
    let job_ref = inserted.job_reference.as_ref().unwrap();
    let job_id = &job_ref.job_id;
    let location = job_ref.location.as_deref().unwrap_or("US");
    println!("  Job ID: {}", job_id);
    println!("  Location: {}", location);

    // Wait for job to complete
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [3/5] Get job by ID
    println!("\n[3/5] Getting job by ID...");
    let job = bq.get_job(&project_id, job_id).await?;
    let status = job.status.as_ref().unwrap();
    println!(
        "  Job state: {}",
        status.state.as_deref().unwrap_or("unknown")
    );

    // [4/5] Get job with location
    println!("\n[4/5] Getting job with explicit location...");
    let job = bq
        .get_job_with_location(&project_id, job_id, location)
        .await?;
    let status = job.status.as_ref().unwrap();
    assert_eq!(status.state, Some("DONE".to_string()));
    println!(
        "  Job state: {}",
        status.state.as_deref().unwrap_or("unknown")
    );

    // [5/7] List jobs
    println!("\n[5/7] Listing jobs...");
    let jobs = bq.list_jobs(&project_id).await?;
    assert!(
        !jobs.is_empty(),
        "Should have at least one job from the query we just ran"
    );
    println!("  Found {} jobs", jobs.len());

    // Verify our job is in the list
    let found = jobs.iter().any(|j| {
        j.job_reference
            .as_ref()
            .is_some_and(|r| r.job_id == *job_id)
    });
    assert!(found, "Our inserted job should be in the list");
    println!("  Verified our job is in the list");

    // [6/7] List jobs with single state filter
    println!("\n[6/7] Listing jobs with stateFilter=done...");
    use gcp_lite::api::bigquery::ListJobsOptions;

    let done_filter = ["done"];
    let done_options = ListJobsOptions {
        state_filters: Some(&done_filter),
        ..Default::default()
    };
    let done_jobs = bq
        .list_jobs_with_options(&project_id, &done_options)
        .await?;
    println!("  Found {} done jobs", done_jobs.len());
    assert!(
        !done_jobs.is_empty(),
        "Should have at least one done job (we just ran queries)"
    );
    // All returned jobs should be in done state
    for j in &done_jobs {
        let state = j.state.as_deref().unwrap_or("");
        assert_eq!(
            state.to_uppercase(),
            "DONE",
            "Expected DONE state, got {}",
            state
        );
    }

    // [7/11] List jobs with multiple state filters (repeated params edge case)
    println!(
        "\n[7/11] Listing jobs with stateFilter=done&stateFilter=running (repeated params)..."
    );
    let multi_filter = ["done", "running"];
    let multi_options = ListJobsOptions {
        state_filters: Some(&multi_filter),
        ..Default::default()
    };
    let multi_jobs = bq
        .list_jobs_with_options(&project_id, &multi_options)
        .await?;
    println!(
        "  Found {} jobs with done+running filters",
        multi_jobs.len()
    );
    // Should include at least the done jobs we already found
    assert!(
        multi_jobs.len() >= done_jobs.len(),
        "Multi-filter should return at least as many as single filter ({} >= {})",
        multi_jobs.len(),
        done_jobs.len()
    );
    // All returned jobs should be in done or running state
    for j in &multi_jobs {
        let state = j.state.as_deref().unwrap_or("").to_uppercase();
        assert!(
            state == "DONE" || state == "RUNNING",
            "Expected DONE or RUNNING state, got {}",
            state
        );
    }

    // [8/11] List jobs with all_users option
    println!("\n[8/11] Listing jobs with all_users=true...");
    let all_users_options = ListJobsOptions {
        all_users: Some(true),
        state_filters: Some(&done_filter),
        ..Default::default()
    };
    let all_users_jobs = bq
        .list_jobs_with_options(&project_id, &all_users_options)
        .await?;
    println!("  Found {} jobs with all_users=true", all_users_jobs.len());
    // all_users should return at least as many jobs as single-user
    assert!(
        all_users_jobs.len() >= done_jobs.len(),
        "all_users should return at least as many ({} >= {})",
        all_users_jobs.len(),
        done_jobs.len()
    );

    // [9/11] List jobs with min_creation_time (time range filter)
    println!("\n[9/11] Listing jobs with min_creation_time filter...");
    // Use a timestamp from 1 hour ago to capture recent jobs
    let one_hour_ago_ms = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - 3_600_000) as u64;
    let time_filter_str = one_hour_ago_ms.to_string();
    let time_options = ListJobsOptions {
        min_creation_time: Some(&time_filter_str),
        state_filters: Some(&done_filter),
        ..Default::default()
    };
    let time_jobs = bq
        .list_jobs_with_options(&project_id, &time_options)
        .await?;
    println!(
        "  Found {} done jobs created in the last hour",
        time_jobs.len()
    );
    // We just created jobs, so should have at least one
    assert!(
        !time_jobs.is_empty(),
        "Should find at least one job created in the last hour"
    );

    // [10/11] Stream jobs (basic stream test)
    println!("\n[10/11] Streaming jobs...");
    use futures::StreamExt;

    let stream = bq.list_jobs_stream(&project_id);
    futures::pin_mut!(stream);

    let mut stream_count = 0;
    let stream_limit = 20;
    while let Some(result) = stream.next().await {
        let _job = result?;
        stream_count += 1;
        if stream_count >= stream_limit {
            break;
        }
    }
    println!("  Streamed {} jobs", stream_count);
    assert!(stream_count > 0, "Stream should return at least one job");

    // [11/11] Stream jobs with state filter
    println!("\n[11/11] Streaming done jobs with state filter...");
    let stream_options = ListJobsOptions {
        state_filters: Some(&done_filter),
        ..Default::default()
    };
    let filtered_stream = bq.list_jobs_stream_with_options(&project_id, stream_options);
    futures::pin_mut!(filtered_stream);

    let mut filtered_count = 0;
    let filtered_limit = 10;
    while let Some(result) = filtered_stream.next().await {
        let job = result?;
        let state = job.state.as_deref().unwrap_or("").to_uppercase();
        assert_eq!(state, "DONE", "Streamed job should be DONE, got {}", state);
        filtered_count += 1;
        if filtered_count >= filtered_limit {
            break;
        }
    }
    println!("  Streamed {} filtered done jobs", filtered_count);

    println!("\nAll BigQuery jobs tests passed!");
    Ok(())
}

const TEST_IAM_DATASET_ID: &str = "bq_iam_integ_test";

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_dataset_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== BigQuery Dataset IAM / ACL Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;
    let bq = client.bigquery();

    // Pre-cleanup
    bq_delete_dataset(&project_id, TEST_IAM_DATASET_ID);
    tokio::time::sleep(Duration::from_secs(2)).await;

    let result = run_dataset_iam_tests(&bq, &project_id).await;

    // Always cleanup
    bq_delete_dataset(&project_id, TEST_IAM_DATASET_ID);

    result?;
    println!("\nAll BigQuery dataset IAM tests passed!");
    Ok(())
}

async fn run_dataset_iam_tests(
    bq: &gcp_lite::api::bigquery::BigqueryClient<'_>,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/5] Create dataset via bq CLI
    println!("\n[1/5] Creating test dataset via bq CLI...");
    let output = std::process::Command::new("bq")
        .args([
            "mk",
            "--dataset",
            "--location=US",
            &format!("{}:{}", project_id, TEST_IAM_DATASET_ID),
        ])
        .output()?;
    assert!(
        output.status.success(),
        "Failed to create dataset: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    tokio::time::sleep(Duration::from_secs(2)).await;

    // [2/5] Get initial ACL
    println!("\n[2/5] Getting initial dataset ACL...");
    let initial_acl = bq
        .get_dataset_iam_policy(project_id, TEST_IAM_DATASET_ID)
        .await?;
    println!("  Initial ACL entries: {}", initial_acl.len());
    assert!(
        !initial_acl.is_empty(),
        "Newly created dataset should have default ACL entries"
    );

    // Verify at least one entry has a role field (standard BigQuery ACL shape)
    let has_role = initial_acl.iter().any(|e| e.get("role").is_some());
    assert!(has_role, "ACL entries should have 'role' field");

    // [3/5] Add a specialGroup:allAuthenticatedUsers entry then restore
    println!("\n[3/5] Setting ACL (add allAuthenticatedUsers/READER entry)...");
    let mut new_acl = initial_acl.clone();
    new_acl.push(serde_json::json!({
        "role": "READER",
        "specialGroup": "allAuthenticatedUsers"
    }));
    let updated_acl = bq
        .set_dataset_iam_policy(project_id, TEST_IAM_DATASET_ID, new_acl)
        .await?;
    println!("  Updated ACL entries: {}", updated_acl.len());
    assert!(
        updated_acl.len() > initial_acl.len(),
        "ACL should have grown after adding entry"
    );
    let has_new_entry = updated_acl.iter().any(|e| {
        e.get("specialGroup").and_then(|v| v.as_str()) == Some("allAuthenticatedUsers")
            && e.get("role").and_then(|v| v.as_str()) == Some("READER")
    });
    assert!(
        has_new_entry,
        "New allAuthenticatedUsers/READER entry should be present"
    );

    // [4/5] Restore original ACL
    println!("\n[4/5] Restoring original ACL...");
    let restored_acl = bq
        .set_dataset_iam_policy(project_id, TEST_IAM_DATASET_ID, initial_acl.clone())
        .await?;
    println!("  Restored ACL entries: {}", restored_acl.len());
    assert_eq!(
        restored_acl.len(),
        initial_acl.len(),
        "Restored ACL should match original"
    );
    let still_has_entry = restored_acl
        .iter()
        .any(|e| e.get("specialGroup").and_then(|v| v.as_str()) == Some("allAuthenticatedUsers"));
    assert!(
        !still_has_entry,
        "allAuthenticatedUsers entry should be removed after restore"
    );

    // [5/5] Verify round-trip: get ACL matches restored ACL
    println!("\n[5/5] Verifying ACL round-trip...");
    let final_acl = bq
        .get_dataset_iam_policy(project_id, TEST_IAM_DATASET_ID)
        .await?;
    println!("  Final ACL entries: {}", final_acl.len());
    assert_eq!(
        final_acl.len(),
        initial_acl.len(),
        "Final ACL entry count should match initial"
    );

    Ok(())
}
