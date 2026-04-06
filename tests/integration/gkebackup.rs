//! Integration tests for the GKE Backup API.
//!
//! Creates an Autopilot GKE cluster, tests backup plan CRUD, then deletes
//! everything. All resources are auto-cleaned via gcloud on success, failure,
//! or panic.
//!
//! Run with:
//!   GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!     cargo test --test integration gkebackup -- --ignored --test-threads=1 --nocapture

use gcp_lite::GcpHttpClient;
use std::env;
use std::process::Command;

const CLUSTER_NAME: &str = "gcp-lite-test-gkebackup";
const PLAN_ID: &str = "gcp-lite-test-gkebackup-plan";
const LOCATION: &str = "us-central1";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

/// Create an Autopilot cluster via gcloud. Blocks until RUNNING.
fn gcloud_create_cluster(project: &str, name: &str, location: &str) {
    println!(
        "  Creating cluster '{}' via gcloud (this takes ~8 min)...",
        name
    );
    let output = Command::new("gcloud")
        .args([
            "container",
            "clusters",
            "create-auto",
            name,
            "--project",
            project,
            "--region",
            location,
            "--quiet",
        ])
        .output()
        .expect("gcloud must be installed");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // If cluster already exists, that's fine
        if stderr.contains("Already exists") || stderr.contains("ALREADY_EXISTS") {
            println!("  Cluster already exists, reusing");
        } else {
            panic!("gcloud cluster create failed: {}", stderr);
        }
    } else {
        println!("  Cluster created");
    }
}

/// Ensure a cluster is fully deleted. Kicks off delete if needed, then polls until gone.
fn ensure_cluster_deleted(project: &str, name: &str, location: &str) {
    // First try to kick off a delete (may fail if already deleting — that's fine)
    let _ = Command::new("gcloud")
        .args([
            "container",
            "clusters",
            "delete",
            name,
            "--project",
            project,
            "--region",
            location,
            "--async",
            "--quiet",
        ])
        .output();

    // Poll until the cluster is gone
    println!("  Waiting for cluster '{}' to be fully deleted...", name);
    for i in 0..60 {
        // 60 * 15s = 15 min max wait
        let output = Command::new("gcloud")
            .args([
                "container",
                "clusters",
                "describe",
                name,
                "--project",
                project,
                "--region",
                location,
                "--format=value(status)",
            ])
            .output()
            .expect("gcloud must be installed");

        if !output.status.success() {
            // Cluster no longer exists
            println!("  Cluster is gone");
            return;
        }

        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if i % 4 == 0 {
            println!("  Cluster still exists (status={}), waiting...", status);
        }
        std::thread::sleep(std::time::Duration::from_secs(15));
    }
    println!("  Warning: timed out waiting for cluster deletion, proceeding anyway");
}

/// Delete a cluster via gcloud (async — fire-and-forget, for final cleanup).
fn gcloud_delete_cluster_async(project: &str, name: &str, location: &str) {
    println!("  Deleting cluster '{}' via gcloud (async)...", name);
    let _ = Command::new("gcloud")
        .args([
            "container",
            "clusters",
            "delete",
            name,
            "--project",
            project,
            "--region",
            location,
            "--async",
            "--quiet",
        ])
        .output();
}

