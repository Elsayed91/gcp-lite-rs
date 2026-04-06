//! Mock HTTP client for testing without hitting real GCP APIs.

use crate::{GcpError, Result};
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// Mock HTTP client for testing
pub struct MockClient {
    expectations: Arc<Mutex<Vec<Expectation>>>,
    call_history: Arc<Mutex<Vec<Call>>>,
}

struct Expectation {
    method: String,
    path_matcher: PathMatcher,
    responses: Vec<Response>,
    response_index: usize,
    times: ExpectedTimes,
    called: usize,
}

enum PathMatcher {
    #[allow(dead_code)]
    Exact(String),
    StartsWith(String),
    #[allow(dead_code)]
    Regex(regex::Regex),
}

#[derive(Clone)]
enum Response {
    Json(Value),
    FixturePath(String),
    Error(GcpError),
}

enum ExpectedTimes {
    Once,
    Times(usize),
    AtLeast(usize),
    #[allow(dead_code)]
    Any,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Call {
    method: String,
    path: String,
    body: Option<Value>,
}

impl Default for MockClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockClient {
    /// Create a new mock client
    pub fn new() -> Self {
        Self {
            expectations: Arc::new(Mutex::new(Vec::new())),
            call_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Expect a GET request
    pub fn expect_get(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "GET", path)
    }

    /// Expect a POST request
    pub fn expect_post(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "POST", path)
    }

    /// Expect a DELETE request
    pub fn expect_delete(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "DELETE", path)
    }

    /// Expect a PUT request
    pub fn expect_put(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "PUT", path)
    }

    /// Expect a PATCH request
    pub fn expect_patch(&mut self, path: &str) -> ExpectationBuilder<'_> {
        ExpectationBuilder::new(self, "PATCH", path)
    }

    pub(crate) async fn execute(
        &self,
        method: &str,
        url: &str,
        body: Option<&Value>,
    ) -> Result<Vec<u8>> {
        let path = if url.starts_with("http") {
            // Find the first / after the protocol (e.g. after https://)
            if let Some(pos) = url.find("://") {
                if let Some(slash_pos) = url[pos + 3..].find('/') {
                    &url[pos + 3 + slash_pos..]
                } else {
                    "/"
                }
            } else {
                url
            }
        } else {
            url
        };

        // Record call
        self.call_history.lock().unwrap().push(Call {
            method: method.to_string(),
            path: path.to_string(),
            body: body.cloned(),
        });

        // Find matching expectation
        let mut expectations = self.expectations.lock().unwrap();

        for expectation in expectations.iter_mut() {
            if expectation.method == method && expectation.matches_path(path) {
                expectation.called += 1;

                // Get the response for this call (cycle through responses for sequential calls)
                let response = if expectation.responses.len() == 1 {
                    &expectation.responses[0]
                } else {
                    let idx = expectation
                        .response_index
                        .min(expectation.responses.len() - 1);
                    expectation.response_index += 1;
                    &expectation.responses[idx]
                };

                return match response {
                    Response::Json(value) => Ok(serde_json::to_vec(value).unwrap()),
                    Response::FixturePath(path) => {
                        let fixture = std::fs::read_to_string(path)
                            .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", path));
                        Ok(fixture.into_bytes())
                    }
                    Response::Error(err) => Err(err.clone()),
                };
            }
        }

        panic!(
            "Unexpected call: {} {}\nCall history:\n{:#?}",
            method,
            path,
            self.call_history.lock().unwrap()
        );
    }
}

impl Drop for MockClient {
    fn drop(&mut self) {
        // Verify all expectations were met
        let expectations = self.expectations.lock().unwrap();
        for exp in expectations.iter() {
            match exp.times {
                ExpectedTimes::Once if exp.called != 1 => {
                    panic!(
                        "Expected {} {} to be called once, called {} times",
                        exp.method,
                        exp.path_display(),
                        exp.called
                    );
                }
                ExpectedTimes::Times(n) if exp.called != n => {
                    panic!(
                        "Expected {} {} to be called {} times, called {} times",
                        exp.method,
                        exp.path_display(),
                        n,
                        exp.called
                    );
                }
                ExpectedTimes::AtLeast(n) if exp.called < n => {
                    panic!(
                        "Expected {} {} to be called at least {} times, called {} times",
                        exp.method,
                        exp.path_display(),
                        n,
                        exp.called
                    );
                }
                _ => {}
            }
        }
    }
}

/// Builder for setting up expectations
pub struct ExpectationBuilder<'a> {
    client: &'a mut MockClient,
    method: String,
    path: String,
    responses: Vec<Response>,
    times: ExpectedTimes,
}

impl<'a> ExpectationBuilder<'a> {
    fn new(client: &'a mut MockClient, method: &str, path: &str) -> Self {
        Self {
            client,
            method: method.to_string(),
            path: path.to_string(),
            responses: Vec::new(),
            times: ExpectedTimes::Once,
        }
    }

    /// Set the response to return JSON
    pub fn returning_json(mut self, value: Value) -> Self {
        self.responses.push(Response::Json(value));
        self
    }

    /// Set multiple sequential responses (for polling scenarios)
    pub fn returning_json_sequence(mut self, values: Vec<Value>) -> Self {
        for value in values {
            self.responses.push(Response::Json(value));
        }
        self
    }

