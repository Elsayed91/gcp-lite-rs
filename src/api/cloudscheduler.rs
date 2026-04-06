//! Cloud Scheduler API client.
//!
//! Provides methods for managing scheduled jobs.

use crate::ops::cloudscheduler::CloudschedulerOps;
use crate::types::cloudscheduler::*;
use crate::{GcpHttpClient, Result};

/// Client for Cloud Scheduler API operations.
pub struct SchedulerClient<'a> {
    ops: CloudschedulerOps<'a>,
}

impl<'a> SchedulerClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: CloudschedulerOps::new(client),
        }
    }

    // ── Jobs CRUD ──────────────────────────────────────────────────────

    /// Create a new scheduled job.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job` - The job configuration
    pub async fn create_job(&self, project: &str, location: &str, job: &Job) -> Result<Job> {
        let parent = format!("projects/{}/locations/{}", project, location);
        self.ops.create_job(&parent, job).await
    }

    /// Get a job by name.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    pub async fn get_job(&self, project: &str, location: &str, job_id: &str) -> Result<Job> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.get_job(&name).await
    }

    /// List all jobs in a location.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    pub async fn list_jobs(&self, project: &str, location: &str) -> Result<Vec<Job>> {
        let parent = format!("projects/{}/locations/{}", project, location);
        let response = self.ops.list_jobs(&parent).await?;
        Ok(response.jobs)
    }

    /// Update a job.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    /// * `job` - The updated job configuration
    /// * `update_mask` - Field mask specifying which fields to update (e.g., "description,schedule")
    pub async fn update_job(
        &self,
        project: &str,
        location: &str,
        job_id: &str,
        job: &Job,
        update_mask: &str,
    ) -> Result<Job> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.update_job(&name, update_mask, job).await
    }

    /// Delete a job.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    pub async fn delete_job(&self, project: &str, location: &str, job_id: &str) -> Result<()> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.delete_job(&name).await?;
        Ok(())
    }

    // ── Jobs Actions ───────────────────────────────────────────────────

    /// Pause a job.
    ///
    /// When a job is paused, the system will stop executing it until it is resumed.
    /// A job must be in ENABLED state to be paused.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    pub async fn pause_job(&self, project: &str, location: &str, job_id: &str) -> Result<Job> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.pause_job(&name, &PauseJobRequest {}).await
    }

    /// Resume a paused job.
    ///
    /// This reenables a job after it has been paused.
    /// A job must be in PAUSED state to be resumed.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    pub async fn resume_job(&self, project: &str, location: &str, job_id: &str) -> Result<Job> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.resume_job(&name, &ResumeJobRequest {}).await
    }

    /// Force a job to run immediately.
    ///
    /// Cloud Scheduler will dispatch the job even if it is already running.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `job_id` - The job ID
    pub async fn run_job(&self, project: &str, location: &str, job_id: &str) -> Result<Job> {
        let name = format!(
            "projects/{}/locations/{}/jobs/{}",
            project, location, job_id
        );
        self.ops.run_job(&name, &RunJobRequest {}).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_job_crud_lifecycle() {
        let mut mock = crate::MockClient::new();

        // Create job
        mock.expect_post("/v1/projects/test-project/locations/us-central1/jobs")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/jobs/test-job",
                "description": "Test job",
                "schedule": "0 9 * * 1",
                "timeZone": "America/New_York"
            }))
            .times(1);

        // Get job
        mock.expect_get("/v1/projects/test-project/locations/us-central1/jobs/test-job")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/jobs/test-job",
                "description": "Test job",
                "schedule": "0 9 * * 1"
            }))
            .times(1);

        // List jobs
        mock.expect_get("/v1/projects/test-project/locations/us-central1/jobs")
            .returning_json(json!({
                "jobs": [{
                    "name": "projects/test-project/locations/us-central1/jobs/test-job",
                    "description": "Test job"
                }]
            }))
            .times(1);

        // Delete job
        mock.expect_delete("/v1/projects/test-project/locations/us-central1/jobs/test-job")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let scheduler = client.scheduler();

        // Create
        let job = crate::types::cloudscheduler::Job {
            name: "projects/test-project/locations/us-central1/jobs/test-job".to_string(),
            description: Some("Test job".to_string()),
            schedule: Some("0 9 * * 1".to_string()),
            time_zone: Some("America/New_York".to_string()),
            ..Default::default()
        };
        let created = scheduler
            .create_job("test-project", "us-central1", &job)
            .await
            .unwrap();
        assert_eq!(
            created.name,
            "projects/test-project/locations/us-central1/jobs/test-job"
        );
        assert_eq!(created.description, Some("Test job".to_string()));

        // Get
        let fetched = scheduler
            .get_job("test-project", "us-central1", "test-job")
            .await
            .unwrap();
        assert_eq!(fetched.schedule, Some("0 9 * * 1".to_string()));

        // List
        let jobs = scheduler
            .list_jobs("test-project", "us-central1")
            .await
            .unwrap();
        assert_eq!(jobs.len(), 1);
        assert_eq!(
            jobs[0].name,
            "projects/test-project/locations/us-central1/jobs/test-job"
        );

        // Delete
        scheduler
            .delete_job("test-project", "us-central1", "test-job")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_update_job_with_mask() {
        let mut mock = crate::MockClient::new();

        // Update uses PATCH with updateMask query param
        mock.expect_patch(
            "/v1/projects/test-project/locations/us-central1/jobs/test-job?updateMask=description",
        )
        .returning_json(json!({
            "name": "projects/test-project/locations/us-central1/jobs/test-job",
            "description": "Updated description",
            "schedule": "0 9 * * 1"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let scheduler = client.scheduler();

        let job = crate::types::cloudscheduler::Job {
            name: "projects/test-project/locations/us-central1/jobs/test-job".to_string(),
            description: Some("Updated description".to_string()),
            ..Default::default()
        };

        let updated = scheduler
            .update_job(
                "test-project",
                "us-central1",
                "test-job",
                &job,
                "description",
            )
            .await
            .unwrap();
        assert_eq!(updated.description, Some("Updated description".to_string()));
    }

    #[tokio::test]
    async fn test_list_jobs_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/jobs")
            .returning_json(json!({ "jobs": [] }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let jobs = client
            .scheduler()
            .list_jobs("test-project", "us-central1")
            .await
            .unwrap();
        assert!(jobs.is_empty());
    }

    #[tokio::test]
    async fn test_pause_resume_job() {
        let mut mock = crate::MockClient::new();

        // Pause uses POST with :pause suffix
        mock.expect_post("/v1/projects/test-project/locations/us-central1/jobs/test-job:pause")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/jobs/test-job",
                "state": "PAUSED"
            }))
            .times(1);

        // Resume uses POST with :resume suffix
        mock.expect_post("/v1/projects/test-project/locations/us-central1/jobs/test-job:resume")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/jobs/test-job",
                "state": "ENABLED"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let scheduler = client.scheduler();

        // Pause
        let paused = scheduler
            .pause_job("test-project", "us-central1", "test-job")
            .await
            .unwrap();
        assert_eq!(
            paused.state,
            Some(crate::types::cloudscheduler::JobState::Paused)
        );

        // Resume
        let resumed = scheduler
            .resume_job("test-project", "us-central1", "test-job")
            .await
            .unwrap();
        assert_eq!(
            resumed.state,
            Some(crate::types::cloudscheduler::JobState::Enabled)
        );
    }

    #[tokio::test]
    async fn test_run_job() {
        let mut mock = crate::MockClient::new();

        // Run uses POST with :run suffix
        mock.expect_post("/v1/projects/test-project/locations/us-central1/jobs/test-job:run")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/jobs/test-job",
                "state": "ENABLED",
                "lastAttemptTime": "2026-02-09T10:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .scheduler()
            .run_job("test-project", "us-central1", "test-job")
            .await
            .unwrap();

        assert_eq!(
            result.last_attempt_time,
            Some("2026-02-09T10:00:00Z".to_string())
        );
    }
}
