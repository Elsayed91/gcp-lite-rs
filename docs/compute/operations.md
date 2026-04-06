# Compute Engine Operations

## Instances

### create_instance

**Signature**: `pub async fn create_instance(project: &str, zone: &str, instance: &Instance) -> Result<()>`

Creates an instance in the specified zone. Blocks until complete.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `zone` | `&str` | Zone (e.g., "us-central1-a") |
| `instance` | `&Instance` | Instance configuration |

**Returns**: `Result<()>`

---

### delete_instance

**Signature**: `pub async fn delete_instance(project: &str, zone: &str, instance: &str) -> Result<()>`

Deletes an instance. Blocks until complete.

---

### start_instance / stop_instance / reset_instance

**Signatures**:
- `pub async fn start_instance(...) -> Result<()>`
- `pub async fn stop_instance(...) -> Result<()>`
- `pub async fn reset_instance(...) -> Result<()>`

Manage instance power state. All block until complete.

---

### get_instance

**Signature**: `pub async fn get_instance(project: &str, zone: &str, instance: &str) -> Result<Instance>`

Get instance details.

---

### list_instances

**Signature**: `pub async fn list_instances(project: &str, zone: &str) -> Result<InstanceList>`

List instances in a zone.

---

## Disks & Snapshots

### create_disk

**Signature**: `pub async fn create_disk(project: &str, zone: &str, disk: &Disk) -> Result<()>`

Create a persistent disk.

---

### resize_disk

**Signature**: `pub async fn resize_disk(project: &str, zone: &str, disk: &str, new_size_gb: u64) -> Result<()>`

Resize a disk to a larger size.

---

### create_snapshot

**Signature**: `pub async fn create_snapshot(project: &str, zone: &str, disk: &str, snapshot_name: &str) -> Result<()>`

Create a snapshot from a disk.

---

## Networking

### get_address / list_addresses

Manage static IP addresses.

### release_address

**Signature**: `pub async fn release_address(project: &str, region: &str, address: &str) -> Result<()>`

Release (delete) a static IP address.

### get_router / patch_router

Manage Cloud Routers.

### delete_nat_gateway

**Signature**: `pub async fn delete_nat_gateway(project: &str, region: &str, router: &str, nat_name: &str) -> Result<()>`

Removes a NAT gateway from a Cloud Router.

---

## Backend Services

### create_global_backend_service / create_regional_backend_service

Create load balancer backend services.

### get_global_backend_service / get_regional_backend_service

Get backend service details.

### delete_global_backend_service / delete_regional_backend_service

Delete backend services.
