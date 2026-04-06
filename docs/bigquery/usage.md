# BigQuery Usage Examples

## Datasets

### List Datasets

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let bigquery = client.bigquery();

let datasets = bigquery.list_datasets("my-project").await?;
for ds in datasets {
    println!("Dataset: {}", ds.id.unwrap_or_default());
}
```

### Get Dataset Details

```rust
let dataset = bigquery.get_dataset("my-project", "my_dataset").await?;
println!("Location: {}", dataset.location.unwrap_or_default());
```

## Tables

### List Tables

```rust
let tables = bigquery.list_tables("my-project", "my_dataset").await?;
for table in tables {
    println!("Table: {}", table.id.unwrap_or_default());
}
```

### Get Table Details

```rust
let table = bigquery.get_table("my-project", "my_dataset", "my_table").await?;
println!("Rows: {}", table.num_rows.unwrap_or("0".to_string()));
```

## Queries

### Run Synchronous Query

```rust
use gcp_lite::types::bigquery::QueryRequest;

let req = QueryRequest {
    query: "SELECT name, count FROM `my-project.my_dataset.my_table` LIMIT 10".to_string(),
    use_legacy_sql: Some(false),
    ..Default::default()
};

let resp = bigquery.query("my-project", &req).await?;

for row in resp.rows {
    // Process row data (represented as JSON-like structure)
    println!("Row: {:?}", row.f);
}
```

## Asynchronous Jobs

### Start a Query Job

```rust
use gcp_lite::types::bigquery::{Job, JobConfiguration, JobConfigurationQuery};

let job_config = JobConfiguration {
    query: Some(JobConfigurationQuery {
        query: "SELECT * FROM `my-project.my_dataset.large_table`".to_string(),
        use_legacy_sql: Some(false),
        ..Default::default()
    }),
    ..Default::default()
};

let job = Job {
    configuration: Some(job_config),
    ..Default::default()
};

let started_job = bigquery.insert_job("my-project", &job).await?;
println!("Job started: {}", started_job.job_reference.unwrap().job_id);
```

### Check Job Status

```rust
let job = bigquery.get_job("my-project", "job_id_from_insert").await?;
if let Some(status) = job.status {
    println!("State: {}", status.state.unwrap_or_default());
}
```
