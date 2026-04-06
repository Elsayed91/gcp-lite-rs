//! Integration tests for Cloud Scheduler API
//!
//! Run with:
//!   GCLOUD_PROJECT_ID=<project> task test:integration:scheduler

use gcp_lite::GcpHttpClient;
use gcp_lite::types::cloudscheduler::{HttpTarget, Job, JobState};
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_LOCATION: &str = "us-central1";
const TEST_JOB_ID: &str = "scheduler-integ-test";
const TEST_ACTIONS_JOB_ID: &str = "scheduler-actions-test";

fn gcloud_delete_job(project_id: &str, location: &str, job_id: &str) {
    let _ = Command::new("gcloud")
        .args([
            "scheduler",
            "jobs",
            "delete",
            job_id,
            "--project",
            project_id,
            "--location",
            location,
            "--quiet",
        ])
        .output();
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_scheduler_job_crud_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Cloud Scheduler Job CRUD Lifecycle Test ===");
    println!("Project: {}", project_id);
    println!("Location: {}", TEST_LOCATION);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    println!("\n[0/6] Pre-cleanup...");
    gcloud_delete_job(&project_id, TEST_LOCATION, TEST_JOB_ID);
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Run tests, always cleanup
    let result = run_crud_tests(&client, &project_id).await;
    gcloud_delete_job(&project_id, TEST_LOCATION, TEST_JOB_ID);

    result?;
    println!("\nAll Cloud Scheduler CRUD tests passed!");
    Ok(())
}

async fn run_crud_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = client.scheduler();
    let job_name = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, TEST_JOB_ID
    );

    // [1/6] CREATE
    println!("\n[1/6] Creating job...");
    let job = Job {
        name: job_name.clone(),
        description: Some("Integration test job".to_string()),
        schedule: Some("0 9 * * 1".to_string()), // Every Monday at 9 AM
        time_zone: Some("America/New_York".to_string()),
        http_target: Some(HttpTarget {
            uri: Some("https://httpbin.org/post".to_string()),
            http_method: None,
            ..Default::default()
        }),
        ..Default::default()
    };
    let created = scheduler
        .create_job(project_id, TEST_LOCATION, &job)
        .await?;
    assert_eq!(created.name, job_name);
    assert_eq!(
        created.description,
        Some("Integration test job".to_string())
    );
    println!("  Created job: {}", created.name);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // [2/6] GET
    println!("\n[2/6] Getting job...");
    let fetched = scheduler
        .get_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    assert_eq!(fetched.name, job_name);
    assert_eq!(fetched.schedule, Some("0 9 * * 1".to_string()));
    println!("  Fetched job: {}", fetched.name);
    println!("  Schedule: {:?}", fetched.schedule);

    // [3/6] LIST
    println!("\n[3/6] Listing jobs...");
    let jobs = scheduler.list_jobs(project_id, TEST_LOCATION).await?;
    let found = jobs.iter().any(|j| j.name == job_name);
    assert!(found, "Created job should appear in list");
    println!("  Found {} jobs in location", jobs.len());
    println!("  Test job found in list: {}", found);

    // [4/6] UPDATE
    println!("\n[4/6] Updating job...");
    let mut updated_job = fetched.clone();
    updated_job.description = Some("Updated integration test job".to_string());
    let updated = scheduler
        .update_job(
            project_id,
            TEST_LOCATION,
            TEST_JOB_ID,
            &updated_job,
            "description",
        )
        .await?;
    assert_eq!(
        updated.description,
        Some("Updated integration test job".to_string())
    );
    println!("  Updated description: {:?}", updated.description);

    // [5/6] DELETE
    println!("\n[5/6] Deleting job...");
    scheduler
        .delete_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    println!("  Deleted job successfully");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // [6/6] GET deleted (expect error)
    println!("\n[6/6] Verifying job deleted (expect NotFound error)...");
    let get_deleted = scheduler
        .get_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await;
    assert!(get_deleted.is_err(), "Deleted job should return error");
    println!("  Correctly received error for deleted job");

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_scheduler_job_actions() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Cloud Scheduler Job Actions Test ===");
    println!("Project: {}", project_id);
    println!("Location: {}", TEST_LOCATION);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    println!("\n[0/5] Pre-cleanup...");
    gcloud_delete_job(&project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID);
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Run tests, always cleanup
    let result = run_actions_tests(&client, &project_id).await;
    gcloud_delete_job(&project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID);

    result?;
    println!("\nAll Cloud Scheduler actions tests passed!");
    Ok(())
}

async fn run_actions_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = client.scheduler();
    let job_name = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID
    );

    // [1/5] CREATE job for testing actions
    println!("\n[1/5] Creating job for actions test...");
    let job = Job {
        name: job_name.clone(),
        description: Some("Actions test job".to_string()),
        schedule: Some("0 9 * * 1".to_string()),
        time_zone: Some("America/New_York".to_string()),
        http_target: Some(HttpTarget {
            uri: Some("https://httpbin.org/post".to_string()),
            http_method: None,
            ..Default::default()
        }),
        ..Default::default()
    };
    let created = scheduler
        .create_job(project_id, TEST_LOCATION, &job)
        .await?;
    println!("  Created job: {}", created.name);
    println!("  Initial state: {:?}", created.state);
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [2/5] PAUSE job
    println!("\n[2/5] Pausing job...");
    let paused = scheduler
        .pause_job(project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID)
        .await?;
    assert_eq!(paused.state, Some(JobState::Paused));
    println!("  Job state after pause: {:?}", paused.state);

    // [3/5] RESUME job
    println!("\n[3/5] Resuming job...");
    let resumed = scheduler
        .resume_job(project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID)
        .await?;
    assert_eq!(resumed.state, Some(JobState::Enabled));
    println!("  Job state after resume: {:?}", resumed.state);

    // [4/5] RUN job (force immediate execution)
    println!("\n[4/5] Running job immediately...");
    let run_result = scheduler
        .run_job(project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID)
        .await?;
    println!("  Job triggered successfully");
    println!("  Last attempt time: {:?}", run_result.last_attempt_time);

    // [5/5] DELETE cleanup
    println!("\n[5/5] Cleaning up...");
    scheduler
        .delete_job(project_id, TEST_LOCATION, TEST_ACTIONS_JOB_ID)
        .await?;
    println!("  Job deleted");

    Ok(())
}
