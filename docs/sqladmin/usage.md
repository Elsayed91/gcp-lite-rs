# Cloud SQL Admin Usage Examples

## Instance Management

### Create a PostgreSQL instance

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::sqladmin::{
    DatabaseInstance, Settings, SqlBackendType, SqlInstanceType,
};

let client = GcpHttpClient::from_adc().await?;
let sql = client.sqladmin();

let instance = DatabaseInstance {
    name: "my-instance".to_string(),
    project: Some("my-project".to_string()),
    database_version: Some("POSTGRES_14".to_string()),
    region: Some("us-central1".to_string()),
    instance_type: Some(SqlInstanceType::CloudSqlInstance),
    backend_type: Some(SqlBackendType::SecondGen),
    settings: Some(Settings {
        tier: Some("db-f1-micro".to_string()),
        ..Default::default()
    }),
    ..Default::default()
};

// Blocking (waits ~5-10 minutes)
sql.create_instance("my-project", &instance).await?;

// Non-blocking
let op = sql.create_instance_start("my-project", &instance).await?;
// Do other work...
op.wait().await?;
```

### Get instance details

```rust
let instance = sql.get_instance("my-project", "my-instance").await?;
println!("Name: {}", instance.name);
println!("Version: {:?}", instance.database_version);
println!("Region: {:?}", instance.region);
println!("State: {:?}", instance.state);
```

### List instances

```rust
let instances = sql.list_instances("my-project").await?;
for inst in &instances {
    println!("{} ({:?})", inst.name, inst.state);
}
```

### Restart an instance

```rust
sql.restart_instance("my-project", "my-instance").await?;
```

### Delete an instance

```rust
sql.delete_instance("my-project", "my-instance").await?;
```

## Database Management

### Create a database

```rust
use gcp_lite::types::sqladmin::Database;

let db = Database {
    name: "mydb".to_string(),
    project: Some("my-project".to_string()),
    instance: Some("my-instance".to_string()),
    ..Default::default()
};
sql.create_database("my-project", "my-instance", &db).await?;
```

### List databases

```rust
let dbs = sql.list_databases("my-project", "my-instance").await?;
for db in &dbs {
    println!("{} (charset={:?})", db.name, db.charset);
}
```

### Delete a database

```rust
sql.delete_database("my-project", "my-instance", "mydb").await?;
```

## User Management

### Create a user

```rust
use gcp_lite::types::sqladmin::User;

let user = User {
    name: "appuser".to_string(),
    instance: Some("my-instance".to_string()),
    password: Some("secure-password".to_string()),
    ..Default::default()
};
sql.create_user("my-project", "my-instance", &user).await?;
```

### Update a user's password

```rust
let update = User {
    name: "appuser".to_string(),
    password: Some("new-password".to_string()),
    ..Default::default()
};
// For PostgreSQL, pass "" for host
sql.update_user("my-project", "my-instance", "appuser", "", &update).await?;
```

### Delete a user

```rust
// For PostgreSQL, pass "" for host
sql.delete_user("my-project", "my-instance", "appuser", "").await?;
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::test_support::SqladminMockHelpers;
use serde_json::json;

#[tokio::test]
async fn test_list_instances() {
    let mut mock = MockClient::new();

    mock.expect_list_instances("test-project")
        .returning_json(json!({
            "items": [{
                "name": "my-instance",
                "databaseVersion": "POSTGRES_14",
                "region": "us-central1"
            }]
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let instances = client.sqladmin().list_instances("test-project").await.unwrap();

    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].name, "my-instance");
}
```
