# Cloud Asset Usage Examples

## List Assets (Streaming)

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::api::cloudasset::ListAssetsOptions;
use futures::StreamExt;
use std::pin::pin;

let client = GcpHttpClient::from_adc().await?;
let cloud_asset = client.cloud_asset();

let options = ListAssetsOptions {
    content_type: Some("RESOURCE"),
    asset_types: Some(&["compute.googleapis.com/Instance"]),
    ..Default::default()
};

let mut stream = pin!(cloud_asset.list_assets_stream("projects/my-project", options));

while let Some(result) = stream.next().await {
    match result {
        Ok(asset) => println!("Found instance: {:?}", asset.name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Search Resources

```rust
use gcp_lite::api::cloudasset::SearchOptions;

let options = SearchOptions {
    query: Some("location:us-central1"),
    order_by: Some("createTime desc"),
    ..Default::default()
};

let results = cloud_asset
    .search_all_resources_all("projects/my-project", &options)
    .await?;

for result in results {
    println!("Resource: {} ({})",
        result.display_name.unwrap_or_default(),
        result.asset_type.unwrap_or_default()
    );
}
```

## Search IAM Policies

Find all resources where a specific user has the Editor role:

```rust
use gcp_lite::api::cloudasset::SearchIamPoliciesOptions;

let options = SearchIamPoliciesOptions {
    query: Some("policy:roles/editor AND policy:user:jane@example.com"),
    ..Default::default()
};

let mut stream = pin!(cloud_asset.search_all_iam_policies_stream("projects/my-project", options));

while let Some(result) = stream.next().await {
    let policy = result?;
    println!("Resource with editor: {:?}", policy.resource);
}
```
