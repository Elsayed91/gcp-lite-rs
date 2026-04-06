//! MockClient helpers for Cloud Logging API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Logging API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait LoggingMockHelpers {
    /// Helper to expect `list_sinks`: Lists sinks.
    fn expect_list_sinks(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_sink`: Gets a sink.
    fn expect_get_sink(&mut self, sink_name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_sink`: Creates a sink that exports specified log entries to a
    /// destination. The export begins upon ingress, unless the sink's writer_identity is not
    /// permitted to write to the destination. A sink can export log entries only from the resource
    /// owning the sink.
    fn expect_create_sink(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_sink`: Deletes a sink. If the sink has a unique writer_identity,
    /// then that service account is also deleted.
    fn expect_delete_sink(&mut self, sink_name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_metrics`: Lists logs-based metrics.
    fn expect_list_metrics(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_metric`: Gets a logs-based metric.
    fn expect_get_metric(&mut self, metric_name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_metric`: Creates a logs-based metric.
    fn expect_create_metric(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_metric`: Deletes a logs-based metric.
    fn expect_delete_metric(&mut self, metric_name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl LoggingMockHelpers for MockClient {
    /// Helper to expect `list_sinks`: Lists sinks.
    fn expect_list_sinks(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v2/{parent}/sinks");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_sink`: Gets a sink.
    fn expect_get_sink(&mut self, sink_name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{sink_name}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_sink`: Creates a sink that exports specified log entries to a
    /// destination. The export begins upon ingress, unless the sink's writer_identity is not
    /// permitted to write to the destination. A sink can export log entries only from the resource
    /// owning the sink.
    fn expect_create_sink(&mut self, parent: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{parent}/sinks");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_sink`: Deletes a sink. If the sink has a unique writer_identity,
    /// then that service account is also deleted.
    fn expect_delete_sink(
        &mut self,
        sink_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{sink_name}");
        self.expect_delete(&path)
    }

    /// Helper to expect `list_metrics`: Lists logs-based metrics.
    fn expect_list_metrics(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v2/{parent}/metrics");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_metric`: Gets a logs-based metric.
    fn expect_get_metric(
        &mut self,
        metric_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{metric_name}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_metric`: Creates a logs-based metric.
    fn expect_create_metric(&mut self, parent: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{parent}/metrics");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_metric`: Deletes a logs-based metric.
    fn expect_delete_metric(
        &mut self,
        metric_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{metric_name}");
        self.expect_delete(&path)
    }
}
