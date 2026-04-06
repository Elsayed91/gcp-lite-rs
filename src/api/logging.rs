//! Cloud Logging API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::logging::LoggingOps`. This layer adds ergonomic method
//! signatures (project/sink_id instead of raw resource names) and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 2.2 (logging_sinks_configured): list/create/delete log sinks
//!   - CIS 2.3 (logging_bucket_retention_locked): list sinks to find destination bucket
//!   - CIS 2.4-2.11 (alert policies): list/create/delete log-based metrics

use crate::{
    GcpHttpClient, Result,
    ops::logging::LoggingOps,
    types::logging::{LogMetric, LogSink},
};

/// Client for the Cloud Logging API.
pub struct LoggingClient<'a> {
    ops: LoggingOps<'a>,
}

impl<'a> LoggingClient<'a> {
    /// Create a new Cloud Logging API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: LoggingOps::new(client),
        }
    }

    // ── Log Sinks ─────────────────────────────────────────────────────

    /// List all log sinks for a project (auto-paginated).
    pub async fn list_sinks(&self, project: &str) -> Result<Vec<LogSink>> {
        let parent = format!("projects/{}", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_sinks(&parent, "100", &page_token, "").await?;
            all.extend(resp.sinks);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a log sink by project and sink ID.
    pub async fn get_sink(&self, project: &str, sink_id: &str) -> Result<LogSink> {
        let name = format!("projects/{}/sinks/{}", project, sink_id);
        self.ops.get_sink(&name).await
    }

    /// Create a log sink for a project.
    pub async fn create_sink(&self, project: &str, sink: &LogSink) -> Result<LogSink> {
        let parent = format!("projects/{}", project);
        self.ops.create_sink(&parent, sink).await
    }

    /// Delete a log sink by project and sink ID.
    pub async fn delete_sink(&self, project: &str, sink_id: &str) -> Result<()> {
        let name = format!("projects/{}/sinks/{}", project, sink_id);
        self.ops.delete_sink(&name).await?;
        Ok(())
    }

    // ── Log-Based Metrics ─────────────────────────────────────────────

    /// List all log-based metrics for a project (auto-paginated).
    pub async fn list_metrics(&self, project: &str) -> Result<Vec<LogMetric>> {
        let parent = format!("projects/{}", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_metrics(&parent, "100", &page_token).await?;
            all.extend(resp.metrics);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a log-based metric by project and metric ID.
    pub async fn get_metric(&self, project: &str, metric_id: &str) -> Result<LogMetric> {
        let name = format!("projects/{}/metrics/{}", project, metric_id);
        self.ops.get_metric(&name).await
    }

    /// Create a log-based metric for a project.
    pub async fn create_metric(&self, project: &str, metric: &LogMetric) -> Result<LogMetric> {
        let parent = format!("projects/{}", project);
        self.ops.create_metric(&parent, metric).await
    }

    /// Delete a log-based metric by project and metric ID.
    pub async fn delete_metric(&self, project: &str, metric_id: &str) -> Result<()> {
        let name = format!("projects/{}/metrics/{}", project, metric_id);
        self.ops.delete_metric(&name).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_sinks() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/sinks?pageSize=100")
            .returning_json(json!({
                "sinks": [
                    {
                        "name": "my-sink",
                        "destination": "logging.googleapis.com/projects/my-project/locations/global/buckets/_Default",
                        "filter": "severity >= ERROR",
                        "writerIdentity": "serviceAccount:p12345-abcdef@gcp-sa-logging.iam.gserviceaccount.com",
                        "createTime": "2026-01-01T00:00:00Z"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.list_sinks("my-project").await;
        assert!(result.is_ok());
        let sinks = result.unwrap();
        assert_eq!(sinks.len(), 1);
        assert_eq!(sinks[0].name, "my-sink");
        assert!(sinks[0].destination.contains("_Default"));
        assert_eq!(sinks[0].filter.as_deref(), Some("severity >= ERROR"));
    }

    #[tokio::test]
    async fn test_get_sink() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/sinks/my-sink")
            .returning_json(json!({
                "name": "my-sink",
                "destination": "logging.googleapis.com/projects/my-project/locations/global/buckets/_Default",
                "filter": "severity >= ERROR",
                "createTime": "2026-01-01T00:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.get_sink("my-project", "my-sink").await;
        assert!(result.is_ok());
        let sink = result.unwrap();
        assert_eq!(sink.name, "my-sink");
        assert_eq!(sink.create_time.as_deref(), Some("2026-01-01T00:00:00Z"));
    }

    #[tokio::test]
    async fn test_create_sink() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/v2/projects/my-project/sinks")
            .returning_json(json!({
                "name": "my-new-sink",
                "destination": "logging.googleapis.com/projects/my-project/locations/global/buckets/_Default",
                "filter": "severity >= ERROR",
                "createTime": "2026-01-01T00:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let sink = crate::types::logging::LogSink {
            name: "my-new-sink".to_string(),
            destination:
                "logging.googleapis.com/projects/my-project/locations/global/buckets/_Default"
                    .to_string(),
            filter: Some("severity >= ERROR".to_string()),
            ..Default::default()
        };
        let result = logging.create_sink("my-project", &sink).await;
        assert!(result.is_ok());
        let created = result.unwrap();
        assert_eq!(created.name, "my-new-sink");
        assert!(created.destination.contains("_Default"));
    }

    #[tokio::test]
    async fn test_delete_sink() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/v2/projects/my-project/sinks/my-sink")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.delete_sink("my-project", "my-sink").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_metrics() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/metrics?pageSize=100")
            .returning_json(json!({
                "metrics": [
                    {
                        "name": "my-metric",
                        "filter": "severity >= ERROR",
                        "description": "Test metric",
                        "createTime": "2026-01-01T00:00:00Z"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.list_metrics("my-project").await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "my-metric");
        assert_eq!(metrics[0].filter, "severity >= ERROR");
    }

    #[tokio::test]
    async fn test_get_metric() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/metrics/my-metric")
            .returning_json(json!({
                "name": "my-metric",
                "filter": "severity >= ERROR",
                "description": "Test metric",
                "createTime": "2026-01-01T00:00:00Z",
                "updateTime": "2026-01-02T00:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.get_metric("my-project", "my-metric").await;
        assert!(result.is_ok());
        let metric = result.unwrap();
        assert_eq!(metric.name, "my-metric");
        assert_eq!(metric.filter, "severity >= ERROR");
        assert_eq!(metric.description.as_deref(), Some("Test metric"));
        assert_eq!(metric.create_time.as_deref(), Some("2026-01-01T00:00:00Z"));
        assert_eq!(metric.update_time.as_deref(), Some("2026-01-02T00:00:00Z"));
    }

    #[tokio::test]
    async fn test_create_metric() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/v2/projects/my-project/metrics")
            .returning_json(json!({
                "name": "my-new-metric",
                "filter": "severity >= ERROR",
                "createTime": "2026-01-01T00:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let metric = crate::types::logging::LogMetric {
            name: "my-new-metric".to_string(),
            filter: "severity >= ERROR".to_string(),
            ..Default::default()
        };
        let result = logging.create_metric("my-project", &metric).await;
        assert!(result.is_ok());
        let created = result.unwrap();
        assert_eq!(created.name, "my-new-metric");
        assert_eq!(created.filter, "severity >= ERROR");
    }

    #[tokio::test]
    async fn test_delete_metric() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/v2/projects/my-project/metrics/my-metric")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let logging = client.logging();
        let result = logging.delete_metric("my-project", "my-metric").await;
        assert!(result.is_ok());
    }
}
