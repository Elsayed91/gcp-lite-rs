# Cloud Scheduler Operations

## Jobs

### create_job

**Signature**: `pub async fn create_job(project: &str, location: &str, job: &Job) -> Result<Job>`

Creates a new scheduled job.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job` | `&Job` | Job configuration |

**Returns**: `Result<Job>` - The created job

---

### get_job

**Signature**: `pub async fn get_job(project: &str, location: &str, job_id: &str) -> Result<Job>`

Gets a job by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<Job>` - The job

---

### list_jobs

**Signature**: `pub async fn list_jobs(project: &str, location: &str) -> Result<Vec<Job>>`

Lists all jobs in a location.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |

**Returns**: `Result<Vec<Job>>` - List of jobs

---

### update_job

**Signature**: `pub async fn update_job(project: &str, location: &str, job_id: &str, job: &Job, update_mask: &str) -> Result<Job>`

Updates a job. Only fields specified in the update mask are modified.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |
| `job` | `&Job` | Updated job configuration |
| `update_mask` | `&str` | Comma-separated list of fields to update |

**Returns**: `Result<Job>` - The updated job

---

### delete_job

**Signature**: `pub async fn delete_job(project: &str, location: &str, job_id: &str) -> Result<()>`

Deletes a job.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<()>`

---

### pause_job

**Signature**: `pub async fn pause_job(project: &str, location: &str, job_id: &str) -> Result<Job>`

Pauses a job. The job must be in ENABLED state.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<Job>` - The job with state PAUSED

---

### resume_job

**Signature**: `pub async fn resume_job(project: &str, location: &str, job_id: &str) -> Result<Job>`

Resumes a paused job. The job must be in PAUSED state.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<Job>` - The job with state ENABLED

---

### run_job

**Signature**: `pub async fn run_job(project: &str, location: &str, job_id: &str) -> Result<Job>`

Forces a job to run immediately, even if already running.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `job_id` | `&str` | Job ID |

**Returns**: `Result<Job>` - The job with updated `last_attempt_time`
