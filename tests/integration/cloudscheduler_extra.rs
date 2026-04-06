//! Extended Cloud Scheduler integration tests.
//!
//! Ported from the arbiter project's scheduler integration tests.
//! Tests detailed field verification, gcloud cross-verification,
//! error handling, and a full E2E CloudSQL stop/start workflow.
//!
//! Run with:
//!   GCLOUD_PROJECT_ID=<project> cargo test --test integration cloudscheduler_extra -- --ignored --test-threads=1 --nocapture

use base64::Engine;
use gcp_lite::GcpHttpClient;
use gcp_lite::types::cloudscheduler::*;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_LOCATION: &str = "us-central1";
const TEST_JOB_ID: &str = "integ-extra-scheduler-job";
const TEST_JOB_ID_2: &str = "integ-extra-scheduler-job-2";

// E2E CloudSQL + Scheduler constants
const E2E_INSTANCE_NAME: &str = "integ-sched-sql-v3";
const E2E_SA_NAME: &str = "integ-extra-sched-sa";
const E2E_STOP_JOB_ID: &str = "integ-extra-stop-cloudsql";
const E2E_START_JOB_ID: &str = "integ-extra-start-cloudsql";
const E2E_SA_ROLE: &str = "roles/cloudsql.admin";

// ── Helpers ──────────────────────────────────────────────────────────

/// Delete a scheduler job via gcloud (idempotent cleanup).
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

/// Check if a scheduler job exists via gcloud.
fn job_exists(project_id: &str, location: &str, job_id: &str) -> bool {
    let output = Command::new("gcloud")
        .args([
            "scheduler",
            "jobs",
            "describe",
            job_id,
            "--project",
            project_id,
            "--location",
            location,
            "--quiet",
        ])
        .output();
    output.map(|o| o.status.success()).unwrap_or(false)
}

/// Get job state via gcloud.
fn get_job_state(project_id: &str, location: &str, job_id: &str) -> Option<String> {
    let output = Command::new("gcloud")
        .args([
            "scheduler",
            "jobs",
            "describe",
            job_id,
            "--project",
            project_id,
            "--location",
            location,
            "--format",
            "value(state)",
            "--quiet",
        ])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn e2e_sa_email(project_id: &str) -> String {
    format!("{}@{}.iam.gserviceaccount.com", E2E_SA_NAME, project_id)
}

fn e2e_sa_member(project_id: &str) -> String {
    format!("serviceAccount:{}", e2e_sa_email(project_id))
}

fn get_project_number(project_id: &str) -> Option<String> {
    let output = Command::new("gcloud")
        .args([
            "projects",
            "describe",
            project_id,
            "--format",
            "value(projectNumber)",
        ])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn cleanup_cloudsql_instance(project_id: &str, instance_name: &str) {
    println!("  Deleting CloudSQL instance: {}", instance_name);
    let _ = Command::new("gcloud")
        .args([
            "sql",
            "instances",
            "delete",
            instance_name,
            "--project",
            project_id,
            "--quiet",
        ])
        .output();
}

fn cleanup_sa(project_id: &str) {
    let email = e2e_sa_email(project_id);
    println!("  Deleting SA: {}", email);
    let _ = Command::new("gcloud")
        .args([
            "iam",
            "service-accounts",
            "delete",
            &email,
            "--project",
            project_id,
            "--quiet",
        ])
        .output();
}

fn cleanup_iam_binding(project_id: &str, member: &str, role: &str) {
    let _ = Command::new("gcloud")
        .args([
            "projects",
            "remove-iam-policy-binding",
            project_id,
            "--member",
            member,
            "--role",
            role,
            "--quiet",
        ])
        .output();
}

fn create_cloudsql_instance(
    project_id: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "  Creating CloudSQL instance: {} (this may take a few minutes)...",
        instance_name
    );
    let output = Command::new("gcloud")
        .args([
            "sql",
            "instances",
            "create",
            instance_name,
            "--project",
            project_id,
            "--database-version=POSTGRES_15",
            "--tier=db-f1-micro",
            "--region=us-central1",
            "--no-backup",
            "--quiet",
        ])
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("already exists") {
            println!("  Instance already exists, continuing...");
            return Ok(());
        }
        return Err(format!("Failed to create CloudSQL instance: {}", stderr).into());
    }
    println!("  CloudSQL instance created");
    Ok(())
}

