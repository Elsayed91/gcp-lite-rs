//! Operation contracts for the Cloud Scheduler API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudscheduler.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudscheduler::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud Scheduler API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudscheduler::CloudschedulerClient`] instead.
pub struct CloudschedulerOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> CloudschedulerOps<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://cloudscheduler.googleapis.com"
    }

    /// Creates a job.
    ///
    /// **GCP API**: `POST v1/{+parent}/jobs`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`. *(required)*
    ///
    /// # Request Body
    /// [`Job`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn create_job(&self, parent: &str, body: &Job) -> Result<Job> {
        let url = format!("{}/v1/{}/jobs", self.base_url(), parent,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a job.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. *(required)*
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn get_job(&self, name: &str) -> Result<Job> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists jobs.
    ///
    /// **GCP API**: `GET v1/{+parent}/jobs`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`. *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Requested page size. The maximum page size is 500. If unspecified, the page size will be the maximum. Fewer jobs than re
    /// - `pageToken` — A token identifying a page of results the server will return. To request the first page results, page_token must be empt
    ///
    /// # Response
    /// [`ListJobsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_jobs(&self, parent: &str) -> Result<ListJobsResponse> {
        let url = format!("{}/v1/{}/jobs", self.base_url(), parent,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_jobs response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates a job. If successful, the updated Job is returned. If the job does not exist,
    /// `NOT_FOUND` is returned. If UpdateJob does not successfully return, it is possible for
    /// the job to be in an Job.State.UPDATE_FAILED state. A job in this state may not be
    /// executed. If this happens, retry the UpdateJob request until a successful response is
    /// received.
    ///
    /// **GCP API**: `PATCH v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Optionally caller-specified in CreateJob, after which it becomes output only. The job name. For example: `projects/PROJE *(required)*
    ///
    /// # Query Parameters
    /// - `updateMask` — A mask used to specify which fields of the job are being updated.
    ///
    /// # Request Body
    /// [`Job`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn update_job(
        &self,
        name: &str,
        update_mask: &str,
        body: &Job,
    ) -> Result<Job> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let url = crate::append_query_params(url, &[("updateMask", update_mask)]);
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a job.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. *(required)*
    ///
    /// # Response
    /// [`SchedulerEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_job(&self, name: &str) -> Result<SchedulerEmpty> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Pauses a job. If a job is paused then the system will stop executing the job until it is
    /// re-enabled via ResumeJob. The state of the job is stored in state; if paused it will be
    /// set to Job.State.PAUSED. A job must be in Job.State.ENABLED to be paused.
    ///
    /// **GCP API**: `POST v1/{+name}:pause`
    ///
    /// # Path Parameters
    /// - `name` — Required. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. *(required)*
    ///
    /// # Request Body
    /// [`PauseJobRequest`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn pause_job(&self, name: &str, body: &PauseJobRequest) -> Result<Job> {
        let url = format!("{}/v1/{}:pause", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse pause_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Resume a job. This method reenables a job after it has been Job.State.PAUSED. The state
    /// of a job is stored in Job.state; after calling this method it will be set to
    /// Job.State.ENABLED. A job must be in Job.State.PAUSED to be resumed.
    ///
    /// **GCP API**: `POST v1/{+name}:resume`
    ///
    /// # Path Parameters
    /// - `name` — Required. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. *(required)*
    ///
    /// # Request Body
    /// [`ResumeJobRequest`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn resume_job(&self, name: &str, body: &ResumeJobRequest) -> Result<Job> {
        let url = format!("{}/v1/{}:resume", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse resume_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Forces a job to run now. When this method is called, Cloud Scheduler will dispatch the
    /// job, even if the job is already running.
    ///
    /// **GCP API**: `POST v1/{+name}:run`
    ///
    /// # Path Parameters
    /// - `name` — Required. The job name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`. *(required)*
    ///
    /// # Request Body
    /// [`RunJobRequest`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn run_job(&self, name: &str, body: &RunJobRequest) -> Result<Job> {
        let url = format!("{}/v1/{}:run", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse run_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-parent/jobs")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let body = Job::fixture();
        let result = ops.create_job("test-parent", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let result = ops.get_job("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_jobs() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/jobs")
            .returning_json(serde_json::to_value(ListJobsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let result = ops.list_jobs("test-parent").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/v1/test-name?updateMask=test-updateMask")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let body = Job::fixture();
        let result = ops.update_job("test-name", "test-updateMask", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(SchedulerEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let result = ops.delete_job("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pause_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name:pause")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let body = PauseJobRequest::fixture();
        let result = ops.pause_job("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resume_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name:resume")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let body = ResumeJobRequest::fixture();
        let result = ops.resume_job("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name:run")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudschedulerOps::new(&client);

        let body = RunJobRequest::fixture();
        let result = ops.run_job("test-name", &body).await;
        assert!(result.is_ok());
    }
}
