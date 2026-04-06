# IAM Usage Examples

## Service Accounts

### Create Service Account

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let iam = client.iam();

let sa = iam.create_service_account(
    "my-project",
    "my-app-sa",
    "My App Service Account",
    "Used for backend application"
).await?;

println!("Created SA: {}", sa.email.unwrap_or_default());
```

### List Service Accounts

```rust
let accounts = iam.list_service_accounts("my-project").await?;
for account in accounts.accounts {
    println!("Found: {}", account.email.unwrap_or_default());
}
```

## Keys

### Create a Key

```rust
use gcp_lite::types::iam::CreateServiceAccountKeyRequest;

let key_req = CreateServiceAccountKeyRequest {
    key_algorithm: Some("KEY_ALG_RSA_2048".to_string()),
    ..Default::default()
};

let key = iam.create_service_account_key(
    "my-project",
    "my-app-sa@my-project.iam.gserviceaccount.com",
    &key_req
).await?;

if let Some(data) = key.private_key_data {
    println!("Private key data received (length: {})", data.len());
    // WARNING: Handle sensitive key data securely!
}
```

### Delete a Key

```rust
iam.delete_service_account_key(
    "my-project",
    "my-app-sa@my-project.iam.gserviceaccount.com",
    "key_id_to_delete"
).await?;
```
