//! App Engine Admin API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::appengine::AppengineOps`. This layer adds ergonomic method
//! signatures (project/app/service IDs instead of raw resource paths) and
//! auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 4.10 (compute_app_engine_https): verify services enforce HTTPS connections
//!     by inspecting networkSettings.ingressTrafficAllowed.

use crate::{
    GcpHttpClient, Result,
    ops::appengine::AppengineOps,
    types::appengine::{Application, Service},
};

/// Client for the App Engine Admin API.
pub struct AppEngineClient<'a> {
    ops: AppengineOps<'a>,
}

impl<'a> AppEngineClient<'a> {
    /// Create a new App Engine client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: AppengineOps::new(client),
        }
    }

    // ── Application ───────────────────────────────────────────────────────

    /// Get the App Engine application for a GCP project.
    ///
    /// In App Engine, the app ID is always equal to the GCP project ID.
    /// Returns the application metadata including `sslPolicy` and `servingStatus`.
    pub async fn get_app(&self, project: &str) -> Result<Application> {
        self.ops.get_app(project).await
    }

    // ── Services ──────────────────────────────────────────────────────────

    /// List all services for an App Engine application (auto-paginated).
    ///
    /// The app ID is equal to the GCP project ID.
    /// Each service includes `networkSettings.ingressTrafficAllowed`
    /// which CIS 4.10 uses to verify HTTPS enforcement.
    pub async fn list_services(&self, app_id: &str) -> Result<Vec<Service>> {
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_services(app_id, "100", &page_token).await?;
            all.extend(resp.services);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get the current configuration of a specific App Engine service.
    ///
    /// The app ID is equal to the GCP project ID.
    pub async fn get_service(&self, app_id: &str, service_id: &str) -> Result<Service> {
        self.ops.get_service(app_id, service_id).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_get_app() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/apps/my-project")
            .returning_json(json!({
                "name": "apps/my-project",
                "id": "my-project",
                "locationId": "us-central",
                "servingStatus": "SERVING",
                "defaultHostname": "my-project.appspot.com",
                "sslPolicy": "DEFAULT"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let ae = client.appengine();
        let result = ae.get_app("my-project").await;
        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.name, "apps/my-project");
        assert_eq!(app.id.as_deref(), Some("my-project"));
        assert_eq!(app.location_id.as_deref(), Some("us-central"));
        assert_eq!(app.serving_status.as_deref(), Some("SERVING"));
        assert_eq!(app.ssl_policy.as_deref(), Some("DEFAULT"));
    }

    #[tokio::test]
    async fn test_list_services() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/apps/my-project/services?pageSize=100")
            .returning_json(json!({
                "services": [
                    {
                        "name": "apps/my-project/services/default",
                        "id": "default",
                        "networkSettings": {
                            "ingressTrafficAllowed": "INGRESS_TRAFFIC_ALLOWED_ALL"
                        }
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let ae = client.appengine();
        let result = ae.list_services("my-project").await;
        assert!(result.is_ok());
        let services = result.unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "apps/my-project/services/default");
        assert_eq!(services[0].id.as_deref(), Some("default"));
        let ns = services[0].network_settings.as_ref().unwrap();
        assert_eq!(
            ns.ingress_traffic_allowed.as_deref(),
            Some("INGRESS_TRAFFIC_ALLOWED_ALL")
        );
    }

    #[tokio::test]
    async fn test_list_services_paginated() {
        let mut mock = crate::MockClient::new();
        // Second page first — more specific URL must come before the less specific first-page prefix
        // to avoid the StartsWith matcher returning the first-page response for both iterations.
        mock.expect_get("/v1/apps/my-project/services?pageSize=100&pageToken=token456")
            .returning_json(json!({
                "services": [{"name": "apps/my-project/services/api", "id": "api"}]
            }))
            .times(1);
        // First page
        mock.expect_get("/v1/apps/my-project/services?pageSize=100")
            .returning_json(json!({
                "services": [{"name": "apps/my-project/services/default", "id": "default"}],
                "nextPageToken": "token456"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let ae = client.appengine();
        let result = ae.list_services("my-project").await;
        assert!(result.is_ok());
        let services = result.unwrap();
        assert_eq!(services.len(), 2);
        assert_eq!(services[0].id.as_deref(), Some("default"));
        assert_eq!(services[1].id.as_deref(), Some("api"));
    }

    #[tokio::test]
    async fn test_get_service() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/apps/my-project/services/default")
            .returning_json(json!({
                "name": "apps/my-project/services/default",
                "id": "default",
                "networkSettings": {
                    "ingressTrafficAllowed": "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY"
                },
                "labels": {"env": "prod"}
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let ae = client.appengine();
        let result = ae.get_service("my-project", "default").await;
        assert!(result.is_ok());
        let svc = result.unwrap();
        assert_eq!(svc.name, "apps/my-project/services/default");
        assert_eq!(svc.id.as_deref(), Some("default"));
        let ns = svc.network_settings.as_ref().unwrap();
        assert_eq!(
            ns.ingress_traffic_allowed.as_deref(),
            Some("INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY")
        );
        assert_eq!(svc.labels.get("env").map(|s| s.as_str()), Some("prod"));
    }
}
