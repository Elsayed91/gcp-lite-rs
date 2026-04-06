# Service Usage Examples

## Enable APIs

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let service_usage = client.service_usage();

// Enable one
service_usage.enable_service("my-project", "compute.googleapis.com").await?;

// Enable multiple
service_usage.batch_enable_services(
    "my-project",
    vec![
        "container.googleapis.com".to_string(),
        "cloudresourcemanager.googleapis.com".to_string()
    ]
).await?;
```

## Check Status

```rust
if service_usage.is_service_enabled("my-project", "compute.googleapis.com").await? {
    println!("Compute Engine is enabled!");
} else {
    println!("Compute Engine is disabled.");
}
```

## List Services

```rust
let services = service_usage.list_services("my-project").await?;
for svc in services.services {
    println!("{} ({:?})", svc.name.unwrap_or_default(), svc.state);
}
```
