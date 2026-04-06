# Cloud Scheduler Usage Examples

## Basic CRUD

### Create a job

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::types::cloudscheduler::{Job, HttpTarget};

let client = GcpHttpClient::from_adc().await?;

let job = Job {
    name: "projects/my-project/locations/us-central1/jobs/daily-backup".to_string(),
    description: Some("Daily database backup".to_string()),
    schedule: Some("0 2 * * *".to_string()),  // Every day at 2 AM
    time_zone: Some("America/New_York".to_string()),
    http_target: Some(HttpTarget {
        uri: Some("https://my-api.example.com/backup".to_string()),
        http_method: None,  // Defaults to POST
        ..Default::default()
    }),
    ..Default::default()
};

let created = client.scheduler().create_job("my-project", "us-central1", &job).await?;
println!("Created job: {}", created.name);
```

### Get a job

```rust
let job = client.scheduler().get_job("my-project", "us-central1", "daily-backup").await?;
println!("Schedule: {:?}", job.schedule);
println!("State: {:?}", job.state);
```

### List jobs

```rust
let jobs = client.scheduler().list_jobs("my-project", "us-central1").await?;
for job in &jobs {
    println!("{}: {:?}", job.name, job.state);
}
```

### Update a job

```rust
let mut job = client.scheduler().get_job("my-project", "us-central1", "daily-backup").await?;
job.schedule = Some("0 3 * * *".to_string());  // Change to 3 AM

let updated = client.scheduler()
    .update_job("my-project", "us-central1", "daily-backup", &job, "schedule")
    .await?;
println!("Updated schedule: {:?}", updated.schedule);
```

### Delete a job

```rust
client.scheduler().delete_job("my-project", "us-central1", "daily-backup").await?;
```

## Job Control

### Pause and resume a job

```rust
// Pause - stops scheduled executions
let paused = client.scheduler()
    .pause_job("my-project", "us-central1", "daily-backup")
    .await?;
assert_eq!(paused.state, Some(JobState::Paused));

// Resume - restarts scheduled executions
let resumed = client.scheduler()
    .resume_job("my-project", "us-central1", "daily-backup")
    .await?;
assert_eq!(resumed.state, Some(JobState::Enabled));
```

### Force run a job immediately

```rust
// Trigger immediate execution (doesn't wait for schedule)
let result = client.scheduler()
    .run_job("my-project", "us-central1", "daily-backup")
    .await?;
println!("Last attempt: {:?}", result.last_attempt_time);
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::types::cloudscheduler::{Job, JobState};
use serde_json::json;

#[tokio::test]
async fn test_job_operations() {
    let mut mock = MockClient::new();
    
    // Set up expectations
    mock.expect_get("/v1/projects/test-project/locations/us-central1/jobs/my-job")
        .returning_json(json!({
            "name": "projects/test-project/locations/us-central1/jobs/my-job",
            "schedule": "0 9 * * 1",
            "state": "ENABLED"
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let job = client.scheduler()
        .get_job("test-project", "us-central1", "my-job")
        .await
        .unwrap();
    
    assert_eq!(job.state, Some(JobState::Enabled));
}
```

## HTTP Target with Authentication

```rust
use gcp_lite::types::cloudscheduler::{Job, HttpTarget, OidcToken};

let job = Job {
    name: "projects/my-project/locations/us-central1/jobs/authenticated-call".to_string(),
    schedule: Some("*/5 * * * *".to_string()),  // Every 5 minutes
    time_zone: Some("UTC".to_string()),
    http_target: Some(HttpTarget {
        uri: Some("https://my-cloud-run-service.run.app/process".to_string()),
        http_method: None,
        oidc_token: Some(OidcToken {
            service_account_email: Some("scheduler@my-project.iam.gserviceaccount.com".to_string()),
            audience: Some("https://my-cloud-run-service.run.app".to_string()),
        }),
        ..Default::default()
    }),
    ..Default::default()
};
```

## Pub/Sub Target

```rust
use gcp_lite::types::cloudscheduler::{Job, PubsubTarget};

let job = Job {
    name: "projects/my-project/locations/us-central1/jobs/pubsub-trigger".to_string(),
    schedule: Some("0 * * * *".to_string()),  // Every hour
    time_zone: Some("UTC".to_string()),
    pubsub_target: Some(PubsubTarget {
        topic_name: Some("projects/my-project/topics/my-topic".to_string()),
        data: Some(base64::encode("trigger message")),
        ..Default::default()
    }),
    ..Default::default()
};
```