/// Poll CloudSQL activation policy via gcloud until it matches expected value.
fn wait_for_cloudsql_state(
    project_id: &str,
    instance_name: &str,
    expected_policy: &str,
    timeout_secs: u64,
) -> bool {
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    loop {
        if start.elapsed() > timeout {
            println!("  Timeout waiting for activationPolicy={}", expected_policy);
            return false;
        }
        let output = Command::new("gcloud")
            .args([
                "sql",
                "instances",
                "describe",
                instance_name,
                "--project",
                project_id,
                "--format",
                "value(settings.activationPolicy)",
                "--quiet",
            ])
            .output();
        if let Ok(output) = output {
            if !output.status.success() {
                std::thread::sleep(Duration::from_secs(10));
                continue;
            }
            let policy = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  Current activationPolicy: {}", policy);
            if policy == expected_policy {
                return true;
            }
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}

/// Poll CloudSQL instance run state via gcloud.
fn wait_for_cloudsql_run_state(
    project_id: &str,
    instance_name: &str,
    expected_state: &str,
    timeout_secs: u64,
) -> bool {
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    loop {
        if start.elapsed() > timeout {
            println!("  Timeout waiting for state={}", expected_state);
            return false;
        }
        let output = Command::new("gcloud")
            .args([
                "sql",
                "instances",
                "describe",
                instance_name,
                "--project",
                project_id,
                "--format",
                "value(state)",
                "--quiet",
            ])
            .output();
        if let Ok(output) = output {
            if !output.status.success() {
                std::thread::sleep(Duration::from_secs(10));
                continue;
            }
            let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  Current instance state: {}", state);
            if state == expected_state {
                return true;
            }
        }
        std::thread::sleep(Duration::from_secs(15));
    }
}

fn cleanup_all(project_id: &str) {
    println!("\nCleaning up test resources...");
    gcloud_delete_job(project_id, TEST_LOCATION, TEST_JOB_ID);
    gcloud_delete_job(project_id, TEST_LOCATION, TEST_JOB_ID_2);
    println!("Cleanup complete");
}

fn cleanup_e2e(project_id: &str) {
    println!("\nCleaning up E2E test resources...");
    gcloud_delete_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID);
    gcloud_delete_job(project_id, TEST_LOCATION, E2E_START_JOB_ID);
    cleanup_iam_binding(project_id, &e2e_sa_member(project_id), E2E_SA_ROLE);
    cleanup_sa(project_id);
    cleanup_cloudsql_instance(project_id, E2E_INSTANCE_NAME);
    println!("E2E cleanup complete");
}

// ── Test: Full Lifecycle with gcloud cross-verification ──────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_scheduler_full_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Cloud Scheduler Full Lifecycle Test ===");
    println!("Project: {}", project_id);
    println!("Location: {}", TEST_LOCATION);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    cleanup_all(&project_id);
    tokio::time::sleep(Duration::from_secs(10)).await;

    let result = run_full_lifecycle_tests(&client, &project_id).await;
    cleanup_all(&project_id);

    result?;
    println!("\nAll scheduler full lifecycle tests passed!");
    Ok(())
}

