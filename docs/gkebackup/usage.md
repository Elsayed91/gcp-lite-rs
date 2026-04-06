# Backup for GKE Usage Examples

## List Backup Plans

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let backup = client.gkebackup();

let plans = backup.list_backup_plans("my-project", "us-central1").await?;
for plan in &plans {
    println!("{} (state={:?}, cluster={:?})", plan.name, plan.state, plan.cluster);
}
```

## Create a Backup Plan

### All namespaces

```rust
use gcp_lite::types::gkebackup::{BackupPlan, BackupConfig};

let plan = BackupPlan {
    cluster: Some("projects/my-project/locations/us-central1/clusters/my-cluster".to_string()),
    description: Some("Daily full backup".to_string()),
    backup_config: Some(BackupConfig {
        all_namespaces: Some(true),
        ..Default::default()
    }),
    ..Default::default()
};

// Blocking (waits for completion)
backup.create_backup_plan("my-project", "us-central1", "daily-backup", &plan).await?;
```

### Non-blocking with extended timeout

```rust
use std::time::Duration;

let op = backup
    .create_backup_plan_start("my-project", "us-central1", "daily-backup", &plan)
    .await?;

// Fresh clusters may need extra time for backup agent installation
op.with_timeout(Duration::from_secs(900)).wait().await?;
```

## Get a Backup Plan

```rust
let plan = backup.get_backup_plan("my-project", "us-central1", "daily-backup").await?;
println!("Name: {}", plan.name);
println!("State: {:?}", plan.state);
println!("Cluster: {:?}", plan.cluster);
println!("Description: {:?}", plan.description);
```

## Delete a Backup Plan

```rust
// Blocking
backup.delete_backup_plan("my-project", "us-central1", "daily-backup").await?;

// Non-blocking
let op = backup.delete_backup_plan_start("my-project", "us-central1", "daily-backup").await?;
op.wait().await?;
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::types::gkebackup::{BackupPlan, BackupPlanState};
use serde_json::json;

#[tokio::test]
async fn test_get_backup_plan() {
    let mut mock = MockClient::new();

    mock.expect_get("/v1/projects/test-project/locations/us-central1/backupPlans/my-plan")
        .returning_json(json!({
            "name": "projects/test-project/locations/us-central1/backupPlans/my-plan",
            "cluster": "projects/test-project/locations/us-central1/clusters/my-cluster",
            "state": "READY",
            "description": "Test backup plan",
            "backupConfig": { "allNamespaces": true }
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let plan = client.gkebackup()
        .get_backup_plan("test-project", "us-central1", "my-plan")
        .await
        .unwrap();

    assert!(plan.name.ends_with("my-plan"));
    assert_eq!(plan.state, Some(BackupPlanState::Ready));
}
```
