# Recommender Usage Examples

## List IAM Recommendations

Find recommendations to remove over-granted permissions:

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::api::recommender::ListRecommendationsOptions;
use futures::StreamExt;
use std::pin::pin;

let client = GcpHttpClient::from_adc().await?;
let recommender = client.recommender();

let options = ListRecommendationsOptions {
    filter: Some("state_info.state=ACTIVE"),
    ..Default::default()
};

let mut stream = pin!(recommender.list_recommendations_stream(
    "my-project",
    "global",
    "google.iam.policy.Recommender",
    options
));

while let Some(result) = stream.next().await {
    let rec = result?;
    println!("Recommendation: {}", rec.description.unwrap_or_default());

    if let Some(impact) = rec.primary_impact {
        println!("  Category: {:?}", impact.category);
    }
}
```

## List VM Sizing Recommendations

Find opportunities to right-size VMs:

```rust
let options = ListRecommendationsOptions::default();

let recommendations = recommender.list_recommendations_all(
    "my-project",
    "us-central1",
    "google.compute.instance.MachineTypeRecommender",
    &options
).await?;

for rec in recommendations {
    println!("VM Sizing: {}", rec.description.unwrap_or_default());

    if let Some(impact) = rec.primary_impact {
        if let Some(cost) = impact.cost_projection {
            if let Some(money) = cost.cost {
                println!("  Estimated savings: {} {}",
                    money.units.unwrap_or_default(),
                    money.currency_code.unwrap_or_default()
                );
            }
        }
    }
}
```
