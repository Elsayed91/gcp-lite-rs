//! MockClient helpers for Cloud Monitoring API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Monitoring API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait MonitoringMockHelpers {
    /// Helper to expect `list_metric_descriptors`: Lists metric descriptors that match a filter.
    fn expect_list_metric_descriptors(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_metric_descriptor`: Gets a single metric descriptor.
    fn expect_get_metric_descriptor(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_monitored_resource_descriptors`: Lists monitored resource descriptors
    /// that match a filter.
    fn expect_list_monitored_resource_descriptors(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_monitored_resource_descriptor`: Gets a single monitored resource
    /// descriptor.
    fn expect_get_monitored_resource_descriptor(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_alert_policies`: Lists the existing alerting policies for the
    /// workspace.
    fn expect_list_alert_policies(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_alert_policy`: Creates a new alerting policy.Design your
    /// application to single-thread API calls that modify the state of alerting policies in a
    /// single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and
    /// UpdateAlertPolicy.
    fn expect_create_alert_policy(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_alert_policy`: Deletes an alerting policy.Design your application
    /// to single-thread API calls that modify the state of alerting policies in a single project.
    /// This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    fn expect_delete_alert_policy(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_notification_channels`: Lists the notification channels that have
    /// been created for the project. To list the types of notification channels that are supported,
    /// use the ListNotificationChannelDescriptors method.
    fn expect_list_notification_channels(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_notification_channel`: Creates a new notification channel,
    /// representing a single notification endpoint such as an email address, SMS number, or
    /// PagerDuty service.Design your application to single-thread API calls that modify the state
    /// of notification channels in a single project. This includes calls to
    /// CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    fn expect_create_notification_channel(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_notification_channel`: Deletes a notification channel.Design your
    /// application to single-thread API calls that modify the state of notification channels in a
    /// single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel
    /// and UpdateNotificationChannel.
    fn expect_delete_notification_channel(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl MonitoringMockHelpers for MockClient {
    /// Helper to expect `list_metric_descriptors`: Lists metric descriptors that match a filter.
    fn expect_list_metric_descriptors(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v3/{name}/metricDescriptors");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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

    /// Helper to expect `get_metric_descriptor`: Gets a single metric descriptor.
    fn expect_get_metric_descriptor(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_monitored_resource_descriptors`: Lists monitored resource descriptors
    /// that match a filter.
    fn expect_list_monitored_resource_descriptors(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v3/{name}/monitoredResourceDescriptors");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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

    /// Helper to expect `get_monitored_resource_descriptor`: Gets a single monitored resource
    /// descriptor.
    fn expect_get_monitored_resource_descriptor(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_alert_policies`: Lists the existing alerting policies for the
    /// workspace.
    fn expect_list_alert_policies(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v3/{name}/alertPolicies");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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

    /// Helper to expect `create_alert_policy`: Creates a new alerting policy.Design your
    /// application to single-thread API calls that modify the state of alerting policies in a
    /// single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and
    /// UpdateAlertPolicy.
    fn expect_create_alert_policy(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}/alertPolicies");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_alert_policy`: Deletes an alerting policy.Design your application
    /// to single-thread API calls that modify the state of alerting policies in a single project.
    /// This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    fn expect_delete_alert_policy(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_delete(&path)
    }

    /// Helper to expect `list_notification_channels`: Lists the notification channels that have
    /// been created for the project. To list the types of notification channels that are supported,
    /// use the ListNotificationChannelDescriptors method.
    fn expect_list_notification_channels(
        &mut self,
        name: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v3/{name}/notificationChannels");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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

    /// Helper to expect `create_notification_channel`: Creates a new notification channel,
    /// representing a single notification endpoint such as an email address, SMS number, or
    /// PagerDuty service.Design your application to single-thread API calls that modify the state
    /// of notification channels in a single project. This includes calls to
    /// CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    fn expect_create_notification_channel(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}/notificationChannels");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_notification_channel`: Deletes a notification channel.Design your
    /// application to single-thread API calls that modify the state of notification channels in a
    /// single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel
    /// and UpdateNotificationChannel.
    fn expect_delete_notification_channel(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_delete(&path)
    }
}