async fn run_full_lifecycle_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = client.scheduler();
    let job_name = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, TEST_JOB_ID
    );

    // === 1. CREATE - job with HTTP target, retry config, attempt deadline ===
    println!("\n[1/9] Creating scheduler job with HTTP target...");
    let job = Job {
        name: job_name.clone(),
        schedule: Some("0 9 * * 1".to_string()),
        time_zone: Some("America/New_York".to_string()),
        description: Some("Integration test job".to_string()),
        http_target: Some(HttpTarget {
            uri: Some("https://httpbin.org/post".to_string()),
            http_method: Some(HttpMethod::Post),
            ..Default::default()
        }),
        retry_config: Some(RetryConfig {
            retry_count: Some(1),
            max_retry_duration: Some("10s".to_string()),
            ..Default::default()
        }),
        attempt_deadline: Some("30s".to_string()),
        ..Default::default()
    };

    let created = scheduler
        .create_job(project_id, TEST_LOCATION, &job)
        .await?;
    assert!(
        created.name.ends_with(TEST_JOB_ID),
        "Created job name should end with job ID"
    );
    assert_eq!(created.schedule, Some("0 9 * * 1".to_string()));
    assert_eq!(created.time_zone, Some("America/New_York".to_string()));
    assert_eq!(
        created.state,
        Some(JobState::Enabled),
        "New job should be ENABLED, got: {:?}",
        created.state
    );
    println!("  Created: {}", created.name);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // === 2. GET - verify all fields ===
    println!("\n[2/9] Getting scheduler job details...");
    let fetched = scheduler
        .get_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    assert_eq!(
        fetched.description,
        Some("Integration test job".to_string())
    );
    assert_eq!(fetched.attempt_deadline, Some("30s".to_string()));

    let target = fetched
        .http_target
        .as_ref()
        .expect("Should have http_target");
    assert_eq!(target.uri, Some("https://httpbin.org/post".to_string()));
    assert_eq!(target.http_method, Some(HttpMethod::Post));

    let retry = fetched
        .retry_config
        .as_ref()
        .expect("Should have retry_config");
    assert_eq!(retry.retry_count, Some(1));
    println!("  Verified all fields match");

    // === 3. CREATE second job (for list test) ===
    println!("\n[3/9] Creating second scheduler job...");
    let job_name_2 = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, TEST_JOB_ID_2
    );
    let job2 = Job {
        name: job_name_2.clone(),
        schedule: Some("*/30 * * * *".to_string()),
        time_zone: Some("UTC".to_string()),
        description: Some("Integration test job 2".to_string()),
        http_target: Some(HttpTarget {
            uri: Some("https://httpbin.org/get".to_string()),
            http_method: Some(HttpMethod::Get),
            ..Default::default()
        }),
        ..Default::default()
    };

    let created2 = scheduler
        .create_job(project_id, TEST_LOCATION, &job2)
        .await?;
    assert!(created2.name.ends_with(TEST_JOB_ID_2));
    println!("  Created: {}", created2.name);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // === 4. LIST - should contain both jobs ===
    println!("\n[4/9] Listing scheduler jobs...");
    let jobs = scheduler.list_jobs(project_id, TEST_LOCATION).await?;
    let found_1 = jobs.iter().any(|j| j.name.contains(TEST_JOB_ID));
    let found_2 = jobs.iter().any(|j| j.name.contains(TEST_JOB_ID_2));
    assert!(found_1, "First test job should be in list");
    assert!(found_2, "Second test job should be in list");
    println!("  Found both jobs in list ({} total)", jobs.len());

    // === 5. PAUSE - pause first job ===
    println!("\n[5/9] Pausing scheduler job...");
    let paused = scheduler
        .pause_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    assert_eq!(
        paused.state,
        Some(JobState::Paused),
        "Job should be PAUSED after pause_job"
    );
    println!("  Job state: {:?}", paused.state);

    // Verify via gcloud
    let state = get_job_state(project_id, TEST_LOCATION, TEST_JOB_ID);
    assert_eq!(state, Some("PAUSED".to_string()));
    println!("  Verified via gcloud: PAUSED");

    tokio::time::sleep(Duration::from_secs(5)).await;

    // === 6. RESUME - resume the paused job ===
    println!("\n[6/9] Resuming scheduler job...");
    let resumed = scheduler
        .resume_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    assert_eq!(
        resumed.state,
        Some(JobState::Enabled),
        "Job should be ENABLED after resume_job"
    );
    println!("  Job state: {:?}", resumed.state);

    // Verify via gcloud
    let state = get_job_state(project_id, TEST_LOCATION, TEST_JOB_ID);
    assert_eq!(state, Some("ENABLED".to_string()));
    println!("  Verified via gcloud: ENABLED");

    // === 7. DELETE first job ===
    println!("\n[7/9] Deleting first scheduler job...");
    scheduler
        .delete_job(project_id, TEST_LOCATION, TEST_JOB_ID)
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        !job_exists(project_id, TEST_LOCATION, TEST_JOB_ID),
        "Job should not exist after deletion"
    );
    println!("  Verified: job deleted");

    // === 8. DELETE second job ===
    println!("\n[8/9] Deleting second scheduler job...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    scheduler
        .delete_job(project_id, TEST_LOCATION, TEST_JOB_ID_2)
        .await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    assert!(
        !job_exists(project_id, TEST_LOCATION, TEST_JOB_ID_2),
        "Second job should not exist after deletion"
    );
    println!("  Verified: second job deleted");

    // === 9. GET non-existent - expect NotFound ===
    println!("\n[9/9] Getting non-existent job (expect NotFound)...");
    let result = scheduler
        .get_job(project_id, TEST_LOCATION, "nonexistent-job-does-not-exist")
        .await;
    match result {
        Err(gcp_lite::GcpError::NotFound { .. }) => {
            println!("  Correctly received NotFound error");
        }
        Err(e) => {
            println!("  Received error (expected NotFound): {:?}", e);
        }
        Ok(_) => {
            return Err("Expected error when getting non-existent job".into());
        }
    }

    Ok(())
}

