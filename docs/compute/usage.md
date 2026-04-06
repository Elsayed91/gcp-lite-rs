# Compute Engine Usage Examples

## Instances

### Create a VM

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::compute::{Instance, NetworkInterface, AccessConfig, AttachedDisk};

let client = GcpHttpClient::from_adc().await?;
let compute = client.compute();

let instance = Instance {
    name: "my-vm".to_string(),
    machine_type: Some("zones/us-central1-a/machineTypes/e2-micro".to_string()),
    disks: vec![AttachedDisk {
        boot: Some(true),
        auto_delete: Some(true),
        initialize_params: Some(AttachedDiskInitializeParams {
            source_image: Some("projects/debian-cloud/global/images/family/debian-11".to_string()),
            disk_size_gb: Some("10".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }],
    network_interfaces: vec![NetworkInterface {
        access_configs: vec![AccessConfig {
            name: Some("External NAT".to_string()),
            type_value: Some("ONE_TO_ONE_NAT".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }],
    ..Default::default()
};

compute.create_instance("my-project", "us-central1-a", &instance).await?;
println!("Instance created!");
```

### Stop and Start a VM

```rust
// Stop
compute.stop_instance("my-project", "us-central1-a", "my-vm").await?;
println!("Instance stopped.");

// Start
compute.start_instance("my-project", "us-central1-a", "my-vm").await?;
println!("Instance started.");
```

## Disks

### Create and Resize a Disk

```rust
use gcp_lite::types::compute::Disk;

// Create 10GB disk
let disk = Disk {
    name: "my-data-disk".to_string(),
    size_gb: Some("10".to_string()),
    ..Default::default()
};
compute.create_disk("my-project", "us-central1-a", &disk).await?;

// Resize to 20GB
compute.resize_disk("my-project", "us-central1-a", "my-data-disk", 20).await?;
```

## Networking

### Reserve a Static IP

```rust
use gcp_lite::types::compute::Address;

let address = Address {
    name: "my-static-ip".to_string(),
    ..Default::default()
};
// Note: region, not zone
compute.insert_address("my-project", "us-central1", &address).await?;
```

### Configure Cloud NAT

```rust
use gcp_lite::types::compute::{Router, RouterNat};

let router = Router {
    name: "my-router".to_string(),
    nats: vec![RouterNat {
        name: "my-nat".to_string(),
        source_subnetwork_ip_ranges_to_nat: Some("ALL_SUBNETWORKS_ALL_IP_RANGES".to_string()),
        ..Default::default()
    }],
    ..Default::default()
};

// Assuming router already exists, patch it to add NAT
compute.patch_router("my-project", "us-central1", "my-router", &router).await?;
```
