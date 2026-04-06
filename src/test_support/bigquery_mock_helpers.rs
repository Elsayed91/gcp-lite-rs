//! MockClient helpers for BigQuery API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with BigQuery helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait BigqueryMockHelpers {
    /// Helper to expect `get_dataset`: Returns the dataset specified by datasetID.
    fn expect_get_dataset(&mut self, project_id: &str, dataset_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_datasets`: Lists all datasets in the specified project to which the
    /// user has been granted the READER dataset role.
    fn expect_list_datasets(
        &mut self,
        project_id: &str,
        all: &str,
        filter: &str,
        max_results: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_dataset`: Creates a new empty dataset.
    fn expect_create_dataset(&mut self, project_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_dataset`: Deletes the dataset specified by the datasetId value.
    /// Before you can delete a dataset, you must delete all its tables, either manually or by
    /// specifying deleteContents. Immediately after deletion, you can create another dataset with
    /// the same name.
    fn expect_delete_dataset(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        delete_contents: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_dataset`: Updates information in an existing dataset. The update
    /// method replaces the entire dataset resource, whereas the patch method only replaces fields
    /// that are provided in the submitted dataset resource. This method supports RFC5789 patch
    /// semantics.
    fn expect_patch_dataset(
        &mut self,
        project_id: &str,
        dataset_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_table`: Gets the specified table resource by table ID. This method
    /// does not return the data in the table, it only returns the table resource, which describes
    /// the structure of this table.
    fn expect_get_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_tables`: Lists all tables in the specified dataset. Requires the
    /// READER dataset role.
    fn expect_list_tables(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        max_results: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_table`: Creates a new, empty table in the dataset.
    fn expect_create_table(&mut self, project_id: &str, dataset_id: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_table`: Deletes the table specified by tableId from the dataset. If
    /// the table contains data, all the data will be deleted.
    fn expect_delete_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_table`: Updates information in an existing table. The update method
    /// replaces the entire table resource, whereas the patch method only replaces fields that are
    /// provided in the submitted table resource. This method supports RFC5789 patch semantics.
    fn expect_patch_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `query_jobs`: Runs a BigQuery SQL query synchronously and returns query
    /// results if the query completes within a specified timeout.
    fn expect_query_jobs(&mut self, project_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `insert_job`: Starts a new asynchronous job. This API has two different
    /// kinds of endpoint URIs, as this method supports a variety of use cases.
    /// * The *Metadata* URI is used for most interactions, as it accepts the job configuration
    ///   directly.
    /// * The *Upload* URI is ONLY for the case when you're sending both a load job configuration
    ///   and a data stream together. In this case, the Upload URI accepts the job configuration and
    ///   the data as two distinct multipart MIME parts.
    fn expect_insert_job(&mut self, project_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_job`: Returns information about a specific job. Job information is
    /// available for a six month period after creation. Requires that you're the person who ran the
    /// job, or have the Is Owner project role.
    fn expect_get_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_jobs`: Lists all jobs that you started in the specified project. Job
    /// information is available for a six month period after creation. The job list is sorted in
    /// reverse chronological order, by job creation time. Requires the Can View project role, or
    /// the Is Owner project role if you set the allUsers property.
    #[allow(clippy::too_many_arguments)]
    fn expect_list_jobs(
        &mut self,
        project_id: &str,
        all_users: &str,
        state_filter: &[&str],
        max_results: &str,
        page_token: &str,
        projection: &str,
        parent_job_id: &str,
        min_creation_time: &str,
        max_creation_time: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `cancel_job`: Requests that a job be cancelled. This call will return
    /// immediately, and the client will need to poll for the job status to see if the cancel
    /// completed successfully. Cancelled jobs may still incur costs.
    fn expect_cancel_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_job`: Requests the deletion of the metadata of a job. This call
    /// returns when the job's metadata is deleted.
    fn expect_delete_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl BigqueryMockHelpers for MockClient {
    /// Helper to expect `get_dataset`: Returns the dataset specified by datasetID.
    fn expect_get_dataset(
        &mut self,
        project_id: &str,
        dataset_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_datasets`: Lists all datasets in the specified project to which the
    /// user has been granted the READER dataset role.
    fn expect_list_datasets(
        &mut self,
        project_id: &str,
        all: &str,
        filter: &str,
        max_results: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/datasets");
        let mut __qp: Vec<String> = Vec::new();
        if !all.is_empty() {
            __qp.push(format!("all={}", all));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_dataset`: Creates a new empty dataset.
    fn expect_create_dataset(
        &mut self,
        project_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/datasets");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_dataset`: Deletes the dataset specified by the datasetId value.
    /// Before you can delete a dataset, you must delete all its tables, either manually or by
    /// specifying deleteContents. Immediately after deletion, you can create another dataset with
    /// the same name.
    fn expect_delete_dataset(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        delete_contents: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}");
        let mut __qp: Vec<String> = Vec::new();
        if !delete_contents.is_empty() {
            __qp.push(format!("deleteContents={}", delete_contents));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_delete(&path)
    }

    /// Helper to expect `patch_dataset`: Updates information in an existing dataset. The update
    /// method replaces the entire dataset resource, whereas the patch method only replaces fields
    /// that are provided in the submitted dataset resource. This method supports RFC5789 patch
    /// semantics.
    fn expect_patch_dataset(
        &mut self,
        project_id: &str,
        dataset_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}");
        self.expect_patch(&path)
    }

    /// Helper to expect `get_table`: Gets the specified table resource by table ID. This method
    /// does not return the data in the table, it only returns the table resource, which describes
    /// the structure of this table.
    fn expect_get_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_tables`: Lists all tables in the specified dataset. Requires the
    /// READER dataset role.
    fn expect_list_tables(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        max_results: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables");
        let mut __qp: Vec<String> = Vec::new();
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_table`: Creates a new, empty table in the dataset.
    fn expect_create_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_table`: Deletes the table specified by tableId from the dataset. If
    /// the table contains data, all the data will be deleted.
    fn expect_delete_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}");
        self.expect_delete(&path)
    }

    /// Helper to expect `patch_table`: Updates information in an existing table. The update method
    /// replaces the entire table resource, whereas the patch method only replaces fields that are
    /// provided in the submitted table resource. This method supports RFC5789 patch semantics.
    fn expect_patch_table(
        &mut self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}");
        self.expect_patch(&path)
    }

    /// Helper to expect `query_jobs`: Runs a BigQuery SQL query synchronously and returns query
    /// results if the query completes within a specified timeout.
    fn expect_query_jobs(
        &mut self,
        project_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/queries");
        self.expect_post(&path)
    }

    /// Helper to expect `insert_job`: Starts a new asynchronous job. This API has two different
    /// kinds of endpoint URIs, as this method supports a variety of use cases.
    /// * The *Metadata* URI is used for most interactions, as it accepts the job configuration
    ///   directly.
    /// * The *Upload* URI is ONLY for the case when you're sending both a load job configuration
    ///   and a data stream together. In this case, the Upload URI accepts the job configuration and
    ///   the data as two distinct multipart MIME parts.
    fn expect_insert_job(
        &mut self,
        project_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/bigquery/v2/projects/{project_id}/jobs");
        self.expect_post(&path)
    }

    /// Helper to expect `get_job`: Returns information about a specific job. Job information is
    /// available for a six month period after creation. Requires that you're the person who ran the
    /// job, or have the Is Owner project role.
    fn expect_get_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/jobs/{job_id}");
        let mut __qp: Vec<String> = Vec::new();
        if !location.is_empty() {
            __qp.push(format!("location={}", location));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `list_jobs`: Lists all jobs that you started in the specified project. Job
    /// information is available for a six month period after creation. The job list is sorted in
    /// reverse chronological order, by job creation time. Requires the Can View project role, or
    /// the Is Owner project role if you set the allUsers property.
    #[allow(clippy::too_many_arguments)]
    fn expect_list_jobs(
        &mut self,
        project_id: &str,
        all_users: &str,
        state_filter: &[&str],
        max_results: &str,
        page_token: &str,
        projection: &str,
        parent_job_id: &str,
        min_creation_time: &str,
        max_creation_time: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/jobs");
        let mut __qp: Vec<String> = Vec::new();
        if !all_users.is_empty() {
            __qp.push(format!("allUsers={}", all_users));
        }
        for __v in state_filter {
            __qp.push(format!("stateFilter={}", __v));
        }
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !projection.is_empty() {
            __qp.push(format!("projection={}", projection));
        }
        if !parent_job_id.is_empty() {
            __qp.push(format!("parentJobId={}", parent_job_id));
        }
        if !min_creation_time.is_empty() {
            __qp.push(format!("minCreationTime={}", min_creation_time));
        }
        if !max_creation_time.is_empty() {
            __qp.push(format!("maxCreationTime={}", max_creation_time));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `cancel_job`: Requests that a job be cancelled. This call will return
    /// immediately, and the client will need to poll for the job status to see if the cancel
    /// completed successfully. Cancelled jobs may still incur costs.
    fn expect_cancel_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/jobs/{job_id}/cancel");
        let mut __qp: Vec<String> = Vec::new();
        if !location.is_empty() {
            __qp.push(format!("location={}", location));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `delete_job`: Requests the deletion of the metadata of a job. This call
    /// returns when the job's metadata is deleted.
    fn expect_delete_job(
        &mut self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/bigquery/v2/projects/{project_id}/jobs/{job_id}/delete");
        let mut __qp: Vec<String> = Vec::new();
        if !location.is_empty() {
            __qp.push(format!("location={}", location));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_delete(&path)
    }
}