/// Delete a backup plan via gcloud (idempotent — ignores errors).
fn gcloud_delete_backup_plan(project: &str, location: &str, plan_id: &str) {
    let name = format!(
        "projects/{}/locations/{}/backupPlans/{}",
        project, location, plan_id
    );
    println!("  Deleting backup plan '{}' via gcloud...", plan_id);
    let _ = Command::new("gcloud")
        .args([
            "beta",
            "container",
            "backup-restore",
            "backup-plans",
            "delete",
            &name,
            "--project",
            project,
            "--quiet",
        ])
        .output();
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_gkebackup_plan_lifecycle() {
    let project = project_id();

    println!("\n=== GKE Backup API: BackupPlan Lifecycle ===");
    println!("Project: {}", project);
    println!("Cluster: {}", CLUSTER_NAME);
    println!("Plan: {}", PLAN_ID);
    println!("Location: {}", LOCATION);

    // Pre-cleanup: delete leftover resources from previous failed runs
    gcloud_delete_backup_plan(&project, LOCATION, PLAN_ID);
    ensure_cluster_deleted(&project, CLUSTER_NAME, LOCATION);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    // Ensure APIs are enabled
    println!("\n[0] Ensuring APIs are enabled...");
    let su = client.service_usage();
    for api in &["container.googleapis.com", "gkebackup.googleapis.com"] {
        let enabled = su
            .is_service_enabled(&project, api)
            .await
            .expect("service usage check failed");
        if !enabled {
            println!("  Enabling {}...", api);
            su.enable_service(&project, api)
                .await
                .expect("enable API failed");
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        } else {
            println!("  {} already enabled", api);
        }
    }

    // Create cluster via gcloud (blocking — needed for backup plans)
    println!("\n[1] Creating test cluster...");
    gcloud_create_cluster(&project, CLUSTER_NAME, LOCATION);

    // Run actual tests, capturing result
    let result = run_gkebackup_tests(&client, &project).await;

    // Always cleanup — delete backup plan first, then cluster (async for final)
    println!("\nCleaning up...");
    gcloud_delete_backup_plan(&project, LOCATION, PLAN_ID);
    gcloud_delete_cluster_async(&project, CLUSTER_NAME, LOCATION);

    result.expect("GKE Backup lifecycle tests failed");
    println!("\n=== GKE Backup tests passed ===");
}

async fn run_gkebackup_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let backup = client.gkebackup();

    let cluster_resource = format!(
        "projects/{}/locations/{}/clusters/{}",
        project, LOCATION, CLUSTER_NAME
    );

    // === LIST (before create) ===
    println!("\n[2] Listing backup plans (before create)...");
    let plans = backup.list_backup_plans(project, LOCATION).await?;
    println!("  Found {} plan(s)", plans.len());
    let had_test_plan = plans
        .iter()
        .any(|p| p.name.ends_with(&format!("/{}", PLAN_ID)));
    assert!(
        !had_test_plan,
        "Test plan should not exist after pre-cleanup"
    );

    // === CREATE (LRO — with extended timeout for fresh clusters) ===
    println!(
        "\n[3] Creating backup plan '{}' (first-time on fresh cluster can be slow)...",
        PLAN_ID
    );
    let plan_body = gcp_lite::types::gkebackup::BackupPlan {
        cluster: Some(cluster_resource),
        description: Some("gcp-lite integration test plan".to_string()),
        backup_config: Some(gcp_lite::types::gkebackup::BackupConfig {
            all_namespaces: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };
    // Fresh Autopilot clusters need backup agent installation, which can take >10 min
    backup
        .create_backup_plan_start(project, LOCATION, PLAN_ID, &plan_body)
        .await?
        .with_timeout(std::time::Duration::from_secs(900))
        .wait()
        .await?;
    println!("  Create LRO completed");

    // === GET ===
    println!("\n[4] Getting backup plan...");
    let fetched = backup.get_backup_plan(project, LOCATION, PLAN_ID).await?;
    assert!(fetched.name.ends_with(PLAN_ID));
    assert!(fetched.cluster.is_some(), "Should have cluster reference");
    assert!(fetched.description.is_some(), "Should have description");
    println!("  Name: {}", fetched.name);
    println!("  State: {:?}", fetched.state);
    println!("  Cluster: {:?}", fetched.cluster);
    println!("  Description: {:?}", fetched.description);

    // === LIST (after create) ===
    println!("\n[5] Listing backup plans (after create)...");
    let plans = backup.list_backup_plans(project, LOCATION).await?;
    let found = plans
        .iter()
        .any(|p| p.name.ends_with(&format!("/{}", PLAN_ID)));
    assert!(found, "Test plan should appear in list");
    println!(
        "  Found {} plan(s), test plan present: {}",
        plans.len(),
        found
    );

    // === DELETE (LRO — blocking) ===
    println!("\n[6] Deleting backup plan via library (LRO)...");
    backup
        .delete_backup_plan(project, LOCATION, PLAN_ID)
        .await?;
    println!("  Delete LRO completed");

    // === VERIFY DELETED ===
    println!("\n[7] Verifying backup plan deleted (expect 404)...");
    let result = backup.get_backup_plan(project, LOCATION, PLAN_ID).await;
    assert!(
        result.is_err(),
        "Deleted backup plan should return error on get"
    );
    println!("  Confirmed: get returns error after deletion");

    // === ERROR: Non-existent backup plan ===
    println!("\n[8] Getting non-existent backup plan (expect error)...");
    let result = backup
        .get_backup_plan(project, LOCATION, "nonexistent-plan-xyz-99999")
        .await;
    assert!(
        result.is_err(),
        "Non-existent backup plan should return error"
    );
    println!("  Confirmed: non-existent backup plan returns error");

    Ok(())
}