// ── Test: E2E CloudSQL Stop/Start via Scheduler + IAM ────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID, ADC credentials, and creates real CloudSQL instance"]
async fn test_scheduler_cloudsql_e2e() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== E2E: CloudSQL Stop/Start Scheduler + IAM Test ===");
    println!("Project: {}", project_id);
    println!("Location: {}", TEST_LOCATION);

    let client = GcpHttpClient::from_adc().await?;

    // Enable required APIs
    println!("\n[0/14] Enabling required APIs...");
    for api in [
        "cloudscheduler.googleapis.com",
        "sqladmin.googleapis.com",
        "iam.googleapis.com",
    ] {
        let enabled = client
            .service_usage()
            .is_service_enabled(&project_id, api)
            .await?;
        if !enabled {
            println!("  Enabling {}...", api);
            client
                .service_usage()
                .enable_service(&project_id, api)
                .await?;
        } else {
            println!("  {} already enabled", api);
        }
    }

    // Pre-cleanup
    cleanup_e2e(&project_id);
    tokio::time::sleep(Duration::from_secs(3)).await;

    let result = run_e2e_tests(&client, &project_id).await;
    cleanup_e2e(&project_id);

    result?;
    println!("\nAll E2E CloudSQL scheduler tests passed!");
    Ok(())
}

