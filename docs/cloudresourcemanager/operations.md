# Cloud Resource Manager Operations

## Project Management (LRO)

### create_project

**Signature**: `pub async fn create_project(project: &Project) -> Result<()>`

Creates a new project. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&Project` | Project metadata including `project_id`, `display_name`, and `parent` |

**Returns**: `Result<()>`

---

### delete_project

**Signature**: `pub async fn delete_project(project: &str) -> Result<()>`

Marks a project for deletion. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID (e.g., "my-project-123") |

**Returns**: `Result<()>`

---

### undelete_project

**Signature**: `pub async fn undelete_project(project: &str) -> Result<()>`

Restores a deleted project. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |

**Returns**: `Result<()>`

---

### move_project

**Signature**: `pub async fn move_project(project: &str, destination_parent: &str) -> Result<()>`

Moves a project to a new parent folder or organization. Blocks until complete.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |
| `destination_parent` | `&str` | New parent (e.g., "folders/123", "organizations/456") |

**Returns**: `Result<()>`

---

### update_project

**Signature**: `pub async fn update_project(project: &Project) -> Result<()>`

Updates the display name and labels of a project. Blocks until complete.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&Project` | Updated project metadata |

**Returns**: `Result<()>`

---

## Querying

### get_project

**Signature**: `pub async fn get_project(project: &str) -> Result<Project>`

Retrieves a project by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |

**Returns**: `Result<Project>`

---

### list_projects

**Signature**: `pub async fn list_projects(parent: &str) -> Result<ListProjectsResponse>`

Lists projects under a parent resource.

| Parameter | Type | Description |
|-----------|------|-------------|
| `parent` | `&str` | Parent resource (e.g., "folders/123") |

**Returns**: `Result<ListProjectsResponse>`

---

### search_projects

**Signature**: `pub async fn search_projects(query: &str) -> Result<SearchProjectsResponse>`

Search for projects matching a query.

| Parameter | Type | Description |
|-----------|------|-------------|
| `query` | `&str` | Search query (e.g., "id:my-project", "state:ACTIVE") |

**Returns**: `Result<SearchProjectsResponse>`

---

## IAM Policy

### get_iam_policy

**Signature**: `pub async fn get_iam_policy(project: &str) -> Result<IamPolicy>`

Gets the IAM policy for a project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |

**Returns**: `Result<IamPolicy>`

---

### set_iam_policy

**Signature**: `pub async fn set_iam_policy(project: &str, policy: &IamPolicy) -> Result<IamPolicy>`

Sets the IAM policy for a project. Replaces the entire policy.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |
| `policy` | `&IamPolicy` | New policy (must include etag for concurrency control) |

**Returns**: `Result<IamPolicy>`

---

### add_iam_policy_binding

**Signature**: `pub async fn add_iam_policy_binding(project: &str, role: &str, member: &str) -> Result<IamPolicy>`

Adds a member to a role. Handles read-modify-write with etag conflict retry.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |
| `role` | `&str` | IAM role (e.g., "roles/viewer") |
| `member` | `&str` | IAM member (e.g., "user:jane@example.com") |

**Returns**: `Result<IamPolicy>`

---

### remove_iam_policy_binding

**Signature**: `pub async fn remove_iam_policy_binding(project: &str, role: &str, member: &str) -> Result<IamPolicy>`

Removes a member from a role. Handles read-modify-write with etag conflict retry.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |
| `role` | `&str` | IAM role |
| `member` | `&str` | IAM member |

**Returns**: `Result<IamPolicy>`

---

### test_iam_permissions

**Signature**: `pub async fn test_iam_permissions(project: &str, permissions: Vec<String>) -> Result<TestIamPermissionsResponse>`

Returns permissions that the caller has on the specified project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | Project ID |
| `permissions` | `Vec<String>` | List of permissions to check |

**Returns**: `Result<TestIamPermissionsResponse>`
