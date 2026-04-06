#[cfg(test)]
mod mock_helpers_demo {
    use crate::test_support::ComputeMockHelpers;
    use crate::{GcpHttpClient, MockClient};

    #[tokio::test]
    async fn test_compute_helpers_ergonomics() {
        let mut mock = MockClient::new();

        // Using helper methods - much more ergonomic than manually constructing paths!
        // Instead of: mock.expect_post("/compute/v1/projects/test-project/zones/us-central1-a/disks")
        mock.expect_create_disk("test-project", "us-central1-a")
            .returning_json(serde_json::json!({"name": "op-123", "status": "PENDING", "selfLink": "https://compute.googleapis.com/compute/v1/projects/test-project/operations/op-123"}));

        mock.expect_get("/compute/v1/projects/test-project/zones/us-central1-a/disks")
            .returning_json(serde_json::json!({"items": []}));

        let client = GcpHttpClient::from_mock(mock);
        let compute = client.compute();

        // Create disk
        let disk = crate::types::compute::Disk::fixture();
        let result = compute
            .create_disk_start("test-project", "us-central1-a", &disk)
            .await;
        assert!(result.is_ok());

        // List disks
        let result = compute.list_disks("test-project", "us-central1-a").await;
        assert!(result.is_ok());
    }
}
