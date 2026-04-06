//! Integration tests for the Kubernetes Engine (Container) API.
//!
//! Creates an Autopilot GKE cluster, tests list/get, then deletes it
//! using the library's LRO delete. All resources are auto-cleaned via
//! gcloud on success, failure, or panic.
//!
//! Run with:
//!   GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!     cargo test --test integration container -- --ignored --test-threads=1 --nocapture

use gcp_lite::GcpHttpClient;
use std::env;
use std::process::Command;

const CLUSTER_NAME: &str = "gcp-lite-test-container";
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

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_container_cluster_lifecycle() {
    let project = project_id();

    println!("\n=== Container API: Cluster Lifecycle ===");
    println!("Project: {}", project);
    println!("Cluster: {}", CLUSTER_NAME);
    println!("Location: {}", LOCATION);

    // Pre-cleanup: delete leftover cluster from previous failed run
    ensure_cluster_deleted(&project, CLUSTER_NAME, LOCATION);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    // Ensure API is enabled
    println!("\n[0] Ensuring Container API is enabled...");
    let su = client.service_usage();
    let enabled = su
        .is_service_enabled(&project, "container.googleapis.com")
        .await
        .expect("service usage check failed");
    if !enabled {
        println!("  Enabling...");
        su.enable_service(&project, "container.googleapis.com")
            .await
            .expect("enable API failed");
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    } else {
        println!("  Already enabled");
    }

    // Create cluster via gcloud (blocking)
    println!("\n[1] Creating test cluster...");
    gcloud_create_cluster(&project, CLUSTER_NAME, LOCATION);

    // Run actual tests, capturing result
    let result = run_container_tests(&client, &project).await;

    // Always cleanup — delete the cluster (async since we don't need to wait)
    println!("\nCleaning up...");
    gcloud_delete_cluster_async(&project, CLUSTER_NAME, LOCATION);

    result.expect("Container lifecycle tests failed");
    println!("\n=== Container tests passed ===");
}

async fn run_container_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let container = client.container();

    // === LIST (all locations) ===
    println!("\n[2] Listing clusters (all locations)...");
    let clusters = container.list_clusters(project, "-").await?;
    println!("  Found {} cluster(s)", clusters.len());
    assert!(
        !clusters.is_empty(),
        "Should have at least our test cluster"
    );

    let test_cluster = clusters
        .iter()
        .find(|c| c.name == CLUSTER_NAME)
        .expect("Test cluster should be in list");
    println!(
        "  Test cluster: status={:?}, version={:?}, nodes={:?}",
        test_cluster.status, test_cluster.current_master_version, test_cluster.current_node_count
    );

    // === LIST (specific location) ===
    println!("\n[3] Listing clusters (us-central1 only)...");
    let regional = container.list_clusters(project, LOCATION).await?;
    assert!(
        regional.iter().any(|c| c.name == CLUSTER_NAME),
        "Test cluster should appear in regional list"
    );
    println!("  Found {} cluster(s) in {}", regional.len(), LOCATION);

    // === GET ===
    println!("\n[4] Getting cluster details...");
    let fetched = container
        .get_cluster(project, LOCATION, CLUSTER_NAME)
        .await?;
    assert_eq!(fetched.name, CLUSTER_NAME);
    assert_eq!(fetched.location.as_deref(), Some(LOCATION));
    assert!(
        fetched.endpoint.is_some(),
        "Running cluster should have endpoint"
    );
    assert!(fetched.self_link.is_some(), "Should have selfLink");
    assert!(fetched.create_time.is_some(), "Should have createTime");
    println!("  Name: {}", fetched.name);
    println!("  Location: {:?}", fetched.location);
    println!("  Status: {:?}", fetched.status);
    println!("  Endpoint: {:?}", fetched.endpoint);
    println!("  Version: {:?}", fetched.current_master_version);
    println!("  Nodes: {:?}", fetched.current_node_count);
    println!("  Created: {:?}", fetched.create_time);
    println!("  Network: {:?}", fetched.network);

    // === DELETE (LRO — blocking) ===
    println!("\n[5] Deleting cluster via library (LRO)...");
    container
        .delete_cluster(project, LOCATION, CLUSTER_NAME)
        .await?;
    println!("  Delete LRO completed");

    // === VERIFY DELETED ===
    println!("\n[6] Verifying cluster deleted (expect 404)...");
    let result = container.get_cluster(project, LOCATION, CLUSTER_NAME).await;
    assert!(
        result.is_err(),
        "Deleted cluster should return error on get"
    );
    println!("  Confirmed: get returns error after deletion");

    // === ERROR: Non-existent cluster ===
    println!("\n[7] Getting non-existent cluster (expect error)...");
    let result = container
        .get_cluster(project, LOCATION, "nonexistent-cluster-xyz-99999")
        .await;
    assert!(result.is_err(), "Non-existent cluster should return error");
    println!("  Confirmed: non-existent cluster returns error");

    Ok(())
}
