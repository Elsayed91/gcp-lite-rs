# Secret Manager Usage Guide

Practical examples for common Secret Manager workflows.

## Setup

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::secretmanager::{Automatic, Replication, Secret};
use std::collections::HashMap;

let client = GcpHttpClient::from_adc().await?;
let secrets = client.secret_manager();
let project = "my-project";
```

## Basic CRUD Lifecycle

```rust
// 1. Create a secret with automatic replication
let replication = Replication {
    automatic: Some(Automatic::default()),
    ..Default::default()
};

let mut labels = HashMap::new();
labels.insert("app".to_string(), "backend".to_string());
labels.insert("env".to_string(), "prod".to_string());

let secret = Secret {
    labels: labels.clone(),
    replication: Some(replication),
    ..Default::default()
};

let created = secrets.create_secret(project, "database-password", &secret).await?;
println!("Created: {}", created.name);

// 2. Get the secret metadata
let fetched = secrets.get_secret(project, "database-password").await?;
assert_eq!(fetched.name, created.name);

// 3. Update labels
labels.insert("env".to_string(), "staging".to_string());
let update = Secret {
    labels,
    ..Default::default()
};

let updated = secrets
    .patch_secret(project, "database-password", "labels", &update)
    .await?;
println!("Updated labels: {:?}", updated.labels);

// 4. Delete the secret
secrets.delete_secret(project, "database-password", None).await?;
```

## Listing and Filtering

### List all secrets
```rust
let response = secrets.list_secrets(project, None, None, None).await?;
for secret in response.secrets {
    println!("Secret: {}", secret.name);
    if !secret.labels.is_empty() {
        println!("  Labels: {:?}", secret.labels);
    }
}
```

### Filter by label
```rust
// Secrets with env=prod label
let response = secrets
    .list_secrets(project, Some("labels.env=prod"), None, None)
    .await?;

// Secrets with app label (any value)
let response = secrets
    .list_secrets(project, Some("labels.app:*"), None, None)
    .await?;
```

### Paginate through large lists
```rust
let mut all_secrets = Vec::new();
let mut page_token = None;

loop {
    let response = secrets
        .list_secrets(project, None, Some("100"), page_token.as_deref())
        .await?;

    all_secrets.extend(response.secrets);

    match response.next_page_token {
        Some(token) if !token.is_empty() => page_token = Some(token),
        _ => break,
    }
}

println!("Total secrets: {}", all_secrets.len());
```

## Replication Strategies

### Automatic replication (recommended)
Replicates to all GCP regions automatically:
```rust
let replication = Replication {
    automatic: Some(Automatic::default()),
    ..Default::default()
};
```

### User-managed replication
Explicit control over regions:
```rust
use gcp_lite::types::secretmanager::{Replica, UserManaged};

let replicas = vec![
    Replica {
        location: Some("us-east1".to_string()),
        ..Default::default()
    },
    Replica {
        location: Some("europe-west1".to_string()),
        ..Default::default()
    },
];

let replication = Replication {
    user_managed: Some(UserManaged { replicas }),
    ..Default::default()
};
```

## Label Management

### Add/update labels
```rust
let secret = secrets.get_secret(project, "api-key").await?;

let mut labels = secret.labels;
labels.insert("team".to_string(), "platform".to_string());
labels.insert("cost-center".to_string(), "engineering".to_string());

let update = Secret {
    labels,
    ..Default::default()
};

secrets.patch_secret(project, "api-key", "labels", &update).await?;
```

### Remove all labels
```rust
let update = Secret {
    labels: HashMap::new(),
    ..Default::default()
};

secrets.patch_secret(project, "api-key", "labels", &update).await?;
```

## Conditional Operations with Etag

Prevent race conditions when updating:
```rust
// Get current secret
let secret = secrets.get_secret(project, "api-key").await?;

// Modify labels
let mut labels = secret.labels;
labels.insert("version".to_string(), "2".to_string());

let update = Secret {
    labels,
    etag: secret.etag.clone(),
    ..Default::default()
};

// Update will fail if another process modified the secret
match secrets.patch_secret(project, "api-key", "labels", &update).await {
    Ok(_) => println!("Updated successfully"),
    Err(e) if e.to_string().contains("etag") => {
        println!("Conflict: secret was modified by another process");
    }
    Err(e) => return Err(e.into()),
}

// Conditional delete
secrets.delete_secret(project, "api-key", secret.etag.as_deref()).await?;
```

## Error Handling

```rust
use gcp_lite::GcpError;

match secrets.get_secret(project, "nonexistent").await {
    Ok(secret) => println!("Found: {}", secret.name),
    Err(GcpError::NotFound { .. }) => println!("Secret does not exist"),
    Err(GcpError::PermissionDenied { .. }) => println!("Access denied"),
    Err(e) => return Err(e.into()),
}
```

## Testing with MockClient

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use gcp_lite::{GcpHttpClient, MockClient};
    use gcp_lite::test_support::SecretmanagerMockHelpers;

    #[tokio::test]
    async fn test_get_secret() {
        let mut mock = MockClient::new();
        mock.expect_get_secret("test-project", "test-secret")
            .returning_json(serde_json::json!({
                "name": "projects/12345/secrets/test-secret",
                "labels": {"env": "test"}
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let result = secrets.get_secret("test-project", "test-secret").await;
        assert!(result.is_ok());
    }
}
```

## Integration Test Setup

Enable the API automatically in tests:
```rust
async fn ensure_api_enabled(client: &GcpHttpClient, project: &str) {
    let service_usage = client.service_usage();
    let api = "secretmanager.googleapis.com";

    if !service_usage.is_service_enabled(project, api).await.unwrap_or(false) {
        println!("Enabling Secret Manager API...");
        service_usage.enable_service(project, api).await?;
    }
}
```

## Best Practices

1. **Always include replication on create**
   - Use automatic replication unless you have specific compliance needs

2. **Use labels for organization**
   - Add `env`, `team`, `app`, `cost-center` labels
   - Makes bulk operations easier with filtering

3. **Use etags for critical updates**
   - Prevents race conditions in concurrent environments
   - Especially important for automated systems

4. **Cleanup test resources**
   - Delete secrets created in tests
   - Use descriptive names like `gcp-lite-test-{resource}`

5. **Handle errors explicitly**
   - Check for `NotFound`, `PermissionDenied`, `ApiNotEnabled`
   - Don't silently ignore errors

## Common Patterns

### Check if secret exists
```rust
let exists = secrets.get_secret(project, "my-secret").await.is_ok();
```

### Create or update labels
```rust
let secret_id = "my-secret";
let new_labels = [("env".to_string(), "prod".to_string())].into();

let result = secrets.get_secret(project, secret_id).await;
match result {
    Ok(secret) => {
        let mut labels = secret.labels;
        labels.extend(new_labels);
        let update = Secret { labels, ..Default::default() };
        secrets.patch_secret(project, secret_id, "labels", &update).await?;
    }
    Err(GcpError::NotFound { .. }) => {
        let replication = Replication {
            automatic: Some(Automatic::default()),
            ..Default::default()
        };
        let secret = Secret {
            labels: new_labels,
            replication: Some(replication),
            ..Default::default()
        };
        secrets.create_secret(project, secret_id, &secret).await?;
    }
    Err(e) => return Err(e.into()),
}
```

### Bulk cleanup
```rust
let response = secrets
    .list_secrets(project, Some("labels.test=true"), None, None)
    .await?;

for secret in response.secrets {
    let secret_id = secret.name.split('/').last().unwrap();
    secrets.delete_secret(project, secret_id, None).await?;
    println!("Deleted: {}", secret_id);
}
```
