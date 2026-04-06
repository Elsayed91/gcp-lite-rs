# Cloud Billing Usage Examples

## Get Project Billing Info

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let billing = client.billing();

let info = billing.get_billing_info("my-project").await?;
println!("Billing enabled: {}", info.billing_enabled.unwrap_or(false));

if let Some(account) = &info.billing_account_name {
    println!("Linked to: {}", account);
} else {
    println!("No billing account linked");
}
```

## Link Project to Billing Account

```rust
let info = billing.update_billing_info(
    "my-project",
    Some("012345-567890-ABCDEF")
).await?;

println!("Billing linked: {}", info.billing_enabled.unwrap_or(false));
```

## Disable Billing for Project

```rust
let info = billing.update_billing_info(
    "my-project",
    None
).await?;

println!("Billing disabled. Enabled status: {}", info.billing_enabled.unwrap_or(false));
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::types::cloudbilling::ProjectBillingInfo;
use serde_json::json;

#[tokio::test]
async fn test_get_billing_info() {
    let mut mock = MockClient::new();

    mock.expect_get("/v1/projects/test-project/billingInfo")
        .returning_json(json!({
            "name": "projects/test-project/billingInfo",
            "projectId": "test-project",
            "billingAccountName": "billingAccounts/012345-567890-ABCDEF",
            "billingEnabled": true
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let info = client.billing().get_billing_info("test-project").await.unwrap();

    assert_eq!(info.project_id.as_deref(), Some("test-project"));
    assert_eq!(info.billing_enabled, Some(true));
}
```
