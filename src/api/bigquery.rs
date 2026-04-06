//! BigQuery API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::bigquery::BigqueryOps`. This layer adds:
//! - Ergonomic method signatures (project/dataset/table instead of raw paths)
//! - Auto-paginating list methods that collect all pages
//! - Capacity commitment operations (Reservation API, different base URL)

use crate::{
    GcpHttpClient, Result,
    ops::bigquery::BigqueryOps,
    types::bigquery::{
        Dataset, DatasetListItem, Job, JobCancelResponse, JobListItem, QueryRequest, QueryResponse,
        Table, TableListItem,
    },
};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

/// Options for listing BigQuery jobs.
#[derive(Debug, Clone, Default)]
pub struct ListJobsOptions<'a> {
    /// Whether to display jobs owned by all users in the project.
    pub all_users: Option<bool>,
    /// Restrict information returned to a set of selected fields.
    pub projection: Option<&'a str>,
    /// Filter by job state. Accepts multiple values (repeated param).
    /// Valid values: "done", "pending", "running".
    pub state_filters: Option<&'a [&'a str]>,
    /// If set, show only child jobs of the specified parent.
    pub parent_job_id: Option<&'a str>,
    /// Min value for Job.statistics.creationTime (milliseconds since epoch).
    pub min_creation_time: Option<&'a str>,
    /// Max value for Job.statistics.creationTime (milliseconds since epoch).
    pub max_creation_time: Option<&'a str>,
}

// ── Reservation API Types (not in BigQuery v2 discovery doc) ────────

/// A capacity commitment for BigQuery reservations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapacityCommitment {
    /// Output only. Resource name (projects/{project}/locations/{location}/capacityCommitments/{id}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of slots in this commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_count: Option<i64>,

    /// Capacity commitment plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CommitmentPlan>,

    /// Output only. State of the commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CommitmentState>,

    /// Output only. Start of the current commitment period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_start_time: Option<String>,

    /// Output only. End of the current commitment period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_end_time: Option<String>,

    /// The plan this capacity commitment converts to after commitment_end_time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_plan: Option<CommitmentPlan>,

    /// Applicable only for commitments in BigQuery multi-regions (US or EU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_auxiliary: Option<bool>,

    /// Edition of the capacity commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
}

/// Commitment plan type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommitmentPlan {
    /// Unspecified plan.
    CommitmentPlanUnspecified,
    /// Flex plan (per-second billing, can be deleted anytime).
    Flex,
    /// Flex flat-rate plan.
    #[serde(rename = "FLEX_FLAT_RATE")]
    FlexFlatRate,
    /// Trial plan.
    Trial,
    /// Monthly plan (30-day commitment).
    Monthly,
    /// Monthly flat-rate plan.
    #[serde(rename = "MONTHLY_FLAT_RATE")]
    MonthlyFlatRate,
    /// Annual plan (365-day commitment).
    Annual,
    /// Annual flat-rate plan.
    #[serde(rename = "ANNUAL_FLAT_RATE")]
    AnnualFlatRate,
    /// Three-year plan.
    ThreeYear,
    /// No plan (used for renewal_plan to indicate no renewal).
    None,
}

/// Commitment state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommitmentState {
    /// Unspecified state.
    StateUnspecified,
    /// Commitment is pending activation.
    Pending,
    /// Commitment is active.
    Active,
    /// Commitment creation failed.
    Failed,
}

/// Request to create a capacity commitment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCapacityCommitmentRequest {
    /// Number of slots in this commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_count: Option<i64>,

    /// Capacity commitment plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CommitmentPlan>,

    /// The plan this commitment converts to after commitment_end_time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_plan: Option<CommitmentPlan>,

    /// If true, fail if another project in the org has a capacity commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_single_admin_project_per_org: Option<bool>,

    /// For commitments in BigQuery multi-regions (US or EU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_auxiliary: Option<bool>,

    /// Edition of the capacity commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapacityCommitmentListResponse {
    #[serde(default)]
    capacity_commitments: Vec<CapacityCommitment>,
    next_page_token: Option<String>,
}

/// Client for the BigQuery API
pub struct BigqueryClient<'a> {
    ops: BigqueryOps<'a>,
}

