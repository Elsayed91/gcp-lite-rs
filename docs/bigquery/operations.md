# BigQuery Operations

## Datasets

### get_dataset

**Signature**: `pub async fn get_dataset(project: &str, dataset_id: &str) -> Result<Dataset>`

Get a dataset by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `dataset_id` | `&str` | Dataset ID (e.g., "my_dataset") |

**Returns**: `Result<Dataset>`

---

### list_datasets

**Signature**: `pub async fn list_datasets(project: &str) -> Result<Vec<DatasetListItem>>`

List all datasets in a project. Auto-paginates to collect all results.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |

**Returns**: `Result<Vec<DatasetListItem>>`

---

### list_datasets_with_filter

**Signature**: `pub async fn list_datasets_with_filter(project: &str, filter: &str) -> Result<Vec<DatasetListItem>>`

List datasets with a label filter.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `filter` | `&str` | Label filter (e.g., "labels.key:value") |

**Returns**: `Result<Vec<DatasetListItem>>`

---

## Tables

### get_table

**Signature**: `pub async fn get_table(project: &str, dataset_id: &str, table_id: &str) -> Result<Table>`

Get a table by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `dataset_id` | `&str` | Dataset ID |
| `table_id` | `&str` | Table ID |

**Returns**: `Result<Table>`

---

### list_tables

**Signature**: `pub async fn list_tables(project: &str, dataset_id: &str) -> Result<Vec<TableListItem>>`

List all tables in a dataset.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `dataset_id` | `&str` | Dataset ID |

**Returns**: `Result<Vec<TableListItem>>`

---

### patch_table

**Signature**: `pub async fn patch_table(project: &str, dataset_id: &str, table_id: &str, body: &Table) -> Result<Table>`

Patch a table (partial update using patch semantics).

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `dataset_id` | `&str` | Dataset ID |
| `table_id` | `&str` | Table ID |
| `body` | `&Table` | Fields to update |

**Returns**: `Result<Table>`

---

## Jobs & Queries

### query

**Signature**: `pub async fn query(project: &str, body: &QueryRequest) -> Result<QueryResponse>`

Run a synchronous SQL query.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `body` | `&QueryRequest` | Query configuration |

**Returns**: `Result<QueryResponse>`

---

### insert_job

**Signature**: `pub async fn insert_job(project: &str, body: &Job) -> Result<Job>`

Insert (start) an asynchronous job.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `body` | `&Job` | Job configuration |

**Returns**: `Result<Job>`

---

### get_job

**Signature**: `pub async fn get_job(project: &str, job_id: &str) -> Result<Job>`

Get a job by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<Job>`

---

### get_job_with_location

**Signature**: `pub async fn get_job_with_location(project: &str, job_id: &str, location: &str) -> Result<Job>`

Get a job by ID with an explicit location.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `job_id` | `&str` | Job ID |
| `location` | `&str` | Job location (e.g., "US", "EU") |

**Returns**: `Result<Job>`

---

### list_jobs

**Signature**: `pub async fn list_jobs(project: &str) -> Result<Vec<JobListItem>>`

List all jobs in a project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |

**Returns**: `Result<Vec<JobListItem>>`
