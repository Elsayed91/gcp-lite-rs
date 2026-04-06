# IAM Operations

## Service Accounts

### create_service_account

**Signature**: `pub async fn create_service_account(project: &str, account_id: &str, display_name: &str, description: &str) -> Result<ServiceAccount>`

Creates a service account in a project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `account_id` | `&str` | Unique ID for the account (e.g., "my-sa") |
| `display_name` | `&str` | Human-readable name |
| `description` | `&str` | Description |

**Returns**: `Result<ServiceAccount>`

---

### get_service_account

**Signature**: `pub async fn get_service_account(project: &str, email: &str) -> Result<ServiceAccount>`

Get a service account by email.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `email` | `&str` | Service account email (e.g., "sa@project.iam.gserviceaccount.com") |

**Returns**: `Result<ServiceAccount>`

---

### list_service_accounts

**Signature**: `pub async fn list_service_accounts(project: &str) -> Result<ListServiceAccountsResponse>`

List all service accounts in a project.

---

### delete_service_account

**Signature**: `pub async fn delete_service_account(project: &str, email: &str) -> Result<()>`

Delete a service account.

---

## Service Account Keys

### create_service_account_key

**Signature**: `pub async fn create_service_account_key(project: &str, email: &str, body: &CreateServiceAccountKeyRequest) -> Result<ServiceAccountKey>`

Create a key for a service account.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `email` | `&str` | Service account email |
| `body` | `&CreateServiceAccountKeyRequest` | Key configuration (algorithm, type) |

**Returns**: `Result<ServiceAccountKey>` (contains private key data)

---

### list_service_account_keys

**Signature**: `pub async fn list_service_account_keys(project: &str, email: &str) -> Result<ListServiceAccountKeysResponse>`

List all keys for a service account.

---

### delete_service_account_key

**Signature**: `pub async fn delete_service_account_key(project: &str, email: &str, key_id: &str) -> Result<()>`

Delete a service account key.
