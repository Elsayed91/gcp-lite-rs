# Service Usage Operations

## Query

### get_service

**Signature**: `pub async fn get_service(project: &str, service: &str) -> Result<ServiceState>`

Get a service's state.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `service` | `&str` | Service ID (e.g., "compute.googleapis.com") |

**Returns**: `Result<ServiceState>`

---

### is_service_enabled

**Signature**: `pub async fn is_service_enabled(project: &str, service: &str) -> Result<bool>`

Helper to check if a service is enabled.

---

### list_services

**Signature**: `pub async fn list_services(project: &str) -> Result<ListServicesResponse>`

List all services available to a project.

---

## Enable / Disable

### enable_service

**Signature**: `pub async fn enable_service(project: &str, service: &str) -> Result<()>`

Enable a service. Blocks until complete.

---

### batch_enable_services

**Signature**: `pub async fn batch_enable_services(project: &str, service_ids: Vec<String>) -> Result<()>`

Enable multiple services at once. Blocks until complete.

---

### disable_service

**Signature**: `pub async fn disable_service(project: &str, service: &str) -> Result<()>`

Disable a service. Blocks until complete.

---

### disable_service_with_request

**Signature**: `pub async fn disable_service_with_request(project: &str, service: &str, request: &DisableServiceRequest) -> Result<()>`

Disable a service with options (e.g., `checkIfServiceHasUsage`).
