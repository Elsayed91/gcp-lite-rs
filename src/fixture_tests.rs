#[cfg(test)]
mod fixture_tests {
    use crate::types::compute::*;
    use crate::types::service_usage::*;

    #[test]
    fn test_service_usage_service_state_fixture() {
        let service = ServiceState::fixture();
        assert!(!service.name.is_empty());
        // Fixture uses the discovery schema name, not the rust name
        assert_eq!(service.name, "test-google_api_serviceusage_v1_service");
    }

    #[test]
    fn test_service_usage_lro_fixture() {
        let op = ServiceUsageLro::fixture();
        assert_eq!(op.name, "operation-done");
        assert!(op.done);
    }

    #[test]
    fn test_service_usage_lro_pending_fixture() {
        let op = ServiceUsageLro::fixture_pending();
        assert_eq!(op.name, "operation-pending");
        assert!(!op.done);
    }

    #[test]
    fn test_compute_disk_fixture() {
        let disk = Disk::fixture();
        assert!(!disk.name.is_empty());
    }

    #[test]
    fn test_compute_operation_fixture() {
        let op = OperationResponse::fixture();
        assert!(!op.name.is_empty());
        assert_eq!(op.status, Some("DONE".into()));
    }
}
