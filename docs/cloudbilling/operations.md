# Cloud Billing Operations

## Project Billing Info

### get_billing_info

**Signature**: `pub async fn get_billing_info(project: &str) -> Result<ProjectBillingInfo>`

Gets the billing information for a project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |

**Returns**: `Result<ProjectBillingInfo>`

---

### update_billing_info

**Signature**: `pub async fn update_billing_info(project: &str, billing_account: Option<&str>) -> Result<ProjectBillingInfo>`

Sets or updates the billing account associated with a project. Pass `None` to disable billing.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `billing_account` | `Option<&str>` | Billing account ID (e.g. "012345-567890-ABCDEF") or None to disable |

**Returns**: `Result<ProjectBillingInfo>`