impl<'a> BigqueryClient<'a> {
    /// Create a new BigQuery API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: BigqueryOps::new(client),
        }
    }

    // ── Datasets ──────────────────────────────────────────────────────

    /// Get a dataset by ID.
    pub async fn get_dataset(&self, project: &str, dataset_id: &str) -> Result<Dataset> {
        self.ops.get_dataset(project, dataset_id).await
    }

    /// List all datasets in a project. Auto-paginates to collect all results.
    pub async fn list_datasets(&self, project: &str) -> Result<Vec<DatasetListItem>> {
        let mut all_items = Vec::new();
        let mut page_token = String::new();

        loop {
            let response = self
                .ops
                .list_datasets(project, "", "", "", &page_token)
                .await?;
            all_items.extend(response.datasets);
            match response.next_page_token {
                Some(token) if !token.is_empty() => page_token = token,
                _ => break,
            }
        }

        Ok(all_items)
    }

    /// List datasets with a label filter. Auto-paginates to collect all results.
    pub async fn list_datasets_with_filter(
        &self,
        project: &str,
        filter: &str,
    ) -> Result<Vec<DatasetListItem>> {
        let mut all_items = Vec::new();
        let mut page_token = String::new();

        loop {
            let response = self
                .ops
                .list_datasets(project, "", filter, "", &page_token)
                .await?;
            all_items.extend(response.datasets);
            match response.next_page_token {
                Some(token) if !token.is_empty() => page_token = token,
                _ => break,
            }
        }

        Ok(all_items)
    }

    /// Stream all datasets in a project, automatically handling pagination.
    pub fn list_datasets_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<DatasetListItem>> + '_ {
        let project = project.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_datasets(&project, "", "", "", page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.datasets { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create a new dataset.
    pub async fn create_dataset(&self, project: &str, body: &Dataset) -> Result<Dataset> {
        self.ops.create_dataset(project, body).await
    }

    /// Delete a dataset. Fails if the dataset contains tables unless `delete_contents` is true.
    pub async fn delete_dataset(&self, project: &str, dataset_id: &str) -> Result<()> {
        self.ops.delete_dataset(project, dataset_id, "").await
    }

    /// Delete a dataset and all of its contents (tables, views, etc.).
    pub async fn delete_dataset_with_contents(
        &self,
        project: &str,
        dataset_id: &str,
    ) -> Result<()> {
        self.ops.delete_dataset(project, dataset_id, "true").await
    }

    /// Patch a dataset (partial update using patch semantics).
    pub async fn patch_dataset(
        &self,
        project: &str,
        dataset_id: &str,
        body: &Dataset,
    ) -> Result<Dataset> {
        self.ops.patch_dataset(project, dataset_id, body).await
    }

    // ── Dataset IAM / ACL ─────────────────────────────────────────────

    /// Return the access control list (ACL) for a dataset.
    ///
    /// BigQuery datasets use an `access` array for ACL management instead of
    /// the standard `getIamPolicy`/`setIamPolicy` endpoints (which require
    /// allowlisting). Each entry may contain fields like `role`, `userByEmail`,
    /// `groupByEmail`, `specialGroup`, `iamMember`, `view`, `routine`, etc.
    pub async fn get_dataset_iam_policy(
        &self,
        project: &str,
        dataset_id: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let dataset = self.ops.get_dataset(project, dataset_id).await?;
        Ok(dataset.access)
    }

    /// Replace the access control list (ACL) for a dataset.
    ///
    /// The `access` array must be the **complete** desired ACL — BigQuery
    /// replaces the existing list entirely (it does not merge). To add a
    /// single entry, first call `get_dataset_iam_policy`, append your entry,
    /// then pass the result here.
    ///
    /// Returns the updated ACL as stored by BigQuery after the patch.
    pub async fn set_dataset_iam_policy(
        &self,
        project: &str,
        dataset_id: &str,
        access: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        let body = Dataset {
            access,
            ..Default::default()
        };
        let updated = self.ops.patch_dataset(project, dataset_id, &body).await?;
        Ok(updated.access)
    }

    // ── Tables ────────────────────────────────────────────────────────

    /// Get a table by ID.
    pub async fn get_table(
        &self,
        project: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<Table> {
        self.ops.get_table(project, dataset_id, table_id).await
    }

    /// List all tables in a dataset. Auto-paginates to collect all results.
    pub async fn list_tables(&self, project: &str, dataset_id: &str) -> Result<Vec<TableListItem>> {
        let mut all_items = Vec::new();
        let mut page_token = String::new();

        loop {
            let response = self
                .ops
                .list_tables(project, dataset_id, "", &page_token)
                .await?;
            all_items.extend(response.tables);
            match response.next_page_token {
                Some(token) if !token.is_empty() => page_token = token,
                _ => break,
            }
        }

        Ok(all_items)
    }

    /// Stream all tables in a dataset, automatically handling pagination.
    pub fn list_tables_stream(
        &self,
        project: &str,
        dataset_id: &str,
    ) -> impl futures::Stream<Item = Result<TableListItem>> + '_ {
        let project = project.to_string();
        let dataset_id = dataset_id.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_tables(&project, &dataset_id, "", page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.tables { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create a new table in a dataset.
    pub async fn create_table(
        &self,
        project: &str,
        dataset_id: &str,
        body: &Table,
    ) -> Result<Table> {
        self.ops.create_table(project, dataset_id, body).await
    }

    /// Delete a table.
    pub async fn delete_table(
        &self,
        project: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<()> {
        self.ops.delete_table(project, dataset_id, table_id).await
    }

    /// Patch a table (partial update using patch semantics).
    pub async fn patch_table(
        &self,
        project: &str,
        dataset_id: &str,
        table_id: &str,
        body: &Table,
    ) -> Result<Table> {
        self.ops
            .patch_table(project, dataset_id, table_id, body)
            .await
    }

    // ── Jobs ──────────────────────────────────────────────────────────

    /// Run a synchronous SQL query.
    pub async fn query(&self, project: &str, body: &QueryRequest) -> Result<QueryResponse> {
        self.ops.query_jobs(project, body).await
    }

    /// Insert (start) an asynchronous job.
    pub async fn insert_job(&self, project: &str, body: &Job) -> Result<Job> {
        self.ops.insert_job(project, body).await
    }

    /// Get a job by ID.
    pub async fn get_job(&self, project: &str, job_id: &str) -> Result<Job> {
        self.ops.get_job(project, job_id, "").await
    }

    /// Get a job by ID with an explicit location.
    pub async fn get_job_with_location(
        &self,
        project: &str,
        job_id: &str,
        location: &str,
    ) -> Result<Job> {
        self.ops.get_job(project, job_id, location).await
    }

    /// List all jobs in a project. Auto-paginates to collect all results.
    pub async fn list_jobs(&self, project: &str) -> Result<Vec<JobListItem>> {
        self.list_jobs_with_options(project, &ListJobsOptions::default())
            .await
    }

    /// List all jobs in a project with filtering options. Auto-paginates.
    ///
    /// # Arguments
    ///
    /// * `project` - Project ID
    /// * `options` - Filtering options (state filters, parent job, etc.)
    ///
    /// # Note
    ///
    /// The `stateFilter` parameter is `repeated` in the GCP API, meaning
    /// filtering by multiple states requires repeated query params
    /// (`?stateFilter=done&stateFilter=running`). This method handles that
    /// correctly by building the URL manually.
    pub async fn list_jobs_with_options(
        &self,
        project: &str,
        options: &ListJobsOptions<'_>,
    ) -> Result<Vec<JobListItem>> {
        let all_users_str = options
            .all_users
            .filter(|&b| b)
            .map(|_| "true".to_string())
            .unwrap_or_default();
        let state_filters = options.state_filters.unwrap_or(&[]);

        let mut all_items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let response = self
                .ops
                .list_jobs(
                    project,
                    &all_users_str,
                    state_filters,
                    "",
                    page_token.as_deref().unwrap_or(""),
                    options.projection.unwrap_or(""),
                    options.parent_job_id.unwrap_or(""),
                    options.min_creation_time.unwrap_or(""),
                    options.max_creation_time.unwrap_or(""),
                )
                .await?;
            all_items.extend(response.jobs);
            match response.next_page_token {
                Some(token) if !token.is_empty() => page_token = Some(token),
                _ => break,
            }
        }

        Ok(all_items)
    }

    /// Stream all jobs in a project, automatically handling pagination.
    pub fn list_jobs_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<JobListItem>> + '_ {
        self.list_jobs_stream_with_options(project, ListJobsOptions::default())
    }

    /// Stream all jobs in a project with filtering options.
    pub fn list_jobs_stream_with_options(
        &self,
        project: &str,
        options: ListJobsOptions<'_>,
    ) -> impl futures::Stream<Item = Result<JobListItem>> + '_ {
        let project = project.to_string();
        let all_users_str = options
            .all_users
            .filter(|&b| b)
            .map(|_| "true".to_string())
            .unwrap_or_default();
        let state_filters: Vec<String> = options
            .state_filters
            .unwrap_or(&[])
            .iter()
            .map(|s| s.to_string())
            .collect();
        let projection = options.projection.unwrap_or("").to_string();
        let parent_job_id = options.parent_job_id.unwrap_or("").to_string();
        let min_creation_time = options.min_creation_time.unwrap_or("").to_string();
        let max_creation_time = options.max_creation_time.unwrap_or("").to_string();

        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let sf_refs: Vec<&str> = state_filters.iter().map(|s| s.as_str()).collect();
                let response = self
                    .ops
                    .list_jobs(
                        &project,
                        &all_users_str,
                        &sf_refs,
                        "",
                        page_token.as_deref().unwrap_or(""),
                        &projection,
                        &parent_job_id,
                        &min_creation_time,
                        &max_creation_time,
                    )
                    .await?;
                for item in response.jobs { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Cancel a running job. Returns the final job state.
    pub async fn cancel_job(&self, project: &str, job_id: &str) -> Result<JobCancelResponse> {
        self.ops.cancel_job(project, job_id, "").await
    }

    /// Cancel a running job with an explicit location. Returns the final job state.
    pub async fn cancel_job_with_location(
        &self,
        project: &str,
        job_id: &str,
        location: &str,
    ) -> Result<JobCancelResponse> {
        self.ops.cancel_job(project, job_id, location).await
    }

    /// Delete a job's metadata.
    pub async fn delete_job(&self, project: &str, job_id: &str, location: &str) -> Result<()> {
        self.ops.delete_job(project, job_id, location).await
    }

    // ── Capacity Commitments (Reservation API) ───────────────────────

    /// List all capacity commitments in a project location. Auto-paginates.
    pub async fn list_capacity_commitments(
        &self,
        project: &str,
        location: &str,
    ) -> Result<Vec<CapacityCommitment>> {
        let mut all = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut url = format!(
                "{}/v1/projects/{}/locations/{}/capacityCommitments",
                self.reservation_base_url(),
                encode(project),
                encode(location),
            );

            if let Some(ref token) = page_token {
                url.push_str(&format!("?pageToken={}", encode(token)));
            }

            let response = self.ops.client.get(&url).await?;
            let list: CapacityCommitmentListResponse =
                serde_json::from_slice(&response).map_err(|e| {
                    crate::GcpError::InvalidResponse {
                        message: format!("Failed to parse capacity commitments list: {e}"),
                        body: Some(String::from_utf8_lossy(&response).to_string()),
                    }
                })?;

            all.extend(list.capacity_commitments);

            match list.next_page_token {
                Some(token) if !token.is_empty() => page_token = Some(token),
                _ => break,
            }
        }

        Ok(all)
    }

    /// Create a capacity commitment in a project location.
    pub async fn create_capacity_commitment(
        &self,
        project: &str,
        location: &str,
        commitment: &CreateCapacityCommitmentRequest,
    ) -> Result<CapacityCommitment> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/capacityCommitments",
            self.reservation_base_url(),
            encode(project),
            encode(location),
        );

        let response = self.ops.client.post(&url, commitment).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse capacity commitment: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Delete a capacity commitment.
    pub async fn delete_capacity_commitment(
        &self,
        project: &str,
        location: &str,
        commitment_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/capacityCommitments/{}",
            self.reservation_base_url(),
            encode(project),
            encode(location),
            encode(commitment_id),
        );

        self.ops.client.delete(&url).await?;
        Ok(())
    }

    // ── Helpers ───────────────────────────────────────────────────────

    /// Base URL for the BigQuery Reservation API.
    fn reservation_base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.ops.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://bigqueryreservation.googleapis.com"
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    // ── Dataset Tests ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets/my_dataset")
            .returning_json(json!({
                "datasetReference": {
                    "datasetId": "my_dataset",
                    "projectId": "test-project"
                },
                "id": "test-project:my_dataset",
                "kind": "bigquery#dataset",
                "location": "US",
                "friendlyName": "My Dataset"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .get_dataset("test-project", "my_dataset")
            .await;
        assert!(result.is_ok());
        let ds = result.unwrap();
        assert_eq!(ds.id, Some("test-project:my_dataset".to_string()));
        assert_eq!(ds.location, Some("US".to_string()));
        assert_eq!(ds.friendly_name, Some("My Dataset".to_string()));

        let ds_ref = ds.dataset_reference.unwrap();
        assert_eq!(ds_ref.dataset_id, "my_dataset");
        assert_eq!(ds_ref.project_id, Some("test-project".to_string()));
    }

    #[tokio::test]
    async fn test_list_datasets_single_page() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets")
            .returning_json(json!({
                "datasets": [
                    {
                        "datasetReference": { "datasetId": "ds1", "projectId": "test-project" },
                        "id": "test-project:ds1",
                        "location": "US"
                    },
                    {
                        "datasetReference": { "datasetId": "ds2", "projectId": "test-project" },
                        "id": "test-project:ds2",
                        "location": "EU"
                    }
                ],
                "kind": "bigquery#datasetList"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.bigquery().list_datasets("test-project").await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, Some("test-project:ds1".to_string()));
        assert_eq!(items[1].id, Some("test-project:ds2".to_string()));
    }

    #[tokio::test]
    async fn test_list_datasets_auto_paginates() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets")
            .returning_json_sequence(vec![
                json!({
                    "datasets": [
                        {
                            "datasetReference": { "datasetId": "ds1", "projectId": "test-project" },
                            "id": "test-project:ds1"
                        }
                    ],
                    "nextPageToken": "page2"
                }),
                json!({
                    "datasets": [
                        {
                            "datasetReference": { "datasetId": "ds2", "projectId": "test-project" },
                            "id": "test-project:ds2"
                        }
                    ]
                }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.bigquery().list_datasets("test-project").await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, Some("test-project:ds1".to_string()));
        assert_eq!(items[1].id, Some("test-project:ds2".to_string()));
    }

    #[tokio::test]
    async fn test_create_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-project/datasets")
            .returning_json(json!({
                "datasetReference": {
                    "datasetId": "new_dataset",
                    "projectId": "test-project"
                },
                "id": "test-project:new_dataset",
                "kind": "bigquery#dataset",
                "location": "US"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::bigquery::Dataset {
            dataset_reference: Some(crate::types::bigquery::DatasetReference {
                dataset_id: "new_dataset".to_string(),
                ..Default::default()
            }),
            location: Some("US".to_string()),
            ..Default::default()
        };
        let ds = client
            .bigquery()
            .create_dataset("test-project", &body)
            .await
            .unwrap();
        assert_eq!(ds.id, Some("test-project:new_dataset".to_string()));
        assert_eq!(ds.location, Some("US".to_string()));
    }

    #[tokio::test]
    async fn test_delete_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/bigquery/v2/projects/test-project/datasets/old_dataset")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .delete_dataset("test-project", "old_dataset")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_dataset_with_contents() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/bigquery/v2/projects/test-project/datasets/old_dataset?deleteContents=true",
        )
        .returning_json(json!({}))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .delete_dataset_with_contents("test-project", "old_dataset")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/bigquery/v2/projects/test-project/datasets/my_dataset")
            .returning_json(json!({
                "datasetReference": {
                    "datasetId": "my_dataset",
                    "projectId": "test-project"
                },
                "description": "Updated",
                "friendlyName": "My Updated Dataset"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::bigquery::Dataset {
            description: Some("Updated".to_string()),
            friendly_name: Some("My Updated Dataset".to_string()),
            ..Default::default()
        };
        let ds = client
            .bigquery()
            .patch_dataset("test-project", "my_dataset", &body)
            .await
            .unwrap();
        assert_eq!(ds.description, Some("Updated".to_string()));
        assert_eq!(ds.friendly_name, Some("My Updated Dataset".to_string()));
    }

    // ── Dataset IAM Tests ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_dataset_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets/my_dataset")
            .returning_json(json!({
                "datasetReference": { "datasetId": "my_dataset", "projectId": "test-project" },
                "id": "test-project:my_dataset",
                "access": [
                    { "role": "OWNER", "specialGroup": "projectOwners" },
                    { "role": "WRITER", "specialGroup": "projectWriters" },
                    { "role": "READER", "specialGroup": "projectReaders" },
                    { "role": "OWNER", "userByEmail": "owner@example.com" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let acl = client
            .bigquery()
            .get_dataset_iam_policy("test-project", "my_dataset")
            .await
            .unwrap();
        assert_eq!(acl.len(), 4);
        assert_eq!(
            acl[0].get("specialGroup").and_then(|v| v.as_str()),
            Some("projectOwners")
        );
        assert_eq!(acl[0].get("role").and_then(|v| v.as_str()), Some("OWNER"));
    }

    #[tokio::test]
    async fn test_set_dataset_iam_policy() {
        let mut mock = crate::MockClient::new();

        // set_dataset_iam_policy calls patch_dataset (PATCH) with the access array
        mock.expect_patch("/bigquery/v2/projects/test-project/datasets/my_dataset")
            .returning_json(json!({
                "datasetReference": { "datasetId": "my_dataset", "projectId": "test-project" },
                "id": "test-project:my_dataset",
                "access": [
                    { "role": "OWNER", "specialGroup": "projectOwners" },
                    { "role": "WRITER", "specialGroup": "projectWriters" },
                    { "role": "READER", "specialGroup": "allAuthenticatedUsers" }
                ]
            }))
            .times(1);

        let new_acl = vec![
            json!({ "role": "OWNER", "specialGroup": "projectOwners" }),
            json!({ "role": "WRITER", "specialGroup": "projectWriters" }),
            json!({ "role": "READER", "specialGroup": "allAuthenticatedUsers" }),
        ];

        let client = crate::GcpHttpClient::from_mock(mock);
        let updated = client
            .bigquery()
            .set_dataset_iam_policy("test-project", "my_dataset", new_acl)
            .await
            .unwrap();
        assert_eq!(updated.len(), 3);
        let has_public = updated.iter().any(|e| {
            e.get("specialGroup").and_then(|v| v.as_str()) == Some("allAuthenticatedUsers")
        });
        assert!(
            has_public,
            "Updated ACL should contain allAuthenticatedUsers entry"
        );
    }

    // ── Table Tests ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets/my_dataset/tables/my_table")
            .returning_json(json!({
                "tableReference": {
                    "projectId": "test-project",
                    "datasetId": "my_dataset",
                    "tableId": "my_table"
                },
                "id": "test-project:my_dataset.my_table",
                "kind": "bigquery#table",
                "type": "TABLE",
                "numRows": "42",
                "numBytes": "1024"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .get_table("test-project", "my_dataset", "my_table")
            .await;
        assert!(result.is_ok());
        let table = result.unwrap();
        assert_eq!(
            table.id,
            Some("test-project:my_dataset.my_table".to_string())
        );
        assert_eq!(table.type_value, Some("TABLE".to_string()));
        assert_eq!(table.num_rows, Some("42".to_string()));

        let tref = table.table_reference.unwrap();
        assert_eq!(tref.project_id, "test-project");
        assert_eq!(tref.dataset_id, "my_dataset");
        assert_eq!(tref.table_id, "my_table");
    }

    #[tokio::test]
    async fn test_list_tables_single_page() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/datasets/my_dataset/tables")
            .returning_json(json!({
                "tables": [
                    {
                        "tableReference": {
                            "projectId": "test-project",
                            "datasetId": "my_dataset",
                            "tableId": "t1"
                        },
                        "id": "test-project:my_dataset.t1",
                        "type": "TABLE"
                    },
                    {
                        "tableReference": {
                            "projectId": "test-project",
                            "datasetId": "my_dataset",
                            "tableId": "t2"
                        },
                        "id": "test-project:my_dataset.t2",
                        "type": "VIEW"
                    }
                ],
                "totalItems": 2
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .list_tables("test-project", "my_dataset")
            .await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, Some("test-project:my_dataset.t1".to_string()));
        assert_eq!(items[1].type_value, Some("VIEW".to_string()));
    }

    #[tokio::test]
    async fn test_patch_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/bigquery/v2/projects/test-project/datasets/my_dataset/tables/my_table")
            .returning_json(json!({
                "tableReference": {
                    "projectId": "test-project",
                    "datasetId": "my_dataset",
                    "tableId": "my_table"
                },
                "description": "Updated description",
                "friendlyName": "Updated Name"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::bigquery::Table {
            description: Some("Updated description".to_string()),
            friendly_name: Some("Updated Name".to_string()),
            ..Default::default()
        };
        let result = client
            .bigquery()
            .patch_table("test-project", "my_dataset", "my_table", &body)
            .await;
        assert!(result.is_ok());
        let table = result.unwrap();
        assert_eq!(table.description, Some("Updated description".to_string()));
        assert_eq!(table.friendly_name, Some("Updated Name".to_string()));
    }

    #[tokio::test]
    async fn test_create_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-project/datasets/my_dataset/tables")
            .returning_json(json!({
                "tableReference": {
                    "projectId": "test-project",
                    "datasetId": "my_dataset",
                    "tableId": "new_table"
                },
                "id": "test-project:my_dataset.new_table",
                "type": "TABLE"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::bigquery::Table {
            table_reference: Some(crate::types::bigquery::TableReference {
                project_id: "test-project".to_string(),
                dataset_id: "my_dataset".to_string(),
                table_id: "new_table".to_string(),
            }),
            ..Default::default()
        };
        let table = client
            .bigquery()
            .create_table("test-project", "my_dataset", &body)
            .await
            .unwrap();
        assert_eq!(
            table.id,
            Some("test-project:my_dataset.new_table".to_string())
        );
    }

    #[tokio::test]
    async fn test_delete_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/bigquery/v2/projects/test-project/datasets/my_dataset/tables/old_table",
        )
        .returning_json(json!({}))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .delete_table("test-project", "my_dataset", "old_table")
            .await;
        assert!(result.is_ok());
    }

    // ── Job Tests ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_query() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-project/queries")
            .returning_json(json!({
                "jobComplete": true,
                "kind": "bigquery#queryResponse",
                "rows": [
                    { "f": [{ "v": "hello" }, { "v": "42" }] }
                ],
                "schema": {
                    "fields": [
                        { "name": "name", "type": "STRING" },
                        { "name": "count", "type": "INTEGER" }
                    ]
                },
                "totalRows": "1",
                "totalBytesProcessed": "0"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let req = crate::types::bigquery::QueryRequest {
            query: "SELECT 'hello' as name, 42 as count".to_string(),
            use_legacy_sql: Some(false),
            ..Default::default()
        };
        let result = client.bigquery().query("test-project", &req).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.job_complete, Some(true));
        assert_eq!(resp.total_rows, Some("1".to_string()));
        assert_eq!(resp.rows.len(), 1);
    }

    #[tokio::test]
    async fn test_insert_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-project/jobs")
            .returning_json(json!({
                "jobReference": {
                    "projectId": "test-project",
                    "jobId": "job_abc123",
                    "location": "US"
                },
                "status": { "state": "RUNNING" },
                "kind": "bigquery#job"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let job = crate::types::bigquery::Job {
            configuration: Some(crate::types::bigquery::JobConfiguration {
                query: Some(crate::types::bigquery::JobConfigurationQuery {
                    query: "SELECT 1".to_string(),
                    use_legacy_sql: Some(false),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let result = client.bigquery().insert_job("test-project", &job).await;
        assert!(result.is_ok());
        let inserted = result.unwrap();
        let job_ref = inserted.job_reference.unwrap();
        assert_eq!(job_ref.job_id, "job_abc123");

        let status = inserted.status.unwrap();
        assert_eq!(status.state, Some("RUNNING".to_string()));
    }

    #[tokio::test]
    async fn test_get_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/jobs/job_abc123")
            .returning_json(json!({
                "jobReference": {
                    "projectId": "test-project",
                    "jobId": "job_abc123",
                    "location": "US"
                },
                "status": { "state": "DONE" },
                "statistics": {
                    "creationTime": "1700000000000",
                    "startTime": "1700000001000",
                    "endTime": "1700000002000",
                    "totalBytesProcessed": "1024"
                },
                "kind": "bigquery#job"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .get_job("test-project", "job_abc123")
            .await;
        assert!(result.is_ok());
        let job = result.unwrap();
        let status = job.status.unwrap();
        assert_eq!(status.state, Some("DONE".to_string()));

        let stats = job.statistics.unwrap();
        assert_eq!(stats.total_bytes_processed, Some("1024".to_string()));
    }

    #[tokio::test]
    async fn test_get_job_with_location() {
        let mut mock = crate::MockClient::new();

        // With location, the URL should have ?location=EU
        mock.expect_get("/bigquery/v2/projects/test-project/jobs/job_xyz")
            .returning_json(json!({
                "jobReference": {
                    "projectId": "test-project",
                    "jobId": "job_xyz",
                    "location": "EU"
                },
                "status": { "state": "DONE" }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .get_job_with_location("test-project", "job_xyz", "EU")
            .await;
        assert!(result.is_ok());
        let job = result.unwrap();
        let job_ref = job.job_reference.unwrap();
        assert_eq!(job_ref.location, Some("EU".to_string()));
    }

    #[tokio::test]
    async fn test_list_jobs_single_page() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/jobs")
            .returning_json(json!({
                "jobs": [
                    {
                        "jobReference": {
                            "projectId": "test-project",
                            "jobId": "j1"
                        },
                        "state": "DONE",
                        "id": "test-project:US.j1"
                    },
                    {
                        "jobReference": {
                            "projectId": "test-project",
                            "jobId": "j2"
                        },
                        "state": "RUNNING",
                        "id": "test-project:US.j2"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.bigquery().list_jobs("test-project").await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].state, Some("DONE".to_string()));
        assert_eq!(items[1].state, Some("RUNNING".to_string()));
    }

    #[tokio::test]
    async fn test_list_jobs_auto_paginates() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-project/jobs")
            .returning_json_sequence(vec![
                json!({
                    "jobs": [{ "id": "j1", "state": "DONE" }],
                    "nextPageToken": "page2"
                }),
                json!({
                    "jobs": [{ "id": "j2", "state": "DONE" }]
                }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.bigquery().list_jobs("test-project").await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
    }

    #[tokio::test]
    async fn test_list_datasets_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/bigquery/v2/projects/test-project/datasets")
            .returning_json(json!({"kind": "bigquery#datasetList"}))
            .times(1);
        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client
            .bigquery()
            .list_datasets("test-project")
            .await
            .unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_list_tables_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/bigquery/v2/projects/test-project/datasets/ds1/tables")
            .returning_json(json!({"kind": "bigquery#tableList"}))
            .times(1);
        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client
            .bigquery()
            .list_tables("test-project", "ds1")
            .await
            .unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_list_jobs_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/bigquery/v2/projects/test-project/jobs")
            .returning_json(json!({"kind": "bigquery#jobList"}))
            .times(1);
        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client.bigquery().list_jobs("test-project").await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_cancel_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-project/jobs/job_abc/cancel")
            .returning_json(json!({
                "job": {
                    "jobReference": {
                        "projectId": "test-project",
                        "jobId": "job_abc"
                    },
                    "status": { "state": "DONE" }
                },
                "kind": "bigquery#jobCancelResponse"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let resp = client
            .bigquery()
            .cancel_job("test-project", "job_abc")
            .await
            .unwrap();
        assert_eq!(resp.kind, Some("bigquery#jobCancelResponse".to_string()));
        let job = resp.job.unwrap();
        let status = job.status.unwrap();
        assert_eq!(status.state, Some("DONE".to_string()));
    }

    #[tokio::test]
    async fn test_delete_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/bigquery/v2/projects/test-project/jobs/job_old/delete?location=US")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .delete_job("test-project", "job_old", "US")
            .await;
        assert!(result.is_ok());
    }

    // ── Capacity Commitment Tests ────────────────────────────────────

    #[tokio::test]
    async fn test_list_capacity_commitments() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/US/capacityCommitments")
            .returning_json(json!({
                "capacityCommitments": [
                    {
                        "name": "projects/test-project/locations/US/capacityCommitments/123",
                        "slotCount": 100,
                        "plan": "FLEX",
                        "state": "ACTIVE"
                    },
                    {
                        "name": "projects/test-project/locations/US/capacityCommitments/456",
                        "slotCount": 200,
                        "plan": "MONTHLY",
                        "state": "ACTIVE"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client
            .bigquery()
            .list_capacity_commitments("test-project", "US")
            .await
            .unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].slot_count, Some(100));
        assert_eq!(items[0].plan, Some(super::CommitmentPlan::Flex));
        assert_eq!(items[1].slot_count, Some(200));
        assert_eq!(items[1].plan, Some(super::CommitmentPlan::Monthly));
    }

    #[tokio::test]
    async fn test_list_capacity_commitments_with_pagination() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/US/capacityCommitments")
            .returning_json_sequence(vec![
                json!({
                    "capacityCommitments": [
                        {
                            "name": "projects/test-project/locations/US/capacityCommitments/123",
                            "slotCount": 100,
                            "plan": "FLEX",
                            "state": "ACTIVE"
                        }
                    ],
                    "nextPageToken": "page2"
                }),
                json!({
                    "capacityCommitments": [
                        {
                            "name": "projects/test-project/locations/US/capacityCommitments/456",
                            "slotCount": 200,
                            "plan": "ANNUAL",
                            "state": "PENDING"
                        }
                    ]
                }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client
            .bigquery()
            .list_capacity_commitments("test-project", "US")
            .await
            .unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].slot_count, Some(100));
        assert_eq!(items[1].plan, Some(super::CommitmentPlan::Annual));
        assert_eq!(items[1].state, Some(super::CommitmentState::Pending));
    }

    #[tokio::test]
    async fn test_list_capacity_commitments_empty() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/test-project/locations/EU/capacityCommitments")
            .returning_json(json!({}))
            .times(1);
        let client = crate::GcpHttpClient::from_mock(mock);
        let items = client
            .bigquery()
            .list_capacity_commitments("test-project", "EU")
            .await
            .unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_create_capacity_commitment() {
        use super::{CommitmentPlan, CreateCapacityCommitmentRequest};

        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/locations/US/capacityCommitments")
            .returning_json(json!({
                "name": "projects/test-project/locations/US/capacityCommitments/789",
                "slotCount": 100,
                "plan": "FLEX",
                "state": "ACTIVE",
                "edition": "ENTERPRISE"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let req = CreateCapacityCommitmentRequest {
            slot_count: Some(100),
            plan: Some(CommitmentPlan::Flex),
            edition: Some("ENTERPRISE".to_string()),
            ..Default::default()
        };
        let created = client
            .bigquery()
            .create_capacity_commitment("test-project", "US", &req)
            .await
            .unwrap();
        assert_eq!(created.slot_count, Some(100));
        assert_eq!(created.plan, Some(CommitmentPlan::Flex));
        assert_eq!(created.state, Some(super::CommitmentState::Active));
        assert_eq!(created.edition, Some("ENTERPRISE".to_string()));
    }

    #[tokio::test]
    async fn test_delete_capacity_commitment() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/v1/projects/test-project/locations/US/capacityCommitments/commitment-123",
        )
        .returning_json(json!({}))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .bigquery()
            .delete_capacity_commitment("test-project", "US", "commitment-123")
            .await;
        assert!(result.is_ok());
    }
}
