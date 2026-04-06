//! Cloud Monitoring API client.
//!
//! Provides read-only access to GCP metrics for policy engine use:
//! - Query time series data (`timeSeries.list`)
//! - Discover available metrics (`metricDescriptors.list/get`)
//! - Discover monitored resource types (`monitoredResourceDescriptors.list/get`)
//!
//! `timeSeries.list` is implemented directly (not via ops) because the
//! Monitoring API uses dotted query params (`interval.startTime`,
//! `aggregation.alignmentPeriod`) that the codegen pipeline cannot express.

use crate::ops::monitoring::MonitoringOps;
use crate::types::monitoring::*;
use crate::{GcpHttpClient, Result};

/// Cloud Monitoring API client.
pub struct MonitoringClient<'a> {
    ops: MonitoringOps<'a>,
    client: &'a GcpHttpClient,
}

impl<'a> MonitoringClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: MonitoringOps::new(client),
            client,
        }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://monitoring.googleapis.com"
    }

    // ── Time Series ──────────────────────────────────────────────────────

    /// Lists time series that match a filter.
    ///
    /// This is implemented directly (bypassing ops) because the Monitoring API
    /// uses dotted query params (`interval.startTime`, `aggregation.alignmentPeriod`)
    /// that the codegen cannot express as valid Rust identifiers.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID (e.g., "my-project").
    /// * `filter` - A monitoring filter string (e.g., `metric.type="compute.googleapis.com/instance/cpu/utilization"`).
    /// * `interval` - The time interval for the query. `end_time` is required.
    /// * `params` - Optional additional parameters (aggregation, pagination, etc.).
    pub async fn list_time_series(
        &self,
        project: &str,
        filter: &str,
        interval: &TimeInterval,
        params: Option<&TimeSeriesParams>,
    ) -> Result<ListTimeSeriesResponse> {
        let url = format!("{}/v3/projects/{}/timeSeries", self.base_url(), project,);

        let mut query_parts: Vec<(&str, String)> = Vec::new();
        query_parts.push(("filter", filter.to_string()));

        if let Some(ref end_time) = interval.end_time {
            query_parts.push(("interval.endTime", end_time.clone()));
        }
        if let Some(ref start_time) = interval.start_time {
            query_parts.push(("interval.startTime", start_time.clone()));
        }

        if let Some(p) = params {
            if let Some(ref period) = p.aggregation_alignment_period {
                query_parts.push(("aggregation.alignmentPeriod", period.clone()));
            }
            if let Some(ref aligner) = p.aggregation_per_series_aligner {
                query_parts.push(("aggregation.perSeriesAligner", aligner.clone()));
            }
            if let Some(ref reducer) = p.aggregation_cross_series_reducer {
                query_parts.push(("aggregation.crossSeriesReducer", reducer.clone()));
            }
            if let Some(ref fields) = p.aggregation_group_by_fields {
                for field in fields {
                    query_parts.push(("aggregation.groupByFields", field.clone()));
                }
            }
            if let Some(ref view) = p.view {
                query_parts.push(("view", view.clone()));
            }
            if let Some(size) = p.page_size {
                query_parts.push(("pageSize", size.to_string()));
            }
            if let Some(ref token) = p.page_token {
                query_parts.push(("pageToken", token.clone()));
            }
            if let Some(ref order) = p.order_by {
                query_parts.push(("orderBy", order.clone()));
            }
        }

        let mut full_url = url;
        if !query_parts.is_empty() {
            let qs: Vec<String> = query_parts
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect();
            full_url = format!("{}?{}", full_url, qs.join("&"));
        }

        let response = self.client.get(&full_url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_time_series response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns a stream of time series matching a filter, automatically paginating.
    pub fn list_time_series_stream(
        &self,
        project: &str,
        filter: &str,
        interval: &TimeInterval,
        params: Option<TimeSeriesParams>,
    ) -> impl futures::Stream<Item = Result<TimeSeries>> + '_ {
        let project = project.to_string();
        let filter = filter.to_string();
        let interval = interval.clone();
        let mut params = params.unwrap_or_default();
        async_stream::try_stream! {
            loop {
                let response = self
                    .list_time_series(&project, &filter, &interval, Some(&params))
                    .await?;
                for ts in response.time_series {
                    yield ts;
                }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => params.page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    // ── Metric Descriptors ───────────────────────────────────────────────

    /// Lists metric descriptors that match a filter.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `filter` - Optional filter (e.g., `metric.type = starts_with("compute.googleapis.com/")`).
    /// * `page_token` - Optional pagination token.
    pub async fn list_metric_descriptors(
        &self,
        project: &str,
        filter: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<ListMetricDescriptorsResponse> {
        let name = format!("projects/{}", project);
        self.ops
            .list_metric_descriptors(&name, filter.unwrap_or(""), "", page_token.unwrap_or(""))
            .await
    }

    /// Returns a stream of metric descriptors, automatically paginating.
    pub fn list_metric_descriptors_stream(
        &self,
        project: &str,
        filter: Option<&str>,
    ) -> impl futures::Stream<Item = Result<MetricDescriptor>> + '_ {
        let project = project.to_string();
        let filter = filter.unwrap_or("").to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self
                    .list_metric_descriptors(&project, Some(&filter), page_token.as_deref())
                    .await?;
                for desc in response.metric_descriptors {
                    yield desc;
                }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Gets a single metric descriptor by its full type name.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `metric_type` - The metric type (e.g., "compute.googleapis.com/instance/cpu/utilization").
    pub async fn get_metric_descriptor(
        &self,
        project: &str,
        metric_type: &str,
    ) -> Result<MetricDescriptor> {
        let name = format!("projects/{}/metricDescriptors/{}", project, metric_type);
        self.ops.get_metric_descriptor(&name).await
    }

    // ── Monitored Resource Descriptors ───────────────────────────────────

    /// Lists monitored resource descriptors that match a filter.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `filter` - Optional filter string.
    /// * `page_token` - Optional pagination token.
    pub async fn list_monitored_resource_descriptors(
        &self,
        project: &str,
        filter: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<ListMonitoredResourceDescriptorsResponse> {
        let name = format!("projects/{}", project);
        self.ops
            .list_monitored_resource_descriptors(
                &name,
                filter.unwrap_or(""),
                "",
                page_token.unwrap_or(""),
            )
            .await
    }

    /// Returns a stream of monitored resource descriptors, automatically paginating.
    pub fn list_monitored_resource_descriptors_stream(
        &self,
        project: &str,
        filter: Option<&str>,
    ) -> impl futures::Stream<Item = Result<MonitoredResourceDescriptor>> + '_ {
        let project = project.to_string();
        let filter = filter.unwrap_or("").to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self
                    .list_monitored_resource_descriptors(
                        &project,
                        Some(&filter),
                        page_token.as_deref(),
                    )
                    .await?;
                for desc in response.resource_descriptors {
                    yield desc;
                }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Gets a single monitored resource descriptor.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `resource_type` - The resource type (e.g., "gce_instance").
    pub async fn get_monitored_resource_descriptor(
        &self,
        project: &str,
        resource_type: &str,
    ) -> Result<MonitoredResourceDescriptor> {
        let name = format!(
            "projects/{}/monitoredResourceDescriptors/{}",
            project, resource_type
        );
        self.ops.get_monitored_resource_descriptor(&name).await
    }

    // ── Alert Policies ───────────────────────────────────────────────────────

    /// Lists alerting policies for a project, automatically paginating.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `filter` - Optional filter string (e.g., `displayName="my-alert"`).
    pub async fn list_alert_policies(
        &self,
        project: &str,
        filter: Option<&str>,
    ) -> Result<Vec<AlertPolicy>> {
        let name = format!("projects/{}", project);
        let mut policies = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_alert_policies(&name, filter.unwrap_or(""), "100", &page_token)
                .await?;
            policies.extend(resp.alert_policies);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(policies)
    }

    /// Creates a new alerting policy.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `policy` - The alert policy to create.
    pub async fn create_alert_policy(
        &self,
        project: &str,
        policy: &AlertPolicy,
    ) -> Result<AlertPolicy> {
        let name = format!("projects/{}", project);
        self.ops.create_alert_policy(&name, policy).await
    }

    /// Deletes an alerting policy by its full resource name.
    ///
    /// # Arguments
    ///
    /// * `policy_name` - Full resource name (e.g., `projects/{project}/alertPolicies/{id}`).
    pub async fn delete_alert_policy(&self, policy_name: &str) -> Result<()> {
        self.ops.delete_alert_policy(policy_name).await?;
        Ok(())
    }

    // ── Notification Channels ────────────────────────────────────────────────

    /// Lists notification channels for a project, automatically paginating.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `filter` - Optional filter string.
    pub async fn list_notification_channels(
        &self,
        project: &str,
        filter: Option<&str>,
    ) -> Result<Vec<NotificationChannel>> {
        let name = format!("projects/{}", project);
        let mut channels = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_notification_channels(&name, filter.unwrap_or(""), "100", &page_token)
                .await?;
            channels.extend(resp.notification_channels);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(channels)
    }

    /// Creates a new notification channel.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `channel` - The notification channel to create.
    pub async fn create_notification_channel(
        &self,
        project: &str,
        channel: &NotificationChannel,
    ) -> Result<NotificationChannel> {
        let name = format!("projects/{}", project);
        self.ops.create_notification_channel(&name, channel).await
    }

    /// Deletes a notification channel by its full resource name.
    ///
    /// # Arguments
    ///
    /// * `channel_name` - Full resource name (e.g., `projects/{project}/notificationChannels/{id}`).
    pub async fn delete_notification_channel(&self, channel_name: &str) -> Result<()> {
        self.ops.delete_notification_channel(channel_name).await?;
        Ok(())
    }
}

/// Optional parameters for [`MonitoringClient::list_time_series`].
#[derive(Debug, Clone, Default)]
pub struct TimeSeriesParams {
    /// Alignment period (e.g., "60s", "3600s").
    pub aggregation_alignment_period: Option<String>,
    /// Per-series aligner (e.g., "ALIGN_MEAN", "ALIGN_RATE").
    pub aggregation_per_series_aligner: Option<String>,
    /// Cross-series reducer (e.g., "REDUCE_SUM", "REDUCE_MEAN").
    pub aggregation_cross_series_reducer: Option<String>,
    /// Fields to group by for cross-series reduction.
    pub aggregation_group_by_fields: Option<Vec<String>>,
    /// View: "FULL" (default) or "HEADERS".
    pub view: Option<String>,
    /// Maximum number of time series to return.
    pub page_size: Option<i32>,
    /// Pagination token from a previous response.
    pub page_token: Option<String>,
    /// Order by clause (e.g., "metric.type").
    pub order_by: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use std::collections::HashMap;

    fn metric_descriptor_json(name: &str, metric_type: &str) -> serde_json::Value {
        serde_json::to_value(MetricDescriptor {
            name: name.to_string(),
            metric_type: Some(metric_type.to_string()),
            metric_kind: Some("GAUGE".to_string()),
            value_type: Some("DOUBLE".to_string()),
            ..Default::default()
        })
        .unwrap()
    }

    fn monitored_resource_descriptor_json(name: &str, resource_type: &str) -> serde_json::Value {
        serde_json::to_value(MonitoredResourceDescriptor {
            name: name.to_string(),
            resource_type: Some(resource_type.to_string()),
            display_name: Some("Test Resource".to_string()),
            ..Default::default()
        })
        .unwrap()
    }

    fn time_series_json(metric_type: &str) -> serde_json::Value {
        serde_json::to_value(TimeSeries {
            metric: Some(Metric {
                metric_type: Some(metric_type.to_string()),
                labels: HashMap::from([("instance_name".to_string(), "test-vm".to_string())]),
            }),
            resource: Some(MonitoredResource {
                resource_type: Some("gce_instance".to_string()),
                labels: HashMap::from([("project_id".to_string(), "test-project".to_string())]),
            }),
            points: vec![Point {
                interval: Some(TimeInterval {
                    start_time: Some("2026-01-01T00:00:00Z".to_string()),
                    end_time: Some("2026-01-01T01:00:00Z".to_string()),
                }),
                value: Some(TypedValue {
                    double_value: Some(0.42),
                    ..Default::default()
                }),
            }],
            ..Default::default()
        })
        .unwrap()
    }

    // ── Alert Policies ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_alert_policies() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/alertPolicies?pageSize=100")
            .returning_json(serde_json::json!({
                "alertPolicies": [{
                    "name": "projects/test-project/alertPolicies/123",
                    "displayName": "test-alert",
                    "enabled": false,
                    "combiner": "OR"
                }],
                "totalSize": 1
            }));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let result = monitoring
            .list_alert_policies("test-project", None)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].display_name.as_deref(), Some("test-alert"));
        assert_eq!(result[0].enabled, Some(false));
        assert_eq!(result[0].combiner.as_deref(), Some("OR"));
    }

    #[tokio::test]
    async fn test_list_alert_policies_with_filter() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/alertPolicies?filter=displayName%3D%22my-alert%22&pageSize=100")
            .returning_json(serde_json::json!({
                "alertPolicies": [],
                "totalSize": 0
            }));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let result = monitoring
            .list_alert_policies("test-project", Some("displayName=\"my-alert\""))
            .await
            .unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_create_alert_policy() {
        let mut mock = MockClient::new();

        mock.expect_post("/v3/projects/test-project/alertPolicies")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/alertPolicies/456",
                "displayName": "new-alert",
                "enabled": false,
                "combiner": "OR",
                "conditions": [{
                    "name": "projects/test-project/alertPolicies/456/conditions/789",
                    "displayName": "Test condition",
                    "conditionThreshold": {
                        "filter": "resource.type=\"consumed_api\" AND metric.type=\"serviceruntime.googleapis.com/api/request_count\"",
                        "comparison": "COMPARISON_GT",
                        "thresholdValue": 1000000,
                        "duration": "60s"
                    }
                }]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let policy = AlertPolicy {
            display_name: Some("new-alert".to_string()),
            enabled: Some(false),
            combiner: Some("OR".to_string()),
            conditions: vec![Condition {
                display_name: Some("Test condition".to_string()),
                condition_threshold: Some(serde_json::json!({
                    "filter": "resource.type=\"consumed_api\" AND metric.type=\"serviceruntime.googleapis.com/api/request_count\"",
                    "comparison": "COMPARISON_GT",
                    "thresholdValue": 1000000,
                    "duration": "60s",
                    "aggregations": [{"alignmentPeriod": "60s", "perSeriesAligner": "ALIGN_RATE"}]
                })),
                ..Default::default()
            }],
            ..Default::default()
        };
        let result = monitoring
            .create_alert_policy("test-project", &policy)
            .await
            .unwrap();
        assert_eq!(
            result.name.as_deref(),
            Some("projects/test-project/alertPolicies/456")
        );
        assert_eq!(result.display_name.as_deref(), Some("new-alert"));
        assert_eq!(result.conditions.len(), 1);
        assert_eq!(
            result.conditions[0].display_name.as_deref(),
            Some("Test condition")
        );
    }

    #[tokio::test]
    async fn test_delete_alert_policy() {
        let mut mock = MockClient::new();

        mock.expect_delete("/v3/projects/test-project/alertPolicies/123")
            .returning_json(serde_json::json!({}));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let result = monitoring
            .delete_alert_policy("projects/test-project/alertPolicies/123")
            .await;
        assert!(result.is_ok());
    }

    // ── Notification Channels ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_notification_channels() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/notificationChannels?pageSize=100")
            .returning_json(serde_json::json!({
                "notificationChannels": [{
                    "name": "projects/test-project/notificationChannels/111",
                    "displayName": "test-email-channel",
                    "type": "email",
                    "enabled": false,
                    "labels": {"email_address": "test@example.com"},
                    "verificationStatus": "UNVERIFIED"
                }],
                "totalSize": 1
            }));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let result = monitoring
            .list_notification_channels("test-project", None)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].display_name.as_deref(),
            Some("test-email-channel")
        );
        assert_eq!(result[0].channel_type.as_deref(), Some("email"));
        assert_eq!(result[0].enabled, Some(false));
        assert_eq!(result[0].verification_status.as_deref(), Some("UNVERIFIED"));
    }

    #[tokio::test]
    async fn test_create_notification_channel() {
        let mut mock = MockClient::new();

        mock.expect_post("/v3/projects/test-project/notificationChannels")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/notificationChannels/222",
                "displayName": "new-email-channel",
                "type": "email",
                "enabled": false,
                "labels": {"email_address": "noreply@cloud-lite-test.invalid"},
                "verificationStatus": "UNVERIFIED"
            }));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let mut labels = HashMap::new();
        labels.insert(
            "email_address".to_string(),
            "noreply@cloud-lite-test.invalid".to_string(),
        );
        let channel = NotificationChannel {
            channel_type: Some("email".to_string()),
            display_name: Some("new-email-channel".to_string()),
            enabled: Some(false),
            labels: Some(labels),
            ..Default::default()
        };
        let result = monitoring
            .create_notification_channel("test-project", &channel)
            .await
            .unwrap();
        assert_eq!(
            result.name.as_deref(),
            Some("projects/test-project/notificationChannels/222")
        );
        assert_eq!(result.channel_type.as_deref(), Some("email"));
        assert_eq!(result.display_name.as_deref(), Some("new-email-channel"));
        let labels = result.labels.as_ref().unwrap();
        assert_eq!(
            labels.get("email_address").map(|s| s.as_str()),
            Some("noreply@cloud-lite-test.invalid")
        );
    }

    #[tokio::test]
    async fn test_delete_notification_channel() {
        let mut mock = MockClient::new();

        mock.expect_delete("/v3/projects/test-project/notificationChannels/111")
            .returning_json(serde_json::json!({}));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let result = monitoring
            .delete_notification_channel("projects/test-project/notificationChannels/111")
            .await;
        assert!(result.is_ok());
    }

    // ── Metric Descriptors ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_metric_descriptors() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/metricDescriptors")
            .returning_json(
                serde_json::to_value(ListMetricDescriptorsResponse {
                    metric_descriptors: vec![
                        MetricDescriptor {
                            name: "projects/test-project/metricDescriptors/compute.googleapis.com/instance/cpu/utilization".to_string(),
                            metric_type: Some("compute.googleapis.com/instance/cpu/utilization".to_string()),
                            ..Default::default()
                        },
                    ],
                    next_page_token: None,
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let response = monitoring
            .list_metric_descriptors("test-project", None, None)
            .await
            .unwrap();
        assert_eq!(response.metric_descriptors.len(), 1);
        assert_eq!(
            response.metric_descriptors[0].metric_type.as_deref(),
            Some("compute.googleapis.com/instance/cpu/utilization")
        );
    }

    #[tokio::test]
    async fn test_list_metric_descriptors_with_filter() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/metricDescriptors?filter=metric.type%20%3D%20starts_with%28%22compute%22%29")
            .returning_json(
                serde_json::to_value(ListMetricDescriptorsResponse {
                    metric_descriptors: vec![],
                    next_page_token: None,
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let response = monitoring
            .list_metric_descriptors(
                "test-project",
                Some("metric.type = starts_with(\"compute\")"),
                None,
            )
            .await
            .unwrap();
        assert!(response.metric_descriptors.is_empty());
    }

    #[tokio::test]
    async fn test_get_metric_descriptor() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/metricDescriptors/compute.googleapis.com/instance/cpu/utilization")
            .returning_json(metric_descriptor_json(
                "projects/test-project/metricDescriptors/compute.googleapis.com/instance/cpu/utilization",
                "compute.googleapis.com/instance/cpu/utilization",
            ));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let desc = monitoring
            .get_metric_descriptor(
                "test-project",
                "compute.googleapis.com/instance/cpu/utilization",
            )
            .await
            .unwrap();
        assert_eq!(
            desc.metric_type.as_deref(),
            Some("compute.googleapis.com/instance/cpu/utilization")
        );
        assert_eq!(desc.metric_kind.as_deref(), Some("GAUGE"));
    }

    // ── Monitored Resource Descriptors ───────────────────────────────

    #[tokio::test]
    async fn test_list_monitored_resource_descriptors() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/monitoredResourceDescriptors")
            .returning_json(
                serde_json::to_value(ListMonitoredResourceDescriptorsResponse {
                    resource_descriptors: vec![MonitoredResourceDescriptor {
                        name: "projects/test-project/monitoredResourceDescriptors/gce_instance"
                            .to_string(),
                        resource_type: Some("gce_instance".to_string()),
                        ..Default::default()
                    }],
                    next_page_token: None,
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let response = monitoring
            .list_monitored_resource_descriptors("test-project", None, None)
            .await
            .unwrap();
        assert_eq!(response.resource_descriptors.len(), 1);
        assert_eq!(
            response.resource_descriptors[0].resource_type.as_deref(),
            Some("gce_instance")
        );
    }

    #[tokio::test]
    async fn test_get_monitored_resource_descriptor() {
        let mut mock = MockClient::new();

        mock.expect_get("/v3/projects/test-project/monitoredResourceDescriptors/gce_instance")
            .returning_json(monitored_resource_descriptor_json(
                "projects/test-project/monitoredResourceDescriptors/gce_instance",
                "gce_instance",
            ));

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let desc = monitoring
            .get_monitored_resource_descriptor("test-project", "gce_instance")
            .await
            .unwrap();
        assert_eq!(desc.resource_type.as_deref(), Some("gce_instance"));
        assert_eq!(desc.display_name.as_deref(), Some("Test Resource"));
    }

    // ── Time Series ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_time_series_basic() {
        let mut mock = MockClient::new();

        // The URL should contain encoded filter and interval params
        mock.expect_get("/v3/projects/test-project/timeSeries?filter=metric.type%3D%22cpu%22&interval.endTime=2026-01-01T01%3A00%3A00Z&interval.startTime=2026-01-01T00%3A00%3A00Z")
            .returning_json(
                serde_json::to_value(ListTimeSeriesResponse {
                    time_series: vec![
                        serde_json::from_value(time_series_json("cpu")).unwrap(),
                    ],
                    next_page_token: None,
                    unit: Some("1".to_string()),
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let interval = TimeInterval {
            start_time: Some("2026-01-01T00:00:00Z".to_string()),
            end_time: Some("2026-01-01T01:00:00Z".to_string()),
        };

        let response = monitoring
            .list_time_series("test-project", "metric.type=\"cpu\"", &interval, None)
            .await
            .unwrap();
        assert_eq!(response.time_series.len(), 1);

        let ts = &response.time_series[0];
        assert!(ts.metric.is_some());
        assert_eq!(ts.points.len(), 1);
        assert_eq!(
            ts.points[0].value.as_ref().unwrap().double_value,
            Some(0.42)
        );
    }

    #[tokio::test]
    async fn test_list_time_series_with_aggregation() {
        let mut mock = MockClient::new();

        // Verify aggregation params are included in URL
        mock.expect_get("/v3/projects/test-project/timeSeries?filter=metric.type%3D%22cpu%22&interval.endTime=2026-01-01T01%3A00%3A00Z&aggregation.alignmentPeriod=60s&aggregation.perSeriesAligner=ALIGN_RATE")
            .returning_json(
                serde_json::to_value(ListTimeSeriesResponse {
                    time_series: vec![],
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let interval = TimeInterval {
            start_time: None,
            end_time: Some("2026-01-01T01:00:00Z".to_string()),
        };
        let params = TimeSeriesParams {
            aggregation_alignment_period: Some("60s".to_string()),
            aggregation_per_series_aligner: Some("ALIGN_RATE".to_string()),
            ..Default::default()
        };

        let response = monitoring
            .list_time_series(
                "test-project",
                "metric.type=\"cpu\"",
                &interval,
                Some(&params),
            )
            .await
            .unwrap();
        assert!(response.time_series.is_empty());
    }

    #[tokio::test]
    async fn test_list_time_series_with_group_by() {
        let mut mock = MockClient::new();

        // Verify repeated aggregation.groupByFields params
        mock.expect_get("/v3/projects/test-project/timeSeries?filter=metric.type%3D%22cpu%22&interval.endTime=2026-01-01T01%3A00%3A00Z&aggregation.alignmentPeriod=3600s&aggregation.crossSeriesReducer=REDUCE_SUM&aggregation.groupByFields=resource.labels.zone&aggregation.groupByFields=resource.labels.project_id")
            .returning_json(
                serde_json::to_value(ListTimeSeriesResponse {
                    time_series: vec![],
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();

        let interval = TimeInterval {
            start_time: None,
            end_time: Some("2026-01-01T01:00:00Z".to_string()),
        };
        let params = TimeSeriesParams {
            aggregation_alignment_period: Some("3600s".to_string()),
            aggregation_cross_series_reducer: Some("REDUCE_SUM".to_string()),
            aggregation_group_by_fields: Some(vec![
                "resource.labels.zone".to_string(),
                "resource.labels.project_id".to_string(),
            ]),
            ..Default::default()
        };

        let response = monitoring
            .list_time_series(
                "test-project",
                "metric.type=\"cpu\"",
                &interval,
                Some(&params),
            )
            .await
            .unwrap();
        assert!(response.time_series.is_empty());
    }

    // ── Pagination ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_metric_descriptors_stream_paginates() {
        use futures::StreamExt;

        let mut mock = MockClient::new();

        // Page 2 (specific path first for mock matching)
        mock.expect_get("/v3/projects/test-project/metricDescriptors?pageToken=tok2")
            .returning_json(
                serde_json::to_value(ListMetricDescriptorsResponse {
                    metric_descriptors: vec![MetricDescriptor {
                        name: "desc-3".to_string(),
                        metric_type: Some("m3".to_string()),
                        ..Default::default()
                    }],
                    next_page_token: None,
                })
                .unwrap(),
            );

        // Page 1
        mock.expect_get("/v3/projects/test-project/metricDescriptors")
            .returning_json(
                serde_json::to_value(ListMetricDescriptorsResponse {
                    metric_descriptors: vec![
                        MetricDescriptor {
                            name: "desc-1".to_string(),
                            metric_type: Some("m1".to_string()),
                            ..Default::default()
                        },
                        MetricDescriptor {
                            name: "desc-2".to_string(),
                            metric_type: Some("m2".to_string()),
                            ..Default::default()
                        },
                    ],
                    next_page_token: Some("tok2".to_string()),
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let monitoring = client.monitoring();
        let stream = monitoring.list_metric_descriptors_stream("test-project", None);
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(desc)) = stream.next().await {
            names.push(desc.name);
        }
        assert_eq!(names, vec!["desc-1", "desc-2", "desc-3"]);
    }
}
