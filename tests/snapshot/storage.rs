use gcp_lite::types::storage::*;
use insta::assert_json_snapshot;

#[test]
fn snapshot_bucket_fixture() {
    assert_json_snapshot!("bucket_fixture", Bucket::fixture());
}

#[test]
fn snapshot_object_fixture() {
    assert_json_snapshot!("object_fixture", Object::fixture());
}

#[test]
fn snapshot_buckets_fixture() {
    assert_json_snapshot!("buckets_fixture", Buckets::fixture());
}

#[test]
fn snapshot_objects_fixture() {
    assert_json_snapshot!("objects_fixture", Objects::fixture());
}

#[test]
fn snapshot_policy_fixture() {
    assert_json_snapshot!("policy_fixture", Policy::fixture());
}

#[test]
fn snapshot_compose_request_fixture() {
    assert_json_snapshot!("compose_request_fixture", ComposeRequest::fixture());
}

#[test]
fn snapshot_rewrite_response_fixture() {
    assert_json_snapshot!("rewrite_response_fixture", RewriteResponse::fixture());
}

#[test]
fn snapshot_bucket_minimal() {
    let bucket = Bucket {
        name: "my-bucket".to_string(),
        ..Default::default()
    };
    assert_json_snapshot!("bucket_minimal", bucket);
}

#[test]
fn snapshot_bucket_with_iam_config() {
    let bucket = Bucket {
        name: "iam-bucket".to_string(),
        iam_configuration: Some(BucketIamConfiguration {
            public_access_prevention: Some("enforced".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_json_snapshot!("bucket_with_iam_config", bucket);
}

#[test]
fn snapshot_object_minimal() {
    let object = Object {
        name: "file.txt".to_string(),
        ..Default::default()
    };
    assert_json_snapshot!("object_minimal", object);
}

#[test]
fn snapshot_compose_request_with_sources() {
    let req = ComposeRequest {
        source_objects: vec![
            serde_json::json!({"name": "part1.txt"}),
            serde_json::json!({"name": "part2.txt"}),
        ],
        destination: Some(Object {
            name: "combined.txt".to_string(),
            content_type: Some("text/plain".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_json_snapshot!("compose_request_with_sources", req);
}

#[test]
fn snapshot_rewrite_response_in_progress() {
    let resp = RewriteResponse {
        done: Some(false),
        rewrite_token: Some("token-abc".to_string()),
        total_bytes_rewritten: Some("500".to_string()),
        object_size: Some("1000".to_string()),
        ..Default::default()
    };
    assert_json_snapshot!("rewrite_response_in_progress", resp);
}

#[test]
fn snapshot_rewrite_response_done() {
    let resp = RewriteResponse {
        done: Some(true),
        resource: Some(Object {
            name: "dest.bin".to_string(),
            bucket: Some("dst-bucket".to_string()),
            size: Some("1000".to_string()),
            ..Default::default()
        }),
        total_bytes_rewritten: Some("1000".to_string()),
        object_size: Some("1000".to_string()),
        ..Default::default()
    };
    assert_json_snapshot!("rewrite_response_done", resp);
}
