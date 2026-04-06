# Cloud Resource Manager Usage Examples

## Project Management

### Create a Project

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::cloudresourcemanager::Project;

let client = GcpHttpClient::from_adc().await?;
let projects = client.projects();

let new_project = Project {
    project_id: Some("my-new-project-123".to_string()),
    display_name: Some("My New Project".to_string()),
    parent: Some("organizations/456".to_string()),
    ..Default::default()
};

projects.create_project(&new_project).await?;
println!("Project created!");
```

### Delete a Project

```rust
projects.delete_project("my-new-project-123").await?;
println!("Project deleted.");
```

### Undelete a Project

```rust
projects.undelete_project("my-new-project-123").await?;
println!("Project restored.");
```

## IAM Operations

### Add IAM Binding

```rust
let policy = projects
    .add_iam_policy_binding(
        "my-project-123",
        "roles/viewer",
        "user:jane@example.com"
    )
    .await?;

println!("Updated policy version: {}", policy.version.unwrap_or_default());
```

### Remove IAM Binding

```rust
let policy = projects
    .remove_iam_policy_binding(
        "my-project-123",
        "roles/viewer",
        "user:jane@example.com"
    )
    .await?;

println!("Updated policy version: {}", policy.version.unwrap_or_default());
```

### Test Permissions

```rust
let perms = projects.test_iam_permissions(
    "my-project-123",
    vec![
        "resourcemanager.projects.get".to_string(),
        "resourcemanager.projects.delete".to_string(),
    ]
).await?;

for perm in perms.permissions {
    println!("I have permission: {}", perm);
}
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::types::cloudresourcemanager::Project;
use serde_json::json;

#[tokio::test]
async fn test_create_project() {
    let mut mock = MockClient::new();

    mock.expect_post("/v3/projects")
        .returning_json(json!({
            "name": "operations/cp.123",
            "done": false
        }))
        .times(1);

    mock.expect_get("/v3/operations/cp.123")
        .returning_json(json!({
            "name": "operations/cp.123",
            "done": true,
            "response": {
                "name": "projects/123",
                "projectId": "my-new-project"
            }
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let projects = client.projects();

    let project = Project {
        project_id: Some("my-new-project".to_string()),
        ..Default::default()
    };

    projects.create_project(&project).await.unwrap();
}
```