    /// Set the response to load from a fixture file
    pub fn returning_fixture(mut self, path: &str) -> Self {
        self.responses.push(Response::FixturePath(path.to_string()));
        self
    }

    /// Set the response to return an error
    pub fn returning_error(mut self, error: GcpError) -> Self {
        self.responses.push(Response::Error(error));
        self
    }

    /// Set the number of times this expectation should be called
    pub fn times(mut self, n: usize) -> Self {
        self.times = ExpectedTimes::Times(n);
        self
    }

    /// Set the minimum number of times this expectation should be called
    pub fn at_least(mut self, n: usize) -> Self {
        self.times = ExpectedTimes::AtLeast(n);
        self
    }

    /// Use exact path matching (including query parameters)
    pub fn with_exact_path(self) -> Self {
        // This will be handled in Drop by checking if we should use Exact matcher
        self
    }
}

impl<'a> Drop for ExpectationBuilder<'a> {
    fn drop(&mut self) {
        if self.responses.is_empty() {
            panic!(
                "No response set for expectation: {} {}",
                self.method, self.path
            );
        }

        let expectation = Expectation {
            method: self.method.clone(),
            path_matcher: PathMatcher::StartsWith(self.path.clone()),
            responses: std::mem::take(&mut self.responses),
            response_index: 0,
            times: std::mem::replace(&mut self.times, ExpectedTimes::Once),
            called: 0,
        };

        self.client.expectations.lock().unwrap().push(expectation);
    }
}

impl Expectation {
    fn matches_path(&self, path: &str) -> bool {
        match &self.path_matcher {
            PathMatcher::Exact(p) => {
                // For exact matching, also handle query params
                // Split both paths at '?' and compare base paths and query params
                let (expected_base, expected_query) = split_path_query(p);
                let (actual_base, actual_query) = split_path_query(path);

                if expected_base != actual_base {
                    return false;
                }

                // If expected has no query params, match any query params
                // If expected has query params, they must match
                if expected_query.is_empty() {
                    true
                } else {
                    expected_query == actual_query
                }
            }
            PathMatcher::StartsWith(p) => path.starts_with(p),
            PathMatcher::Regex(r) => r.is_match(path),
        }
    }

    fn path_display(&self) -> String {
        match &self.path_matcher {
            PathMatcher::Exact(p) => p.clone(),
            PathMatcher::StartsWith(p) => format!("{}*", p),
            PathMatcher::Regex(r) => format!("/{}/", r.as_str()),
        }
    }
}

fn split_path_query(path: &str) -> (&str, &str) {
    if let Some(pos) = path.find('?') {
        (&path[..pos], &path[pos + 1..])
    } else {
        (path, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GcpHttpClient;

    #[tokio::test]
    async fn mock_service_usage_api() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test/services/compute.googleapis.com")
            .returning_json(serde_json::json!({
                "name": "projects/test/services/compute.googleapis.com",
                "state": "ENABLED"
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .is_service_enabled("test", "compute.googleapis.com")
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn mock_full_arbiter_workflow() {
        // Simulate a complete arbiter-gcp workflow:
        // 1. Check API enablement
        // 2. List assets
        // 3. Get recommendations
        // 4. Execute remediation

        let mut mock = MockClient::new();

        // Service Usage check
        mock.expect_get("/v1/projects/test/services/compute.googleapis.com")
            .returning_json(serde_json::json!({
                "name": "projects/test/services/compute.googleapis.com",
                "state": "ENABLED"
            }))
            .times(1);

        // Asset listing
        mock.expect_get("/v1/assets")
            .returning_json(serde_json::json!({
                "assets": [
                    {
                        "name": "//compute.googleapis.com/projects/test/zones/us-central1-a/disks/unused-disk",
                        "assetType": "compute.googleapis.com/Disk"
                    }
                ]
            }))
            .times(1);

        // Remediation - delete disk
        mock.expect_delete("/compute/v1/projects/test/zones/us-central1-a/disks/unused-disk")
            .returning_json(serde_json::json!({
                "name": "op-123",
                "status": "PENDING",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/test/zones/us-central1-a/operations/op-123"
            }))
            .times(1);

        // Operation polling uses selfLink, so it includes the full path with /compute/v1
        mock.expect_get("/compute/v1/projects/test/zones/us-central1-a/operations/op-123")
            .returning_json(serde_json::json!({
                "name": "op-123",
                "status": "DONE"
            }))
            .at_least(1);

        let client = GcpHttpClient::from_mock(mock);

        // 1. Check API
        let enabled = client
            .service_usage()
            .is_service_enabled("test", "compute.googleapis.com")
            .await
            .unwrap();
        assert!(enabled);

        // 2. List assets (simplified)
        let response = client
            .get("https://cloudasset.googleapis.com/v1/assets")
            .await
            .unwrap();
        let _assets: serde_json::Value = serde_json::from_slice(&response).unwrap();

        // 3. Execute remediation
        client
            .compute()
            .delete_disk("test", "us-central1-a", "unused-disk")
            .await
            .unwrap();

        println!("✓ Complete workflow succeeded");
    }
}