async fn run_e2e_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = client.scheduler();
    let sa_email = e2e_sa_email(project_id);
    let sa_member = e2e_sa_member(project_id);

    // === 1. Create minimal CloudSQL instance ===
    println!("\n[1/14] Creating minimal CloudSQL instance...");
    create_cloudsql_instance(project_id, E2E_INSTANCE_NAME)?;

    // === 2. Create service account via our IAM client ===
    println!("\n[2/14] Creating service account via IAM client...");
    let sa = client
        .iam()
        .create_service_account(
            project_id,
            E2E_SA_NAME,
            "Scheduler CloudSQL Test SA",
            "SA for scheduler integration test - stop/start CloudSQL",
        )
        .await?;
    assert_eq!(sa.email, Some(sa_email.clone()));
    println!("  Created SA: {}", sa.email.as_ref().unwrap());

    tokio::time::sleep(Duration::from_secs(5)).await;

    // === 3. Grant SA roles/cloudsql.admin via projects client ===
    println!(
        "\n[3/14] Granting {} to SA via add_iam_policy_binding...",
        E2E_SA_ROLE
    );
    let policy = client
        .projects()
        .add_iam_policy_binding(project_id, E2E_SA_ROLE, &sa_member)
        .await?;
    let has_binding = policy
        .bindings
        .iter()
        .any(|b| b.role.as_deref() == Some(E2E_SA_ROLE) && b.members.contains(&sa_member));
    assert!(has_binding, "SA should have {} role", E2E_SA_ROLE);
    println!("  Granted {} to {}", E2E_SA_ROLE, sa_email);

    // Grant Cloud Scheduler service agent permission to impersonate our SA
    println!("  Granting Scheduler service agent permission to impersonate SA...");
    if let Some(ref num) = get_project_number(project_id) {
        let scheduler_agent = format!(
            "serviceAccount:service-{}@gcp-sa-cloudscheduler.iam.gserviceaccount.com",
            num
        );
        let output = Command::new("gcloud")
            .args([
                "iam",
                "service-accounts",
                "add-iam-policy-binding",
                &sa_email,
                "--project",
                project_id,
                "--member",
                &scheduler_agent,
                "--role",
                "roles/iam.serviceAccountUser",
                "--quiet",
            ])
            .output();
        if let Ok(o) = &output {
            if o.status.success() {
                println!("  Granted roles/iam.serviceAccountUser to Scheduler agent");
            } else {
                let stderr = String::from_utf8_lossy(&o.stderr);
                println!("  Warning: could not grant SA User role: {}", stderr.trim());
            }
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    } else {
        println!("  Warning: could not determine project number, OAuth may fail");
    }

    // === 4. Create "stop" scheduler job ===
    println!("\n[4/14] Creating 'stop' scheduler job with OAuth token...");
    let instance_uri = format!(
        "https://sqladmin.googleapis.com/v1/projects/{}/instances/{}",
        project_id, E2E_INSTANCE_NAME
    );
    let stop_body = serde_json::json!({"settings": {"activationPolicy": "NEVER"}});
    let stop_body_b64 =
        base64::engine::general_purpose::STANDARD.encode(stop_body.to_string().as_bytes());

    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "application/json; charset=utf-8".to_string(),
    );

    let stop_job_name = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, E2E_STOP_JOB_ID
    );
    let stop_job = Job {
        name: stop_job_name,
        schedule: Some("0 18 * * 1-5".to_string()),
        time_zone: Some("America/New_York".to_string()),
        description: Some("Stop dev CloudSQL instance weekdays at 6pm ET".to_string()),
        http_target: Some(HttpTarget {
            uri: Some(instance_uri.clone()),
            http_method: Some(HttpMethod::Patch),
            headers: headers.clone(),
            body: Some(stop_body_b64),
            oauth_token: Some(OAuthToken {
                service_account_email: Some(sa_email.clone()),
                scope: Some("https://www.googleapis.com/auth/cloud-platform".to_string()),
            }),
            ..Default::default()
        }),
        retry_config: Some(RetryConfig {
            retry_count: Some(3),
            ..Default::default()
        }),
        attempt_deadline: Some("60s".to_string()),
        ..Default::default()
    };

    let created_stop = scheduler
        .create_job(project_id, TEST_LOCATION, &stop_job)
        .await?;
    assert!(created_stop.name.ends_with(E2E_STOP_JOB_ID));
    assert_eq!(created_stop.state, Some(JobState::Enabled));
    println!("  Created stop job: {}", created_stop.name);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // === 5. Create "start" scheduler job ===
    println!("\n[5/14] Creating 'start' scheduler job with OAuth token...");
    let start_body = serde_json::json!({"settings": {"activationPolicy": "ALWAYS"}});
    let start_body_b64 =
        base64::engine::general_purpose::STANDARD.encode(start_body.to_string().as_bytes());

    let start_job_name = format!(
        "projects/{}/locations/{}/jobs/{}",
        project_id, TEST_LOCATION, E2E_START_JOB_ID
    );
    let start_job = Job {
        name: start_job_name,
        schedule: Some("0 8 * * 1-5".to_string()),
        time_zone: Some("America/New_York".to_string()),
        description: Some("Start dev CloudSQL instance weekdays at 8am ET".to_string()),
        http_target: Some(HttpTarget {
            uri: Some(instance_uri.clone()),
            http_method: Some(HttpMethod::Patch),
            headers: headers.clone(),
            body: Some(start_body_b64),
            oauth_token: Some(OAuthToken {
                service_account_email: Some(sa_email.clone()),
                scope: Some("https://www.googleapis.com/auth/cloud-platform".to_string()),
            }),
            ..Default::default()
        }),
        retry_config: Some(RetryConfig {
            retry_count: Some(3),
            ..Default::default()
        }),
        attempt_deadline: Some("60s".to_string()),
        ..Default::default()
    };

    let created_start = scheduler
        .create_job(project_id, TEST_LOCATION, &start_job)
        .await?;
    assert!(created_start.name.ends_with(E2E_START_JOB_ID));
    assert_eq!(created_start.state, Some(JobState::Enabled));
    println!("  Created start job: {}", created_start.name);

    tokio::time::sleep(Duration::from_secs(3)).await;

    // === 6. Verify stop job configuration ===
    println!("\n[6/14] Verifying stop job configuration...");
    let fetched_stop = scheduler
        .get_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID)
        .await?;
    let stop_target = fetched_stop
        .http_target
        .as_ref()
        .expect("Should have http_target");
    assert_eq!(
        stop_target.uri.as_deref(),
        Some(instance_uri.as_str()),
        "Stop job URI should target CloudSQL instance"
    );
    assert_eq!(
        stop_target.http_method,
        Some(HttpMethod::Patch),
        "Stop job should use PATCH"
    );
    let stop_oauth = stop_target
        .oauth_token
        .as_ref()
        .expect("Should have oauth_token");
    assert_eq!(
        stop_oauth.service_account_email.as_deref(),
        Some(sa_email.as_str()),
        "OAuth SA email should match created SA"
    );
    assert!(
        stop_target.body.is_some(),
        "Stop job should have a body (activationPolicy: NEVER)"
    );
    println!("  Stop job: URI={}", stop_target.uri.as_deref().unwrap());
    println!(
        "  Stop job: OAuth SA={}",
        stop_oauth.service_account_email.as_deref().unwrap()
    );
    println!("  Stop job: method={:?}", stop_target.http_method);

    // === 7. Verify start job configuration ===
    println!("\n[7/14] Verifying start job configuration...");
    let fetched_start = scheduler
        .get_job(project_id, TEST_LOCATION, E2E_START_JOB_ID)
        .await?;
    let start_target = fetched_start
        .http_target
        .as_ref()
        .expect("Should have http_target");
    assert_eq!(
        start_target.uri.as_deref(),
        Some(instance_uri.as_str()),
        "Start job URI should target same instance"
    );
    assert_eq!(
        start_target.http_method,
        Some(HttpMethod::Patch),
        "Start job should use PATCH"
    );
    let start_oauth = start_target
        .oauth_token
        .as_ref()
        .expect("Should have oauth_token");
    assert_eq!(
        start_oauth.service_account_email.as_deref(),
        Some(sa_email.as_str()),
        "OAuth SA email should match created SA"
    );
    assert!(
        start_target.body.is_some(),
        "Start job should have a body (activationPolicy: ALWAYS)"
    );
    println!("  Start job: URI={}", start_target.uri.as_deref().unwrap());
    println!(
        "  Start job: OAuth SA={}",
        start_oauth.service_account_email.as_deref().unwrap()
    );

    // === 8. LIST - both E2E jobs should appear ===
    println!("\n[8/14] Listing jobs - verifying both E2E jobs present...");
    let jobs = scheduler.list_jobs(project_id, TEST_LOCATION).await?;
    let found_stop = jobs.iter().any(|j| j.name.contains(E2E_STOP_JOB_ID));
    let found_start = jobs.iter().any(|j| j.name.contains(E2E_START_JOB_ID));
    assert!(found_stop, "Stop job should appear in list");
    assert!(found_start, "Start job should appear in list");
    println!("  Both E2E jobs found in list ({} total jobs)", jobs.len());

    // === 9. FORCE RUN stop job ===
    println!("\n[9/14] Force-running stop job (triggering CloudSQL shutdown)...");
    let run_result = scheduler
        .run_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID)
        .await?;
    println!("  Run triggered, job state: {:?}", run_result.state);

    println!("  Waiting for CloudSQL to process stop request (up to 120s)...");
    let stopped = wait_for_cloudsql_state(project_id, E2E_INSTANCE_NAME, "NEVER", 120);
    assert!(
        stopped,
        "CloudSQL instance activationPolicy should become NEVER after stop job runs"
    );
    println!("  CloudSQL instance stopped (activationPolicy=NEVER)");

    println!("  Waiting for CloudSQL instance to fully stop (up to 300s)...");
    let fully_stopped = wait_for_cloudsql_run_state(project_id, E2E_INSTANCE_NAME, "STOPPED", 300);
    if !fully_stopped {
        println!("  Warning: instance didn't reach STOPPED state, trying to start anyway");
    } else {
        println!("  CloudSQL instance fully STOPPED");
    }

    // === 10. FORCE RUN start job ===
    println!("\n[10/14] Force-running start job (triggering CloudSQL startup)...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let run_result = scheduler
        .run_job(project_id, TEST_LOCATION, E2E_START_JOB_ID)
        .await?;
    println!("  Run triggered, job state: {:?}", run_result.state);

    println!("  Waiting for CloudSQL to process start request (up to 300s)...");
    let started = wait_for_cloudsql_state(project_id, E2E_INSTANCE_NAME, "ALWAYS", 300);
    assert!(
        started,
        "CloudSQL instance activationPolicy should become ALWAYS after start job runs"
    );
    println!("  CloudSQL instance started (activationPolicy=ALWAYS)");

    // === 11. PAUSE stop job, verify, RESUME ===
    println!("\n[11/14] Pausing stop job, then resuming...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let paused = scheduler
        .pause_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID)
        .await?;
    assert_eq!(paused.state, Some(JobState::Paused));
    println!("  Stop job PAUSED");

    tokio::time::sleep(Duration::from_secs(5)).await;
    let resumed = scheduler
        .resume_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID)
        .await?;
    assert_eq!(resumed.state, Some(JobState::Enabled));
    println!("  Stop job RESUMED -> ENABLED");

    // === 12. DELETE stop job ===
    println!("\n[12/14] Deleting stop scheduler job...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    scheduler
        .delete_job(project_id, TEST_LOCATION, E2E_STOP_JOB_ID)
        .await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    assert!(
        !job_exists(project_id, TEST_LOCATION, E2E_STOP_JOB_ID),
        "Stop job should be deleted"
    );
    println!("  Stop job deleted");

    // === 13. DELETE start job ===
    println!("\n[13/14] Deleting start scheduler job...");
    scheduler
        .delete_job(project_id, TEST_LOCATION, E2E_START_JOB_ID)
        .await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    assert!(
        !job_exists(project_id, TEST_LOCATION, E2E_START_JOB_ID),
        "Start job should be deleted"
    );
    println!("  Start job deleted");

    // === 14. Final verification ===
    println!("\n[14/14] Final verification...");
    assert!(
        !job_exists(project_id, TEST_LOCATION, E2E_STOP_JOB_ID),
        "Stop job should not exist"
    );
    assert!(
        !job_exists(project_id, TEST_LOCATION, E2E_START_JOB_ID),
        "Start job should not exist"
    );
    println!("  All E2E jobs confirmed deleted");

    Ok(())
}
