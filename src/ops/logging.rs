//! Operation contracts for the Cloud Logging API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/logging.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::logging::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud Logging API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::logging::LoggingClient`] instead.
pub struct LoggingOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> LoggingOps<'a> {
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
        "https://logging.googleapis.com"
    }

    /// Lists sinks.
    ///
    /// **GCP API**: `GET v2/{+parent}/sinks`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "b *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. A filter expression to constrain the sinks returned. Today, this only supports the following strings: '' 'in_s
    /// - `pageSize` — Optional. The maximum number of results to return from this request. Non-positive values are ignored. The presence of ne
    /// - `pageToken` — Optional. If present, then retrieve the next batch of results from the preceding call to this method. pageToken must be
    ///
    /// # Response
    /// [`ListSinksResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_sinks(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> Result<ListSinksResponse> {
        let url = format!("{}/v2/{}/sinks", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_sinks response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a sink.
    ///
    /// **GCP API**: `GET v2/{+sinkName}`
    ///
    /// # Path Parameters
    /// - `sinkName` — Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/ *(required)*
    ///
    /// # Response
    /// [`LogSink`]
    #[allow(dead_code)]
    pub(crate) async fn get_sink(&self, sink_name: &str) -> Result<LogSink> {
        let url = format!("{}/v2/{}", self.base_url(), sink_name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_sink response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a sink that exports specified log entries to a destination. The export begins
    /// upon ingress, unless the sink's writer_identity is not permitted to write to the
    /// destination. A sink can export log entries only from the resource owning the sink.
    ///
    /// **GCP API**: `POST v2/{+parent}/sinks`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAc *(required)*
    ///
    /// # Query Parameters
    /// - `customWriterIdentity` — Optional. The service account provided by the caller that will be used to write the log entries. The format must be serv
    /// - `uniqueWriterIdentity` — Optional. Determines the kind of IAM identity returned as writer_identity in the new sink. If this value is omitted or s
    ///
    /// # Request Body
    /// [`LogSink`]
    ///
    /// # Response
    /// [`LogSink`]
    #[allow(dead_code)]
    pub(crate) async fn create_sink(&self, parent: &str, body: &LogSink) -> Result<LogSink> {
        let url = format!("{}/v2/{}/sinks", self.base_url(), parent,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_sink response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a sink. If the sink has a unique writer_identity, then that service account is
    /// also deleted.
    ///
    /// **GCP API**: `DELETE v2/{+sinkName}`
    ///
    /// # Path Parameters
    /// - `sinkName` — Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects *(required)*
    ///
    /// # Response
    /// [`LoggingEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_sink(&self, sink_name: &str) -> Result<LoggingEmpty> {
        let url = format!("{}/v2/{}", self.base_url(), sink_name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_sink response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists logs-based metrics.
    ///
    /// **GCP API**: `GET v2/{+parent}/metrics`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The name of the project containing the metrics: "projects/[PROJECT_ID]" *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. The maximum number of results to return from this request. Non-positive values are ignored. The presence of ne
    /// - `pageToken` — Optional. If present, then retrieve the next batch of results from the preceding call to this method. pageToken must be
    ///
    /// # Response
    /// [`ListLogMetricsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_metrics(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListLogMetricsResponse> {
        let url = format!("{}/v2/{}/metrics", self.base_url(), parent,);
        let url =
            crate::append_query_params(url, &[("pageSize", page_size), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_metrics response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a logs-based metric.
    ///
    /// **GCP API**: `GET v2/{+metricName}`
    ///
    /// # Path Parameters
    /// - `metricName` — Required. The resource name of the desired metric: "projects/[PROJECT_ID]/metrics/[METRIC_ID]" *(required)*
    ///
    /// # Response
    /// [`LogMetric`]
    #[allow(dead_code)]
    pub(crate) async fn get_metric(&self, metric_name: &str) -> Result<LogMetric> {
        let url = format!("{}/v2/{}", self.base_url(), metric_name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_metric response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a logs-based metric.
    ///
    /// **GCP API**: `POST v2/{+parent}/metrics`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the project in which to create the metric: "projects/[PROJECT_ID]" The new metric must be *(required)*
    ///
    /// # Request Body
    /// [`LogMetric`]
    ///
    /// # Response
    /// [`LogMetric`]
    #[allow(dead_code)]
    pub(crate) async fn create_metric(&self, parent: &str, body: &LogMetric) -> Result<LogMetric> {
        let url = format!("{}/v2/{}/metrics", self.base_url(), parent,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_metric response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a logs-based metric.
    ///
    /// **GCP API**: `DELETE v2/{+metricName}`
    ///
    /// # Path Parameters
    /// - `metricName` — Required. The resource name of the metric to delete: "projects/[PROJECT_ID]/metrics/[METRIC_ID]" *(required)*
    ///
    /// # Response
    /// [`LoggingEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_metric(&self, metric_name: &str) -> Result<LoggingEmpty> {
        let url = format!("{}/v2/{}", self.base_url(), metric_name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_metric response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_sinks() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-parent/sinks?pageSize=test-pageSize&pageToken=test-pageToken&filter=test-filter")
            .returning_json(serde_json::to_value(ListSinksResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops
            .list_sinks(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_sink() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-sinkName")
            .returning_json(serde_json::to_value(LogSink::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops.get_sink("test-sinkName").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_sink() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v2/test-parent/sinks")
            .returning_json(serde_json::to_value(LogSink::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let body = LogSink::fixture();
        let result = ops.create_sink("test-parent", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_sink() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v2/test-sinkName")
            .returning_json(serde_json::to_value(LoggingEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops.delete_sink("test-sinkName").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_metrics() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-parent/metrics?pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListLogMetricsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops
            .list_metrics("test-parent", "test-pageSize", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_metric() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-metricName")
            .returning_json(serde_json::to_value(LogMetric::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops.get_metric("test-metricName").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_metric() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v2/test-parent/metrics")
            .returning_json(serde_json::to_value(LogMetric::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let body = LogMetric::fixture();
        let result = ops.create_metric("test-parent", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_metric() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v2/test-metricName")
            .returning_json(serde_json::to_value(LoggingEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = LoggingOps::new(&client);

        let result = ops.delete_metric("test-metricName").await;
        assert!(result.is_ok());
    }
}
