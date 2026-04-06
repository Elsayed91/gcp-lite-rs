# Cloud Storage Usage Examples

## Buckets

### Create a Bucket

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::storage::Bucket;

let client = GcpHttpClient::from_adc().await?;
let storage = client.storage();

let bucket = Bucket {
    name: "my-unique-bucket-name".to_string(),
    location: Some("US".to_string()),
    storage_class: Some("STANDARD".to_string()),
    ..Default::default()
};

storage.create_bucket("my-project", &bucket).await?;
println!("Bucket created!");
```

### Enable Public Access Prevention

```rust
storage.set_public_access_prevention("my-unique-bucket-name", true).await?;
println!("Public access prevention enforced.");
```

## Objects

### List Objects

```rust
let objects = storage.list_objects("my-unique-bucket-name", Some("logs/"), None).await?;

for obj in objects.items {
    println!("Found object: {}", obj.name.unwrap_or_default());
}
```

### Rewrite (Move) an Object

```rust
use gcp_lite::types::storage::Object;

let mut token = None;
loop {
    let resp = storage.rewrite_object(
        "source-bucket", "source.txt",
        "dest-bucket", "dest.txt",
        token.as_deref(),
        &Object::default() // No metadata changes
    ).await?;

    if resp.done.unwrap_or(false) {
        println!("Rewrite complete!");
        break;
    }

    token = resp.rewrite_token;
    println!("Rewritten {} bytes...", resp.total_bytes_rewritten.unwrap_or("0".to_string()));
}
```
