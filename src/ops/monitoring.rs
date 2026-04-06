//! Operation contracts for the Cloud Monitoring API API (v3).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/monitoring.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::monitoring::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud Monitoring API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::monitoring::MonitoringClient`] instead.
pub struct MonitoringOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> MonitoringOps<'a> {
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
        "https://monitoring.googleapis.com"
    }

    /// Lists metric descriptors that match a filter.
    ///
    /// **GCP API**: `GET v3/{+name}/metricDescriptors`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The for *(required)*
    ///
    /// # Query Parameters
    /// - `activeOnly` — Optional. If true, only metrics and monitored resource types that have recent data (within roughly 25 hours) will be inc
    /// - `filter` — Optional. If this field is empty, all custom and system-defined metric descriptors are returned. Otherwise, the filter (
    /// - `pageSize` — Optional. A positive number that is the maximum number of results to return. The default and maximum value is 10,000. If
    /// - `pageToken` — Optional. If this field is not empty then it must contain the nextPageToken value returned by a previous call to this me
    ///
    /// # Response
    /// [`ListMetricDescriptorsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_metric_descriptors(
        &self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListMetricDescriptorsResponse> {
        let url = format!("{}/v3/{}/metricDescriptors", self.base_url(), name,);
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_metric_descriptors response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a single metric descriptor.
    ///
    /// **GCP API**: `GET v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The metric descriptor on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/metricDe *(required)*
    ///
    /// # Response
    /// [`MetricDescriptor`]
    #[allow(dead_code)]
    pub(crate) async fn get_metric_descriptor(&self, name: &str) -> Result<MetricDescriptor> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_metric_descriptor response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists monitored resource descriptors that match a filter.
    ///
    /// **GCP API**: `GET v3/{+name}/monitoredResourceDescriptors`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The for *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — An optional filter (https://cloud.google.com/monitoring/api/v3/filters) describing the descriptors to be returned. The f
    /// - `pageSize` — A positive number that is the maximum number of results to return.
    /// - `pageToken` — If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Usin
    ///
    /// # Response
    /// [`ListMonitoredResourceDescriptorsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_monitored_resource_descriptors(
        &self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListMonitoredResourceDescriptorsResponse> {
        let url = format!(
            "{}/v3/{}/monitoredResourceDescriptors",
            self.base_url(),
            name,
        );
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_monitored_resource_descriptors response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a single monitored resource descriptor.
    ///
    /// **GCP API**: `GET v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The monitored resource descriptor to get. The format is: projects/[PROJECT_ID_OR_NUMBER]/monitoredResourceDesc *(required)*
    ///
    /// # Response
    /// [`MonitoredResourceDescriptor`]
    #[allow(dead_code)]
    pub(crate) async fn get_monitored_resource_descriptor(
        &self,
        name: &str,
    ) -> Result<MonitoredResourceDescriptor> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_monitored_resource_descriptor response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists the existing alerting policies for the workspace.
    ///
    /// **GCP API**: `GET v3/{+name}/alertPolicies`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) whose alert policies are to be listed. T *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. If provided, this field specifies the criteria that must be met by alert policies to be included in the respon
    /// - `orderBy` — Optional. A comma-separated list of fields by which to sort the result. Supports the same set of field references as the
    /// - `pageSize` — Optional. The maximum number of results to return in a single response.
    /// - `pageToken` — Optional. If this field is not empty then it must contain the nextPageToken value returned by a previous call to this me
    ///
    /// # Response
    /// [`ListAlertPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_alert_policies(
        &self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListAlertPoliciesResponse> {
        let url = format!("{}/v3/{}/alertPolicies", self.base_url(), name,);
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_alert_policies response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new alerting policy.Design your application to single-thread API calls that
    /// modify the state of alerting policies in a single project. This includes calls to
    /// CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    ///
    /// **GCP API**: `POST v3/{+name}/alertPolicies`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the alerting policy.  *(required)*
    ///
    /// # Request Body
    /// [`AlertPolicy`]
    ///
    /// # Response
    /// [`AlertPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn create_alert_policy(
        &self,
        name: &str,
        body: &AlertPolicy,
    ) -> Result<AlertPolicy> {
        let url = format!("{}/v3/{}/alertPolicies", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_alert_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes an alerting policy.Design your application to single-thread API calls that
    /// modify the state of alerting policies in a single project. This includes calls to
    /// CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    ///
    /// **GCP API**: `DELETE v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The alerting policy to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID]  *(required)*
    ///
    /// # Response
    /// [`MonitoringEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_alert_policy(&self, name: &str) -> Result<MonitoringEmpty> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_alert_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists the notification channels that have been created for the project. To list the
    /// types of notification channels that are supported, use the
    /// ListNotificationChannelDescriptors method.
    ///
    /// **GCP API**: `GET v3/{+name}/notificationChannels`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The for *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. If provided, this field specifies the criteria that must be met by notification channels to be included in the
    /// - `orderBy` — Optional. A comma-separated list of fields by which to sort the result. Supports the same set of fields as in filter. En
    /// - `pageSize` — Optional. The maximum number of results to return in a single response. If not set to a positive number, a reasonable va
    /// - `pageToken` — Optional. If non-empty, page_token must contain a value returned as the next_page_token in a previous response to reques
    ///
    /// # Response
    /// [`ListNotificationChannelsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_notification_channels(
        &self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListNotificationChannelsResponse> {
        let url = format!("{}/v3/{}/notificationChannels", self.base_url(), name,);
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_notification_channels response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new notification channel, representing a single notification endpoint such as
    /// an email address, SMS number, or PagerDuty service.Design your application to single-
    /// thread API calls that modify the state of notification channels in a single project.
    /// This includes calls to CreateNotificationChannel, DeleteNotificationChannel and
    /// UpdateNotificationChannel.
    ///
    /// **GCP API**: `POST v3/{+name}/notificationChannels`
    ///
    /// # Path Parameters
    /// - `name` — Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The for *(required)*
    ///
    /// # Request Body
    /// [`NotificationChannel`]
    ///
    /// # Response
    /// [`NotificationChannel`]
    #[allow(dead_code)]
    pub(crate) async fn create_notification_channel(
        &self,
        name: &str,
        body: &NotificationChannel,
    ) -> Result<NotificationChannel> {
        let url = format!("{}/v3/{}/notificationChannels", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_notification_channel response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a notification channel.Design your application to single-thread API calls that
    /// modify the state of notification channels in a single project. This includes calls to
    /// CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    ///
    /// **GCP API**: `DELETE v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The channel for which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChann *(required)*
    ///
    /// # Query Parameters
    /// - `force` — If true, the notification channel will be deleted regardless of its use in alert policies (the policies will be updated
    ///
    /// # Response
    /// [`MonitoringEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_notification_channel(&self, name: &str) -> Result<MonitoringEmpty> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_notification_channel response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_metric_descriptors() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name/metricDescriptors?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListMetricDescriptorsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops
            .list_metric_descriptors(
                "test-name",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_metric_descriptor() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name")
            .returning_json(serde_json::to_value(MetricDescriptor::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops.get_metric_descriptor("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_monitored_resource_descriptors() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name/monitoredResourceDescriptors?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListMonitoredResourceDescriptorsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops
            .list_monitored_resource_descriptors(
                "test-name",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_monitored_resource_descriptor() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name")
            .returning_json(serde_json::to_value(MonitoredResourceDescriptor::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops.get_monitored_resource_descriptor("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_alert_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name/alertPolicies?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListAlertPoliciesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops
            .list_alert_policies(
                "test-name",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_alert_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-name/alertPolicies")
            .returning_json(serde_json::to_value(AlertPolicy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let body = AlertPolicy::fixture();
        let result = ops.create_alert_policy("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_alert_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v3/test-name")
            .returning_json(serde_json::to_value(MonitoringEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops.delete_alert_policy("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_notification_channels() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name/notificationChannels?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListNotificationChannelsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops
            .list_notification_channels(
                "test-name",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_notification_channel() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-name/notificationChannels")
            .returning_json(serde_json::to_value(NotificationChannel::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let body = NotificationChannel::fixture();
        let result = ops.create_notification_channel("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_notification_channel() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v3/test-name")
            .returning_json(serde_json::to_value(MonitoringEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = MonitoringOps::new(&client);

        let result = ops.delete_notification_channel("test-name").await;
        assert!(result.is_ok());
    }
}
