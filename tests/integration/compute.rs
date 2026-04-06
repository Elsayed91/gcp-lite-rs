//! Integration tests for Compute Engine operations.
//!
//! These tests require:
//! - `GCLOUD_PROJECT_ID` environment variable
//! - ADC credentials configured (`GOOGLE_AUTH_USE_GCLOUD=1` or service account)
//! - Sufficient permissions (`roles/compute.admin`)
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration compute -- --ignored --test-threads=1 --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::compute::*;
use std::env;
use std::process::Command;

const TEST_ZONE: &str = "us-central1-a";
const TEST_REGION: &str = "us-central1";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

fn gcloud(args: &[&str]) -> bool {
    let output = Command::new("gcloud")
        .args(args)
        .output()
        .expect("gcloud must be installed");
    output.status.success()
}

async fn wait_secs(n: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(n)).await;
}

// =============================================================================
// Disk Lifecycle + Resize + Snapshot
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_disk_lifecycle() {
    let project = project_id();
    let disk_name = "gcp-lite-test-disk-lifecycle";
    let snap_name = "gcp-lite-test-snap-lifecycle";
    let restored_name = "gcp-lite-test-disk-restored";

    // Pre-cleanup
    let _ = gcloud(&[
        "compute",
        "disks",
        "delete",
        restored_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "snapshots",
        "delete",
        snap_name,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "disks",
        "delete",
        disk_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(3).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let result = run_disk_tests(&client, &project, disk_name, snap_name, restored_name).await;

    // Cleanup
    println!("\nCleaning up...");
    let _ = gcloud(&[
        "compute",
        "disks",
        "delete",
        restored_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "snapshots",
        "delete",
        snap_name,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "disks",
        "delete",
        disk_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);

    result.expect("disk lifecycle tests failed");
    println!("\nAll disk lifecycle tests passed!");
}

async fn run_disk_tests(
    client: &GcpHttpClient,
    project: &str,
    disk_name: &str,
    snap_name: &str,
    restored_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let compute = client.compute();

    // === 1. CREATE DISK ===
    println!("\n[1/9] Creating disk...");
    compute
        .create_disk(
            project,
            TEST_ZONE,
            &Disk {
                name: disk_name.to_string(),
                size_gb: Some("10".to_string()),
                ..Default::default()
            },
        )
        .await?;
    println!("  Created: {disk_name}");

    // === 2. GET DISK ===
    println!("\n[2/9] Getting disk...");
    let disk = compute.get_disk(project, TEST_ZONE, disk_name).await?;
    assert_eq!(disk.name, disk_name);
    assert_eq!(disk.size_gb.as_deref(), Some("10"));
    println!("  Name: {}, Size: {:?} GB", disk.name, disk.size_gb);

    // === 3. LIST DISKS ===
    println!("\n[3/9] Listing disks...");
    let list = compute.list_disks(project, TEST_ZONE).await?;
    let found = list.items.iter().any(|d| d.name == disk_name);
    assert!(found, "Created disk should appear in list");
    println!("  Found disk in list ({} total)", list.items.len());

    // === 4. RESIZE DISK ===
    println!("\n[4/9] Resizing disk from 10 GB to 20 GB...");
    compute
        .resize_disk(project, TEST_ZONE, disk_name, 20)
        .await?;
    let resized = compute.get_disk(project, TEST_ZONE, disk_name).await?;
    assert_eq!(resized.size_gb.as_deref(), Some("20"));
    println!("  Resized to {:?} GB", resized.size_gb);

    // === 5. CREATE SNAPSHOT ===
    println!("\n[5/9] Creating snapshot from disk...");
    compute
        .create_snapshot(project, TEST_ZONE, disk_name, snap_name)
        .await?;
    println!("  Created snapshot: {snap_name}");

    // === 6. GET SNAPSHOT ===
    println!("\n[6/9] Getting snapshot...");
    let snap = compute.get_snapshot(project, snap_name).await?;
    assert_eq!(snap.name, snap_name);
    assert!(snap.source_disk.is_some());
    println!("  Name: {}, Source: {:?}", snap.name, snap.source_disk);

    // === 7. CREATE DISK FROM SNAPSHOT ===
    println!("\n[7/9] Creating disk from snapshot...");
    compute
        .create_disk_from_snapshot(project, TEST_ZONE, restored_name, snap_name, None, None)
        .await?;
    let restored = compute.get_disk(project, TEST_ZONE, restored_name).await?;
    assert_eq!(restored.name, restored_name);
    assert!(restored.source_snapshot.is_some());
    println!(
        "  Restored disk: {}, Source snapshot: {:?}",
        restored.name, restored.source_snapshot
    );

    // === 8. DELETE RESTORED DISK + SNAPSHOT + ORIGINAL DISK ===
    println!("\n[8/9] Deleting restored disk...");
    compute
        .delete_disk(project, TEST_ZONE, restored_name)
        .await?;
    println!("  Deleted restored disk");

    println!("\n  Deleting snapshot...");
    compute.delete_snapshot(project, snap_name).await?;
    println!("  Deleted snapshot");

    println!("\n  Deleting original disk...");
    compute.delete_disk(project, TEST_ZONE, disk_name).await?;
    println!("  Deleted original disk");

    // === 9. VERIFY DELETION ===
    println!("\n[9/10] Verifying deletions...");
    wait_secs(3).await;
    let result = compute.get_disk(project, TEST_ZONE, disk_name).await;
    assert!(result.is_err(), "Deleted disk should return error");
    println!("  Confirmed: disk is gone");

    // === 10. ERROR: Get non-existent disk ===
    println!("\n[10/10] Getting non-existent disk (expect error)...");
    let result = compute
        .get_disk(project, TEST_ZONE, "nonexistent-disk-xyz-12345")
        .await;
    assert!(result.is_err(), "Non-existent disk should return error");
    println!("  Confirmed: non-existent disk returns error");

    Ok(())
}

// =============================================================================
// Address Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_address_lifecycle() {
    let project = project_id();
    let addr_name = "gcp-lite-test-addr";

    // Pre-cleanup
    let _ = gcloud(&[
        "compute",
        "addresses",
        "delete",
        addr_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(3).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let result = run_address_tests(&client, &project, addr_name).await;

    // Cleanup
    println!("\nCleaning up...");
    let _ = gcloud(&[
        "compute",
        "addresses",
        "delete",
        addr_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);

    result.expect("address lifecycle tests failed");
    println!("\nAll address lifecycle tests passed!");
}

async fn run_address_tests(
    client: &GcpHttpClient,
    project: &str,
    addr_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let compute = client.compute();

    // === 1. RESERVE ADDRESS (via gcloud — we don't have reserve_address op) ===
    println!("\n[1/4] Reserving address via gcloud...");
    assert!(
        gcloud(&[
            "compute",
            "addresses",
            "create",
            addr_name,
            "--region",
            TEST_REGION,
            "--project",
            project,
        ]),
        "gcloud address creation failed"
    );
    wait_secs(3).await;
    println!("  Reserved: {addr_name}");

    // === 2. GET ADDRESS ===
    println!("\n[2/4] Getting address...");
    let addr = compute.get_address(project, TEST_REGION, addr_name).await?;
    assert_eq!(addr.name, addr_name);
    assert!(addr.address.is_some());
    println!(
        "  Name: {}, IP: {:?}, Status: {:?}",
        addr.name, addr.address, addr.status
    );

    // === 3. LIST ADDRESSES ===
    println!("\n[3/4] Listing addresses...");
    let list = compute.list_addresses(project, TEST_REGION).await?;
    let found = list.items.iter().any(|a| a.name == addr_name);
    assert!(found, "Reserved address should appear in list");
    println!("  Found address in list ({} total)", list.items.len());

    // === 4. RELEASE ADDRESS ===
    println!("\n[4/4] Releasing address...");
    compute
        .release_address(project, TEST_REGION, addr_name)
        .await?;
    println!("  Released");

    wait_secs(3).await;
    let result = compute.get_address(project, TEST_REGION, addr_name).await;
    assert!(result.is_err(), "Released address should return error");
    println!("  Confirmed: address is gone");

    Ok(())
}

// =============================================================================
// Router / NAT Gateway Lifecycle
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_router_nat_lifecycle() {
    let project = project_id();
    let network_name = "gcp-lite-test-net-nat";
    let subnet_name = "gcp-lite-test-subnet-nat";
    let router_name = "gcp-lite-test-router-nat";
    let nat_name = "gcp-lite-test-nat";

    // Pre-cleanup (reverse order of dependency)
    let _ = gcloud(&[
        "compute",
        "routers",
        "delete",
        router_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "networks",
        "subnets",
        "delete",
        subnet_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "networks",
        "delete",
        network_name,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(5).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let result = run_router_nat_tests(
        &client,
        &project,
        network_name,
        subnet_name,
        router_name,
        nat_name,
    )
    .await;

    // Cleanup
    println!("\nCleaning up...");
    let _ = gcloud(&[
        "compute",
        "routers",
        "delete",
        router_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(3).await;
    let _ = gcloud(&[
        "compute",
        "networks",
        "subnets",
        "delete",
        subnet_name,
        "--region",
        TEST_REGION,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(3).await;
    let _ = gcloud(&[
        "compute",
        "networks",
        "delete",
        network_name,
        "--project",
        &project,
        "--quiet",
    ]);

    result.expect("router/NAT lifecycle tests failed");
    println!("\nAll router/NAT lifecycle tests passed!");
}

async fn run_router_nat_tests(
    client: &GcpHttpClient,
    project: &str,
    network_name: &str,
    subnet_name: &str,
    router_name: &str,
    nat_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let compute = client.compute();

    // === 1. CREATE NETWORK + SUBNET + ROUTER via gcloud ===
    println!("\n[1/8] Creating network infrastructure via gcloud...");
    assert!(
        gcloud(&[
            "compute",
            "networks",
            "create",
            network_name,
            "--subnet-mode",
            "custom",
            "--project",
            project,
        ]),
        "gcloud network creation failed"
    );
    println!("  Created network: {network_name}");

    assert!(
        gcloud(&[
            "compute",
            "networks",
            "subnets",
            "create",
            subnet_name,
            "--network",
            network_name,
            "--region",
            TEST_REGION,
            "--range",
            "10.0.0.0/24",
            "--project",
            project,
        ]),
        "gcloud subnet creation failed"
    );
    println!("  Created subnet: {subnet_name}");

    // Create router with a description — we'll verify it survives NAT operations
    assert!(
        gcloud(&[
            "compute",
            "routers",
            "create",
            router_name,
            "--network",
            network_name,
            "--region",
            TEST_REGION,
            "--description",
            "gcp-lite non-interference test router",
            "--project",
            project,
        ]),
        "gcloud router creation failed"
    );
    println!("  Created router: {router_name}");
    wait_secs(5).await;

    // === 2. GET ROUTER ===
    println!("\n[2/8] Getting router...");
    let router = compute
        .get_router(project, TEST_REGION, router_name)
        .await?;
    assert_eq!(router.name, router_name);
    assert!(
        router.nats.is_empty(),
        "Router should have no NATs initially"
    );
    assert_eq!(
        router.description.as_deref(),
        Some("gcp-lite non-interference test router"),
        "Router should have the description we set"
    );
    println!(
        "  Name: {}, NATs: {}, Description: {:?}",
        router.name,
        router.nats.len(),
        router.description
    );

    // === 3. PATCH ROUTER — add NAT ===
    println!("\n[3/8] Adding NAT gateway via patch_router...");
    let patch_body = Router {
        nats: vec![RouterNat {
            name: nat_name.to_string(),
            source_subnetwork_ip_ranges_to_nat: Some("ALL_SUBNETWORKS_ALL_IP_RANGES".to_string()),
            nat_ip_allocate_option: Some("AUTO_ONLY".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    compute
        .patch_router(project, TEST_REGION, router_name, &patch_body)
        .await?;
    println!("  Patched router with NAT: {nat_name}");

    // === 4. VERIFY NAT EXISTS ===
    println!("\n[4/8] Verifying NAT was added...");
    let updated_router = compute
        .get_router(project, TEST_REGION, router_name)
        .await?;
    assert_eq!(updated_router.nats.len(), 1);
    assert_eq!(updated_router.nats[0].name, nat_name);
    println!(
        "  NATs: {:?}",
        updated_router
            .nats
            .iter()
            .map(|n| &n.name)
            .collect::<Vec<_>>()
    );

    // === 5. DELETE NAT GATEWAY ===
    println!("\n[5/8] Deleting NAT gateway...");
    compute
        .delete_nat_gateway(project, TEST_REGION, router_name, nat_name)
        .await?;
    println!("  Deleted NAT: {nat_name}");

    // === 6. VERIFY NAT REMOVED + NON-INTERFERENCE ===
    // Critical: delete_nat_gateway uses raw JSON round-trip. Verify it didn't
    // overwrite the router description or other fields during the round-trip.
    println!("\n[6/8] Verifying NAT removed and non-interference (description preserved)...");
    let final_router = compute
        .get_router(project, TEST_REGION, router_name)
        .await?;
    assert!(
        final_router.nats.is_empty(),
        "Router should have no NATs after deletion"
    );
    assert_eq!(
        final_router.description.as_deref(),
        Some("gcp-lite non-interference test router"),
        "Router description should survive NAT deletion (non-interference)"
    );
    println!("  NATs: {} (empty as expected)", final_router.nats.len());
    println!("  Description preserved: {:?}", final_router.description);

    // === 7. ERROR: Delete non-existent NAT ===
    println!("\n[7/8] Deleting non-existent NAT (expect error)...");
    let not_found = compute
        .delete_nat_gateway(project, TEST_REGION, router_name, "nonexistent-nat")
        .await;
    assert!(not_found.is_err(), "Deleting non-existent NAT should error");
    println!("  Confirmed: non-existent NAT delete returns error");

    // === 8. ERROR: Get non-existent router ===
    println!("\n[8/8] Getting non-existent router (expect error)...");
    let result = compute
        .get_router(project, TEST_REGION, "nonexistent-router-xyz")
        .await;
    assert!(result.is_err(), "Non-existent router should return error");
    println!("  Confirmed: non-existent router returns error");

    Ok(())
}

// =============================================================================
// Instance: Machine Type + Access Config
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_instance_extensions() {
    let project = project_id();
    let instance_name = "gcp-lite-test-inst-ext";

    // Pre-cleanup
    let _ = gcloud(&[
        "compute",
        "instances",
        "delete",
        instance_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(5).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let result = run_instance_extension_tests(&client, &project, instance_name).await;

    // Cleanup
    println!("\nCleaning up...");
    let _ = gcloud(&[
        "compute",
        "instances",
        "delete",
        instance_name,
        "--zone",
        TEST_ZONE,
        "--project",
        &project,
        "--quiet",
    ]);

    result.expect("instance extension tests failed");
    println!("\nAll instance extension tests passed!");
}

async fn run_instance_extension_tests(
    client: &GcpHttpClient,
    project: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let compute = client.compute();

    // === 1. CREATE INSTANCE via gcloud (our types don't include initializeParams) ===
    println!("\n[1/9] Creating instance via gcloud...");
    assert!(
        gcloud(&[
            "compute",
            "instances",
            "create",
            instance_name,
            "--zone",
            TEST_ZONE,
            "--machine-type",
            "e2-micro",
            "--image-family",
            "debian-12",
            "--image-project",
            "debian-cloud",
            "--boot-disk-size",
            "10GB",
            "--project",
            project,
        ]),
        "gcloud instance creation failed"
    );
    wait_secs(10).await;
    println!("  Created: {instance_name}");

    // === 2. VERIFY INSTANCE HAS EXTERNAL IP ===
    println!("\n[2/9] Verifying instance has external IP...");
    let inst = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await?;
    assert!(!inst.network_interfaces.is_empty());
    let nic = &inst.network_interfaces[0];
    assert!(
        !nic.access_configs.is_empty(),
        "Instance should have an access config (external IP)"
    );
    let external_ip = nic.access_configs[0].nat_ip.as_deref();
    println!(
        "  External IP: {:?}, NIC: {:?}",
        external_ip,
        nic.name.as_deref()
    );

    let nic_name = nic.name.as_deref().expect("NIC should have a name");
    let access_config_name = nic.access_configs[0]
        .name
        .as_deref()
        .expect("Access config should have a name");

    // === 3. REMOVE ACCESS CONFIG (external IP) ===
    println!("\n[3/9] Removing access config '{access_config_name}' (external IP)...");
    compute
        .remove_access_config(
            project,
            TEST_ZONE,
            instance_name,
            access_config_name,
            nic_name,
        )
        .await?;
    println!("  Removed access config");

    // Verify no more external IP
    let inst = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await?;
    let has_access_config = inst
        .network_interfaces
        .first()
        .map(|nic| !nic.access_configs.is_empty())
        .unwrap_or(false);
    assert!(
        !has_access_config,
        "Instance should have no access configs after removal"
    );
    println!("  Confirmed: no external IP");

    // === 4. STOP INSTANCE (required before machine type change) ===
    println!("\n[4/9] Stopping instance...");
    compute
        .stop_instance(project, TEST_ZONE, instance_name)
        .await?;
    println!("  Stopped");

    // Wait for instance to fully stop
    wait_secs(5).await;

    // === 5. CHANGE MACHINE TYPE ===
    println!("\n[5/9] Changing machine type to e2-small...");
    compute
        .set_machine_type(project, TEST_ZONE, instance_name, "e2-small")
        .await?;
    println!("  Machine type changed");

    // Verify machine type changed
    let inst = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await?;
    let mt = inst.machine_type.as_deref().unwrap_or("");
    assert!(
        mt.ends_with("/e2-small"),
        "Machine type should end with /e2-small, got: {mt}",
    );
    println!("  Confirmed: {mt}");

    // === 6. SET DISK AUTO_DELETE = FALSE (protect boot disk from deletion) ===
    println!("\n[6/9] Setting boot disk auto_delete = false...");
    let boot_disk = inst
        .disks
        .iter()
        .find(|d| d.boot == Some(true))
        .expect("Instance should have a boot disk");
    let device_name = boot_disk
        .device_name
        .as_deref()
        .expect("Boot disk should have a device_name");
    println!("  Boot disk device_name: {device_name}");
    compute
        .set_disk_auto_delete(project, TEST_ZONE, instance_name, device_name, false)
        .await?;
    // Verify the change
    let inst = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await?;
    let boot_disk = inst
        .disks
        .iter()
        .find(|d| d.boot == Some(true))
        .expect("Instance should have a boot disk");
    assert_eq!(
        boot_disk.auto_delete,
        Some(false),
        "Boot disk auto_delete should be false"
    );
    println!("  Confirmed: auto_delete = false");

    // === 7. RESTORE DISK AUTO_DELETE = TRUE ===
    println!("\n[7/9] Restoring boot disk auto_delete = true...");
    compute
        .set_disk_auto_delete(project, TEST_ZONE, instance_name, device_name, true)
        .await?;
    let inst = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await?;
    let boot_disk = inst
        .disks
        .iter()
        .find(|d| d.boot == Some(true))
        .expect("Instance should have a boot disk");
    assert_eq!(
        boot_disk.auto_delete,
        Some(true),
        "Boot disk auto_delete should be true after restore"
    );
    println!("  Confirmed: auto_delete = true (restored)");

    // === 8. START INSTANCE ===
    println!("\n[8/9] Starting instance...");
    compute
        .start_instance(project, TEST_ZONE, instance_name)
        .await?;
    println!("  Started");

    // === 9. DELETE INSTANCE ===
    println!("\n[9/9] Deleting instance...");
    compute
        .delete_instance(project, TEST_ZONE, instance_name)
        .await?;
    println!("  Deleted");

    wait_secs(5).await;
    let result = compute
        .get_instance(project, TEST_ZONE, instance_name)
        .await;
    assert!(result.is_err(), "Deleted instance should return error");
    println!("  Confirmed: instance is gone");

    Ok(())
}

// =============================================================================
// Backend Service Lifecycle (Global)
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_backend_service_lifecycle() {
    let project = project_id();
    let bs_name = "gcp-lite-test-bs";
    let hc_name = "gcp-lite-test-hc-for-bs";

    // Pre-cleanup
    let _ = gcloud(&[
        "compute",
        "backend-services",
        "delete",
        bs_name,
        "--global",
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "health-checks",
        "delete",
        hc_name,
        "--project",
        &project,
        "--quiet",
    ]);
    wait_secs(3).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let result = run_backend_service_tests(&client, &project, bs_name, hc_name).await;

    // Cleanup
    println!("\nCleaning up...");
    let _ = gcloud(&[
        "compute",
        "backend-services",
        "delete",
        bs_name,
        "--global",
        "--project",
        &project,
        "--quiet",
    ]);
    let _ = gcloud(&[
        "compute",
        "health-checks",
        "delete",
        hc_name,
        "--project",
        &project,
        "--quiet",
    ]);

    result.expect("backend service lifecycle tests failed");
    println!("\nAll backend service lifecycle tests passed!");
}

async fn run_backend_service_tests(
    client: &GcpHttpClient,
    project: &str,
    bs_name: &str,
    hc_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let compute = client.compute();

    // === 1. CREATE HEALTH CHECK (prerequisite, via gcloud) ===
    println!("\n[1/7] Creating health check via gcloud...");
    assert!(
        gcloud(&[
            "compute",
            "health-checks",
            "create",
            "http",
            hc_name,
            "--port",
            "80",
            "--project",
            project,
        ]),
        "gcloud health check creation failed"
    );
    wait_secs(3).await;
    println!("  Created health check: {hc_name}");

    // === 2. CREATE BACKEND SERVICE ===
    println!("\n[2/7] Creating global backend service...");
    let hc_url = format!("projects/{}/global/healthChecks/{}", project, hc_name);
    let bs = BackendService {
        name: bs_name.to_string(),
        health_checks: vec![hc_url],
        protocol: Some("HTTP".to_string()),
        timeout_sec: Some(30),
        ..Default::default()
    };
    compute.create_global_backend_service(project, &bs).await?;
    println!("  Created: {bs_name}");

    // === 3. GET BACKEND SERVICE ===
    println!("\n[3/7] Getting backend service...");
    let got = compute.get_global_backend_service(project, bs_name).await?;
    assert_eq!(got.name, bs_name);
    assert_eq!(got.protocol.as_deref(), Some("HTTP"));
    assert_eq!(got.timeout_sec, Some(30));
    println!(
        "  Name: {}, Protocol: {:?}, Timeout: {:?}s",
        got.name, got.protocol, got.timeout_sec
    );

    // === 4. LIST BACKEND SERVICES ===
    println!("\n[4/7] Listing global backend services...");
    let list = compute.list_global_backend_services(project).await?;
    let found = list.items.iter().any(|b| b.name == bs_name);
    assert!(found, "Created backend service should appear in list");
    println!("  Found in list ({} total)", list.items.len());

    // === 5. PATCH BACKEND SERVICE ===
    println!("\n[5/7] Patching backend service timeout...");
    let patch = BackendService {
        timeout_sec: Some(60),
        ..Default::default()
    };
    compute
        .patch_global_backend_service(project, bs_name, &patch)
        .await?;
    let patched = compute.get_global_backend_service(project, bs_name).await?;
    assert_eq!(patched.timeout_sec, Some(60));
    println!("  Timeout updated to {:?}s", patched.timeout_sec);

    // === 6. DELETE BACKEND SERVICE ===
    println!("\n[6/7] Deleting backend service...");
    compute
        .delete_global_backend_service(project, bs_name)
        .await?;
    println!("  Deleted");

    // === 7. VERIFY DELETION ===
    println!("\n[7/7] Verifying deletion...");
    wait_secs(3).await;
    let result = compute.get_global_backend_service(project, bs_name).await;
    assert!(
        result.is_err(),
        "Deleted backend service should return error"
    );
    println!("  Confirmed: backend service is gone");

    Ok(())
}

// =============================================================================
// list_ssl_policies — read-only, CIS 3.9
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_ssl_policies() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let compute = client.compute();

    println!("\n=== SSL Policies List ===");
    println!("Project: {}", project);

    let policies = compute.list_ssl_policies(&project).await?;
    println!("  Found {} SSL policy(-ies)", policies.len());

    for p in &policies {
        println!(
            "  - {} (profile={:?}, minTlsVersion={:?})",
            p.name, p.profile, p.min_tls_version,
        );
    }

    Ok(())
}

// =============================================================================
// patch_subnetwork — enable flow logs on the default subnetwork, then disable
// CIS 3.8
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_patch_subnetwork_flow_logs() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let compute = client.compute();

    // Use the default subnetwork in us-central1 — present in most projects.
    let subnetwork_name = "default";

    println!("\n=== Subnetwork Flow Logs Patch ===");
    println!("Project: {}", project);
    println!("Region: {}", TEST_REGION);
    println!("Subnetwork: {}", subnetwork_name);

    // Get fingerprint from gcloud (needed for fingerprint-based optimistic locking)
    let output = Command::new("gcloud")
        .args([
            "compute",
            "networks",
            "subnets",
            "describe",
            subnetwork_name,
            "--region",
            TEST_REGION,
            "--project",
            &project,
            "--format",
            "value(fingerprint)",
        ])
        .output()?;

    if !output.status.success() {
        println!(
            "  Subnetwork 'default' not found in {} — skipping",
            TEST_REGION
        );
        return Ok(());
    }

    let fingerprint = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("  Fingerprint: {}", fingerprint);

    // Enable flow logs
    println!("\n[1/2] Enabling flow logs...");
    let patch = Subnetwork {
        fingerprint: Some(fingerprint),
        log_config: Some(SubnetworkLogConfig {
            enable: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };
    compute
        .patch_subnetwork(&project, TEST_REGION, subnetwork_name, &patch)
        .await?;
    println!("  Flow logs enabled");

    // Get new fingerprint for rollback
    let output2 = Command::new("gcloud")
        .args([
            "compute",
            "networks",
            "subnets",
            "describe",
            subnetwork_name,
            "--region",
            TEST_REGION,
            "--project",
            &project,
            "--format",
            "value(fingerprint)",
        ])
        .output()?;
    let fingerprint2 = String::from_utf8_lossy(&output2.stdout).trim().to_string();

    // Disable flow logs (rollback)
    println!("\n[2/2] Disabling flow logs (rollback)...");
    let rollback = Subnetwork {
        fingerprint: Some(fingerprint2),
        log_config: Some(SubnetworkLogConfig {
            enable: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    };
    compute
        .patch_subnetwork(&project, TEST_REGION, subnetwork_name, &rollback)
        .await?;
    println!("  Flow logs disabled");

    Ok(())
}

// =============================================================================
// set_instance_metadata — set/remove a metadata key on a pre-existing instance
// CIS 4.3/4.4/4.5
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_set_instance_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let compute = client.compute();

    // Use a pre-existing test instance. Skip if not found.
    let instance_name = "gcp-lite-test-metadata-vm";

    println!("\n=== Set Instance Metadata ===");
    println!("Project: {}", project);
    println!("Zone: {}", TEST_ZONE);
    println!("Instance: {}", instance_name);

    let instance = match compute
        .get_instance(&project, TEST_ZONE, instance_name)
        .await
    {
        Ok(i) => i,
        Err(_) => {
            println!("  Instance '{}' not found — skipping", instance_name);
            return Ok(());
        }
    };

    let fingerprint = instance
        .metadata
        .as_ref()
        .and_then(|m| m.fingerprint.as_deref())
        .unwrap_or("")
        .to_string();
    let current_items = instance
        .metadata
        .as_ref()
        .map(|m| m.items.clone())
        .unwrap_or_default();

    println!("  Current metadata fingerprint: {}", fingerprint);
    println!("  Current metadata items: {}", current_items.len());

    // Add a test metadata key
    let test_key = "cloud-lite-test-key";
    let test_value = "cloud-lite-test-value";
    let mut new_items = current_items.clone();
    new_items.retain(|item| item.key.as_deref() != Some(test_key));
    new_items.push(MetadataItem {
        key: Some(test_key.to_string()),
        value: Some(test_value.to_string()),
    });

    println!("\n[1/2] Setting metadata key '{}'...", test_key);
    let new_meta = Metadata {
        fingerprint: Some(fingerprint),
        items: new_items,
        ..Default::default()
    };
    compute
        .set_instance_metadata(&project, TEST_ZONE, instance_name, &new_meta)
        .await?;
    println!("  Metadata set");

    // Verify
    let updated = compute
        .get_instance(&project, TEST_ZONE, instance_name)
        .await?;
    let updated_items = updated
        .metadata
        .as_ref()
        .map(|m| m.items.clone())
        .unwrap_or_default();
    let found = updated_items
        .iter()
        .any(|i| i.key.as_deref() == Some(test_key) && i.value.as_deref() == Some(test_value));
    assert!(found, "Test metadata key should appear after set");
    println!("  Verified: key '{}' = '{}'", test_key, test_value);

    // Remove test key (rollback)
    let new_fp = updated
        .metadata
        .as_ref()
        .and_then(|m| m.fingerprint.as_deref())
        .unwrap_or("")
        .to_string();
    let mut rollback_items = updated_items;
    rollback_items.retain(|i| i.key.as_deref() != Some(test_key));

    println!("\n[2/2] Removing test metadata key (rollback)...");
    let rollback_meta = Metadata {
        fingerprint: Some(new_fp),
        items: rollback_items,
        ..Default::default()
    };
    compute
        .set_instance_metadata(&project, TEST_ZONE, instance_name, &rollback_meta)
        .await?;
    println!("  Test key removed");

    Ok(())
}
