//! MockClient helpers for Cloud Scheduler API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Scheduler helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudschedulerMockHelpers {
    /// Helper to expect `create_job`: Creates a job.
    fn expect_create_job(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_job`: Gets a job.
    fn expect_get_job(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_jobs`: Lists jobs.
    fn expect_list_jobs(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_job`: Updates a job. If successful, the updated Job is returned. If
    /// the job does not exist, `NOT_FOUND` is returned. If UpdateJob does not successfully return,
    /// it is possible for the job to be in an Job.State.UPDATE_FAILED state. A job in this state
    /// may not be executed. If this happens, retry the UpdateJob request until a successful
    /// response is received.
    fn expect_update_job(&mut self, name: &str, update_mask: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_job`: Deletes a job.
    fn expect_delete_job(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `pause_job`: Pauses a job. If a job is paused then the system will stop
    /// executing the job until it is re-enabled via ResumeJob. The state of the job is stored in
    /// state; if paused it will be set to Job.State.PAUSED. A job must be in Job.State.ENABLED to
    /// be paused.
    fn expect_pause_job(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `resume_job`: Resume a job. This method reenables a job after it has been
    /// Job.State.PAUSED. The state of a job is stored in Job.state; after calling this method it
    /// will be set to Job.State.ENABLED. A job must be in Job.State.PAUSED to be resumed.
    fn expect_resume_job(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `run_job`: Forces a job to run now. When this method is called, Cloud
    /// Scheduler will dispatch the job, even if the job is already running.
    fn expect_run_job(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudschedulerMockHelpers for MockClient {
    /// Helper to expect `create_job`: Creates a job.
    fn expect_create_job(&mut self, parent: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/jobs");
        self.expect_post(&path)
    }

    /// Helper to expect `get_job`: Gets a job.
    fn expect_get_job(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_jobs`: Lists jobs.
    fn expect_list_jobs(&mut self, parent: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/jobs");
        self.expect_get(&path)
    }

    /// Helper to expect `update_job`: Updates a job. If successful, the updated Job is returned. If
    /// the job does not exist, `NOT_FOUND` is returned. If UpdateJob does not successfully return,
    /// it is possible for the job to be in an Job.State.UPDATE_FAILED state. A job in this state
    /// may not be executed. If this happens, retry the UpdateJob request until a successful
    /// response is received.
    fn expect_update_job(
        &mut self,
        name: &str,
        update_mask: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{name}");
        let mut __qp: Vec<String> = Vec::new();
        if !update_mask.is_empty() {
            __qp.push(format!("updateMask={}", update_mask));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_job`: Deletes a job.
    fn expect_delete_job(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }

    /// Helper to expect `pause_job`: Pauses a job. If a job is paused then the system will stop
    /// executing the job until it is re-enabled via ResumeJob. The state of the job is stored in
    /// state; if paused it will be set to Job.State.PAUSED. A job must be in Job.State.ENABLED to
    /// be paused.
    fn expect_pause_job(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}:pause");
        self.expect_post(&path)
    }

    /// Helper to expect `resume_job`: Resume a job. This method reenables a job after it has been
    /// Job.State.PAUSED. The state of a job is stored in Job.state; after calling this method it
    /// will be set to Job.State.ENABLED. A job must be in Job.State.PAUSED to be resumed.
    fn expect_resume_job(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}:resume");
        self.expect_post(&path)
    }

    /// Helper to expect `run_job`: Forces a job to run now. When this method is called, Cloud
    /// Scheduler will dispatch the job, even if the job is already running.
    fn expect_run_job(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}:run");
        self.expect_post(&path)
    }
}
