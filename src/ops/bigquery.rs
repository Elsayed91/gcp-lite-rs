//! Operation contracts for the BigQuery API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/bigquery.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::bigquery::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the BigQuery API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::bigquery::BigqueryClient`] instead.
pub struct BigqueryOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> BigqueryOps<'a> {
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
        "https://bigquery.googleapis.com/bigquery/v2"
    }

    /// Returns the dataset specified by datasetID.
    ///
    /// **GCP API**: `GET projects/{+projectId}/datasets/{+datasetId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/get>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the requested dataset *(required)*
    /// - `datasetId` — Required. Dataset ID of the requested dataset *(required)*
    ///
    /// # Query Parameters
    /// - `accessPolicyVersion` — Optional. The version of the access policy schema to fetch. Valid values are 0, 1, and 3. Requests specifying an invalid
    /// - `datasetView` — Optional. Specifies the view that determines which dataset information is returned. By default, metadata and ACL informa
    ///
    /// # Response
    /// [`Dataset`]
    #[allow(dead_code)]
    pub(crate) async fn get_dataset(&self, project_id: &str, dataset_id: &str) -> Result<Dataset> {
        let url = format!(
            "{}/projects/{}/datasets/{}",
            self.base_url(),
            project_id,
            dataset_id,
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_dataset response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all datasets in the specified project to which the user has been granted the
    /// READER dataset role.
    ///
    /// **GCP API**: `GET projects/{+projectId}/datasets`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/list>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the datasets to be listed *(required)*
    ///
    /// # Query Parameters
    /// - `all` — Whether to list all datasets, including hidden ones
    /// - `filter` — An expression for filtering the results of the request by label. The syntax is `labels.[:]`. Multiple filters can be AND
    /// - `maxResults` — The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entir
    /// - `pageToken` — Page token, returned by a previous call, to request the next page of results
    ///
    /// # Response
    /// [`DatasetList`]
    #[allow(dead_code)]
    pub(crate) async fn list_datasets(
        &self,
        project_id: &str,
        all: &str,
        filter: &str,
        max_results: &str,
        page_token: &str,
    ) -> Result<DatasetList> {
        let url = format!("{}/projects/{}/datasets", self.base_url(), project_id,);
        let url = crate::append_query_params(
            url,
            &[
                ("all", all),
                ("filter", filter),
                ("maxResults", max_results),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_datasets response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new empty dataset.
    ///
    /// **GCP API**: `POST projects/{+projectId}/datasets`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/insert>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the new dataset *(required)*
    ///
    /// # Query Parameters
    /// - `accessPolicyVersion` — Optional. The version of the provided access policy schema. Valid values are 0, 1, and 3. Requests specifying an invalid
    ///
    /// # Request Body
    /// [`Dataset`]
    ///
    /// # Response
    /// [`Dataset`]
    #[allow(dead_code)]
    pub(crate) async fn create_dataset(&self, project_id: &str, body: &Dataset) -> Result<Dataset> {
        let url = format!("{}/projects/{}/datasets", self.base_url(), project_id,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_dataset response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the dataset specified by the datasetId value. Before you can delete a dataset,
    /// you must delete all its tables, either manually or by specifying deleteContents.
    /// Immediately after deletion, you can create another dataset with the same name.
    ///
    /// **GCP API**: `DELETE projects/{+projectId}/datasets/{+datasetId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/delete>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the dataset being deleted *(required)*
    /// - `datasetId` — Required. Dataset ID of dataset being deleted *(required)*
    ///
    /// # Query Parameters
    /// - `deleteContents` — If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default
    #[allow(dead_code)]
    pub(crate) async fn delete_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        delete_contents: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/projects/{}/datasets/{}",
            self.base_url(),
            project_id,
            dataset_id,
        );
        let url = crate::append_query_params(url, &[("deleteContents", delete_contents)]);
        let _ = self.client.delete(&url).await?;
        Ok(())
    }

    /// Updates information in an existing dataset. The update method replaces the entire
    /// dataset resource, whereas the patch method only replaces fields that are provided in the
    /// submitted dataset resource. This method supports RFC5789 patch semantics.
    ///
    /// **GCP API**: `PATCH projects/{+projectId}/datasets/{+datasetId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/patch>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the dataset being updated *(required)*
    /// - `datasetId` — Required. Dataset ID of the dataset being updated *(required)*
    ///
    /// # Query Parameters
    /// - `accessPolicyVersion` — Optional. The version of the provided access policy schema. Valid values are 0, 1, and 3. Requests specifying an invalid
    /// - `updateMode` — Optional. Specifies the fields of dataset that update/patch operation is targeting By default, both metadata and ACL fie
    ///
    /// # Request Body
    /// [`Dataset`]
    ///
    /// # Response
    /// [`Dataset`]
    #[allow(dead_code)]
    pub(crate) async fn patch_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        body: &Dataset,
    ) -> Result<Dataset> {
        let url = format!(
            "{}/projects/{}/datasets/{}",
            self.base_url(),
            project_id,
            dataset_id,
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_dataset response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the specified table resource by table ID. This method does not return the data in
    /// the table, it only returns the table resource, which describes the structure of this
    /// table.
    ///
    /// **GCP API**: `GET projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/get>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the requested table *(required)*
    /// - `datasetId` — Required. Dataset ID of the requested table *(required)*
    /// - `tableId` — Required. Table ID of the requested table *(required)*
    ///
    /// # Query Parameters
    /// - `selectedFields` — List of table schema fields to return (comma-separated). If unspecified, all fields are returned. A fieldMask cannot be
    /// - `view` — Optional. Specifies the view that determines which table information is returned. By default, basic table information an
    ///
    /// # Response
    /// [`Table`]
    #[allow(dead_code)]
    pub(crate) async fn get_table(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<Table> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables/{}",
            self.base_url(),
            project_id,
            dataset_id,
            table_id,
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    ///
    /// **GCP API**: `GET projects/{+projectId}/datasets/{+datasetId}/tables`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/list>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the tables to list *(required)*
    /// - `datasetId` — Required. Dataset ID of the tables to list *(required)*
    ///
    /// # Query Parameters
    /// - `maxResults` — The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entir
    /// - `pageToken` — Page token, returned by a previous call, to request the next page of results
    ///
    /// # Response
    /// [`TableList`]
    #[allow(dead_code)]
    pub(crate) async fn list_tables(
        &self,
        project_id: &str,
        dataset_id: &str,
        max_results: &str,
        page_token: &str,
    ) -> Result<TableList> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables",
            self.base_url(),
            project_id,
            dataset_id,
        );
        let url = crate::append_query_params(
            url,
            &[("maxResults", max_results), ("pageToken", page_token)],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_tables response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new, empty table in the dataset.
    ///
    /// **GCP API**: `POST projects/{+projectId}/datasets/{+datasetId}/tables`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/insert>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the new table *(required)*
    /// - `datasetId` — Required. Dataset ID of the new table *(required)*
    ///
    /// # Request Body
    /// [`Table`]
    ///
    /// # Response
    /// [`Table`]
    #[allow(dead_code)]
    pub(crate) async fn create_table(
        &self,
        project_id: &str,
        dataset_id: &str,
        body: &Table,
    ) -> Result<Table> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables",
            self.base_url(),
            project_id,
            dataset_id,
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the table specified by tableId from the dataset. If the table contains data, all
    /// the data will be deleted.
    ///
    /// **GCP API**: `DELETE projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/delete>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the table to delete *(required)*
    /// - `datasetId` — Required. Dataset ID of the table to delete *(required)*
    /// - `tableId` — Required. Table ID of the table to delete *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_table(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables/{}",
            self.base_url(),
            project_id,
            dataset_id,
            table_id,
        );
        let _ = self.client.delete(&url).await?;
        Ok(())
    }

    /// Updates information in an existing table. The update method replaces the entire table
    /// resource, whereas the patch method only replaces fields that are provided in the
    /// submitted table resource. This method supports RFC5789 patch semantics.
    ///
    /// **GCP API**: `PATCH projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/patch>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the table to update *(required)*
    /// - `datasetId` — Required. Dataset ID of the table to update *(required)*
    /// - `tableId` — Required. Table ID of the table to update *(required)*
    ///
    /// # Query Parameters
    /// - `autodetect_schema` — Optional. When true will autodetect schema, else will keep original schema
    ///
    /// # Request Body
    /// [`Table`]
    ///
    /// # Response
    /// [`Table`]
    #[allow(dead_code)]
    pub(crate) async fn patch_table(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        body: &Table,
    ) -> Result<Table> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables/{}",
            self.base_url(),
            project_id,
            dataset_id,
            table_id,
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_table response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes
    /// within a specified timeout.
    ///
    /// **GCP API**: `POST projects/{+projectId}/queries`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the query request. *(required)*
    ///
    /// # Request Body
    /// [`QueryRequest`]
    ///
    /// # Response
    /// [`QueryResponse`]
    #[allow(dead_code)]
    pub(crate) async fn query_jobs(
        &self,
        project_id: &str,
        body: &QueryRequest,
    ) -> Result<QueryResponse> {
        let url = format!("{}/projects/{}/queries", self.base_url(), project_id,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse query_jobs response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Starts a new asynchronous job. This API has two different kinds of endpoint URIs, as
    /// this method supports a variety of use cases.
    /// * The *Metadata* URI is used for most interactions, as it accepts the job configuration
    ///   directly.
    /// * The *Upload* URI is ONLY for the case when you're sending both a load job
    ///   configuration and a data stream together. In this case, the Upload URI accepts the job
    ///   configuration and the data as two distinct multipart MIME parts.
    ///
    /// **GCP API**: `POST projects/{+projectId}/jobs`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/insert>
    ///
    /// # Path Parameters
    /// - `projectId` — Project ID of project that will be billed for the job. *(required)*
    ///
    /// # Request Body
    /// [`Job`]
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn insert_job(&self, project_id: &str, body: &Job) -> Result<Job> {
        let url = format!("{}/projects/{}/jobs", self.base_url(), project_id,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse insert_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns information about a specific job. Job information is available for a six month
    /// period after creation. Requires that you're the person who ran the job, or have the Is
    /// Owner project role.
    ///
    /// **GCP API**: `GET projects/{+projectId}/jobs/{+jobId}`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/get>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the requested job. *(required)*
    /// - `jobId` — Required. Job ID of the requested job. *(required)*
    ///
    /// # Query Parameters
    /// - `location` — The geographic location of the job. You must specify the location to run the job for the following scenarios: * If the l
    ///
    /// # Response
    /// [`Job`]
    #[allow(dead_code)]
    pub(crate) async fn get_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> Result<Job> {
        let url = format!(
            "{}/projects/{}/jobs/{}",
            self.base_url(),
            project_id,
            job_id,
        );
        let url = crate::append_query_params(url, &[("location", location)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all jobs that you started in the specified project. Job information is available
    /// for a six month period after creation. The job list is sorted in reverse chronological
    /// order, by job creation time. Requires the Can View project role, or the Is Owner project
    /// role if you set the allUsers property.
    ///
    /// **GCP API**: `GET projects/{+projectId}/jobs`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/list>
    ///
    /// # Path Parameters
    /// - `projectId` — Project ID of the jobs to list. *(required)*
    ///
    /// # Query Parameters
    /// - `allUsers` — Whether to display jobs owned by all users in the project. Default False.
    /// - `maxCreationTime` — Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this time
    /// - `maxResults` — The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entir
    /// - `minCreationTime` — Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this times
    /// - `pageToken` — Page token, returned by a previous call, to request the next page of results.
    /// - `parentJobId` — If set, show only child jobs of the specified parent. Otherwise, show all top-level jobs.
    /// - `projection` — Restrict information returned to a set of selected fields
    /// - `stateFilter` — Filter for job state
    ///
    /// # Response
    /// [`JobList`]
    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub(crate) async fn list_jobs(
        &self,
        project_id: &str,
        all_users: &str,
        state_filter: &[&str],
        max_results: &str,
        page_token: &str,
        projection: &str,
        parent_job_id: &str,
        min_creation_time: &str,
        max_creation_time: &str,
    ) -> Result<JobList> {
        let url = format!("{}/projects/{}/jobs", self.base_url(), project_id,);
        let mut __qp: Vec<String> = Vec::new();
        if !all_users.is_empty() {
            __qp.push(format!("allUsers={}", encode(all_users)));
        }
        for __v in state_filter {
            __qp.push(format!("stateFilter={}", encode(__v)));
        }
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", encode(max_results)));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", encode(page_token)));
        }
        if !projection.is_empty() {
            __qp.push(format!("projection={}", encode(projection)));
        }
        if !parent_job_id.is_empty() {
            __qp.push(format!("parentJobId={}", encode(parent_job_id)));
        }
        if !min_creation_time.is_empty() {
            __qp.push(format!("minCreationTime={}", encode(min_creation_time)));
        }
        if !max_creation_time.is_empty() {
            __qp.push(format!("maxCreationTime={}", encode(max_creation_time)));
        }
        let url = if __qp.is_empty() {
            url
        } else {
            format!("{}?{}", url, __qp.join("&"))
        };
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_jobs response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Requests that a job be cancelled. This call will return immediately, and the client will
    /// need to poll for the job status to see if the cancel completed successfully. Cancelled
    /// jobs may still incur costs.
    ///
    /// **GCP API**: `POST projects/{+projectId}/jobs/{+jobId}/cancel`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/cancel>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the job to cancel *(required)*
    /// - `jobId` — Required. Job ID of the job to cancel *(required)*
    ///
    /// # Query Parameters
    /// - `location` — The geographic location of the job. You must [specify the location](https://cloud.google.com/bigquery/docs/locations#spe
    ///
    /// # Response
    /// [`JobCancelResponse`]
    #[allow(dead_code)]
    pub(crate) async fn cancel_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> Result<JobCancelResponse> {
        let url = format!(
            "{}/projects/{}/jobs/{}/cancel",
            self.base_url(),
            project_id,
            job_id,
        );
        let url = crate::append_query_params(url, &[("location", location)]);
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse cancel_job response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Requests the deletion of the metadata of a job. This call returns when the job's
    /// metadata is deleted.
    ///
    /// **GCP API**: `DELETE projects/{+projectId}/jobs/{+jobId}/delete`
    /// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/delete>
    ///
    /// # Path Parameters
    /// - `projectId` — Required. Project ID of the job for which metadata is to be deleted. *(required)*
    /// - `jobId` — Required. Job ID of the job for which metadata is to be deleted. If this is a parent job which has child jobs, the metad *(required)*
    ///
    /// # Query Parameters
    /// - `location` — The geographic location of the job. Required. For more information, see how to [specify locations](https://cloud.google.
    #[allow(dead_code)]
    pub(crate) async fn delete_job(
        &self,
        project_id: &str,
        job_id: &str,
        location: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/projects/{}/jobs/{}/delete",
            self.base_url(),
            project_id,
            job_id,
        );
        let url = crate::append_query_params(url, &[("location", location)]);
        let _ = self.client.delete(&url).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-projectId/datasets/test-datasetId")
            .returning_json(serde_json::to_value(Dataset::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops.get_dataset("test-projectId", "test-datasetId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_datasets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-projectId/datasets?all=test-all&filter=test-filter&maxResults=test-maxResults&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(DatasetList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .list_datasets(
                "test-projectId",
                "test-all",
                "test-filter",
                "test-maxResults",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-projectId/datasets")
            .returning_json(serde_json::to_value(Dataset::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = Dataset::fixture();
        let result = ops.create_dataset("test-projectId", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/bigquery/v2/projects/test-projectId/datasets/test-datasetId?deleteContents=test-deleteContents")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .delete_dataset("test-projectId", "test-datasetId", "test-deleteContents")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_dataset() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/bigquery/v2/projects/test-projectId/datasets/test-datasetId")
            .returning_json(serde_json::to_value(Dataset::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = Dataset::fixture();
        let result = ops
            .patch_dataset("test-projectId", "test-datasetId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/bigquery/v2/projects/test-projectId/datasets/test-datasetId/tables/test-tableId",
        )
        .returning_json(serde_json::to_value(Table::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .get_table("test-projectId", "test-datasetId", "test-tableId")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_tables() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-projectId/datasets/test-datasetId/tables?maxResults=test-maxResults&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(TableList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .list_tables(
                "test-projectId",
                "test-datasetId",
                "test-maxResults",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-projectId/datasets/test-datasetId/tables")
            .returning_json(serde_json::to_value(Table::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = Table::fixture();
        let result = ops
            .create_table("test-projectId", "test-datasetId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/bigquery/v2/projects/test-projectId/datasets/test-datasetId/tables/test-tableId",
        )
        .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .delete_table("test-projectId", "test-datasetId", "test-tableId")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_table() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/bigquery/v2/projects/test-projectId/datasets/test-datasetId/tables/test-tableId",
        )
        .returning_json(serde_json::to_value(Table::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = Table::fixture();
        let result = ops
            .patch_table("test-projectId", "test-datasetId", "test-tableId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_jobs() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-projectId/queries")
            .returning_json(serde_json::to_value(QueryResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = QueryRequest::fixture();
        let result = ops.query_jobs("test-projectId", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/bigquery/v2/projects/test-projectId/jobs")
            .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let body = Job::fixture();
        let result = ops.insert_job("test-projectId", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/bigquery/v2/projects/test-projectId/jobs/test-jobId?location=test-location",
        )
        .returning_json(serde_json::to_value(Job::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .get_job("test-projectId", "test-jobId", "test-location")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_jobs() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/bigquery/v2/projects/test-projectId/jobs?allUsers=test-allUsers&stateFilter=test-stateFilter&maxResults=test-maxResults&pageToken=test-pageToken&projection=test-projection&parentJobId=test-parentJobId&minCreationTime=test-minCreationTime&maxCreationTime=test-maxCreationTime")
            .returning_json(serde_json::to_value(JobList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .list_jobs(
                "test-projectId",
                "test-allUsers",
                &["test-stateFilter"],
                "test-maxResults",
                "test-pageToken",
                "test-projection",
                "test-parentJobId",
                "test-minCreationTime",
                "test-maxCreationTime",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancel_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/bigquery/v2/projects/test-projectId/jobs/test-jobId/cancel?location=test-location",
        )
        .returning_json(serde_json::to_value(JobCancelResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .cancel_job("test-projectId", "test-jobId", "test-location")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_job() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/bigquery/v2/projects/test-projectId/jobs/test-jobId/delete?location=test-location",
        )
        .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = BigqueryOps::new(&client);

        let result = ops
            .delete_job("test-projectId", "test-jobId", "test-location")
            .await;
        assert!(result.is_ok());
    }
}
