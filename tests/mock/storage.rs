use gcp_lite::types::storage::*;
use gcp_lite::{GcpHttpClient, MockClient};

fn bucket_json(name: &str) -> serde_json::Value {
    serde_json::to_value(Bucket {
        name: name.to_string(),
        kind: Some("storage#bucket".to_string()),
        ..Default::default()
    })
    .unwrap()
}

#[tokio::test]
async fn test_bucket_crud_lifecycle() {
    let mut mock = MockClient::new();
    let project = "test-project";
    let bucket = "lifecycle-bucket";

    // NOTE: Register expectations with more specific paths FIRST because
    // MockClient uses StartsWith matching and checks expectations in order.
    // Register GET with prefix before POST (which has shorter path)

    mock.expect_get("/storage/v1/b?project=test-project&prefix=lifecycle-")
        .returning_json(
            serde_json::to_value(Buckets {
                items: vec![Bucket {
                    name: bucket.to_string(),
                    location: Some("US".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_get("/storage/v1/b/lifecycle-bucket")
        .returning_json(
            serde_json::to_value(Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_patch("/storage/v1/b/lifecycle-bucket")
        .returning_json(
            serde_json::to_value(Bucket {
                name: bucket.to_string(),
                storage_class: Some("NEARLINE".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_delete("/storage/v1/b/lifecycle-bucket")
        .returning_json(serde_json::json!({}));

    mock.expect_post("/storage/v1/b?project=test-project")
        .returning_json(bucket_json(bucket));

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let created = storage
        .create_bucket(
            project,
            &Bucket {
                name: bucket.to_string(),
                location: Some("US".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(created.name, bucket);

    let listed = storage
        .list_buckets(project, Some("lifecycle-"), None)
        .await
        .unwrap();
    assert_eq!(listed.items.len(), 1);
    assert_eq!(listed.items[0].name, bucket);

    let got = storage.get_bucket(bucket).await.unwrap();
    assert_eq!(got.name, bucket);
    assert_eq!(got.location.as_deref(), Some("US"));

    let patched = storage
        .patch_bucket(
            bucket,
            &Bucket {
                storage_class: Some("NEARLINE".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(patched.storage_class.as_deref(), Some("NEARLINE"));

    storage.delete_bucket(bucket).await.unwrap();
}

#[tokio::test]
async fn test_object_crud_lifecycle() {
    let mut mock = MockClient::new();
    let bucket = "obj-bucket";
    let obj_name = "data/file.json";

    // NOTE: Register expectations in order of specificity (most specific first)
    // Also note: create_object uses the upload endpoint, not the regular storage endpoint
    // And empty query params are filtered out

    mock.expect_delete("/storage/v1/b/obj-bucket/o/data%2Ffile.json")
        .returning_json(serde_json::json!({}));

    mock.expect_get("/storage/v1/b/obj-bucket/o?prefix=data%2F")
        .returning_json(
            serde_json::to_value(Objects {
                items: vec![Object {
                    name: obj_name.to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_get("/storage/v1/b/obj-bucket/o/data%2Ffile.json")
        .returning_json(
            serde_json::to_value(Object {
                name: obj_name.to_string(),
                bucket: Some(bucket.to_string()),
                size: Some("256".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    // create_object uses upload endpoint with ?name= query param
    mock.expect_post("/upload/storage/v1/b/obj-bucket/o?name=data%2Ffile.json")
        .returning_json(
            serde_json::to_value(Object {
                name: obj_name.to_string(),
                bucket: Some(bucket.to_string()),
                size: Some("256".to_string()),
                content_type: Some("application/json".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let created = storage
        .create_object(
            bucket,
            &Object {
                name: obj_name.to_string(),
                content_type: Some("application/json".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(created.name, obj_name);
    assert_eq!(created.size.as_deref(), Some("256"));

    let got = storage.get_object(bucket, obj_name).await.unwrap();
    assert_eq!(got.name, obj_name);

    let listed = storage
        .list_objects(bucket, Some("data/"), None)
        .await
        .unwrap();
    assert_eq!(listed.items.len(), 1);

    storage.delete_object(bucket, obj_name).await.unwrap();
}

#[tokio::test]
async fn test_compose_workflow() {
    let mut mock = MockClient::new();
    let bucket = "compose-bucket";

    mock.expect_post("/storage/v1/b/compose-bucket/o/merged.txt/compose")
        .returning_json(
            serde_json::to_value(Object {
                name: "merged.txt".to_string(),
                bucket: Some(bucket.to_string()),
                component_count: Some(2),
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_get("/storage/v1/b/compose-bucket/o/merged.txt")
        .returning_json(
            serde_json::to_value(Object {
                name: "merged.txt".to_string(),
                bucket: Some(bucket.to_string()),
                component_count: Some(2),
                size: Some("512".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let composed = storage
        .compose_object(
            bucket,
            "merged.txt",
            &ComposeRequest {
                source_objects: vec![
                    serde_json::json!({"name": "part1.txt"}),
                    serde_json::json!({"name": "part2.txt"}),
                ],
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(composed.name, "merged.txt");
    assert_eq!(composed.component_count, Some(2));

    let verified = storage.get_object(bucket, "merged.txt").await.unwrap();
    assert_eq!(verified.size.as_deref(), Some("512"));
}

#[tokio::test]
async fn test_copy_across_buckets() {
    let mut mock = MockClient::new();

    mock.expect_post("/storage/v1/b/src-bucket/o/original.bin/copyTo/b/dst-bucket/o/backup.bin")
        .returning_json(
            serde_json::to_value(Object {
                name: "backup.bin".to_string(),
                bucket: Some("dst-bucket".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    mock.expect_get("/storage/v1/b/dst-bucket/o/backup.bin")
        .returning_json(
            serde_json::to_value(Object {
                name: "backup.bin".to_string(),
                bucket: Some("dst-bucket".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let copied = storage
        .copy_object(
            "src-bucket",
            "original.bin",
            "dst-bucket",
            "backup.bin",
            &Object::default(),
        )
        .await
        .unwrap();
    assert_eq!(copied.name, "backup.bin");
    assert_eq!(copied.bucket.as_deref(), Some("dst-bucket"));

    let verified = storage
        .get_object("dst-bucket", "backup.bin")
        .await
        .unwrap();
    assert_eq!(verified.name, "backup.bin");
}

#[tokio::test]
async fn test_rewrite_with_resume() {
    let mut mock = MockClient::new();

    // NOTE: Register expectations in REVERSE order (most specific first) because
    // MockClient uses StartsWith matching and checks expectations in order.
    // The path without query params would match all paths otherwise.

    // Third call: rewriteToken=tok-2, returns done
    mock.expect_post(
        "/storage/v1/b/src/o/large.zip/rewriteTo/b/dst/o/large.zip?rewriteToken=tok-2",
    )
    .returning_json(
        serde_json::to_value(RewriteResponse {
            done: Some(true),
            resource: Some(Object {
                name: "large.zip".to_string(),
                bucket: Some("dst".to_string()),
                size: Some("3000".to_string()),
                ..Default::default()
            }),
            total_bytes_rewritten: Some("3000".to_string()),
            object_size: Some("3000".to_string()),
            ..Default::default()
        })
        .unwrap(),
    )
    .times(1);

    // Second call: rewriteToken=tok-1
    mock.expect_post(
        "/storage/v1/b/src/o/large.zip/rewriteTo/b/dst/o/large.zip?rewriteToken=tok-1",
    )
    .returning_json(
        serde_json::to_value(RewriteResponse {
            done: Some(false),
            rewrite_token: Some("tok-2".to_string()),
            total_bytes_rewritten: Some("2000".to_string()),
            object_size: Some("3000".to_string()),
            ..Default::default()
        })
        .unwrap(),
    )
    .times(1);

    // First call: no rewriteToken (token is None, so empty string is filtered out)
    // Registered LAST so it doesn't match the more specific paths above
    mock.expect_post("/storage/v1/b/src/o/large.zip/rewriteTo/b/dst/o/large.zip")
        .returning_json(
            serde_json::to_value(RewriteResponse {
                done: Some(false),
                rewrite_token: Some("tok-1".to_string()),
                total_bytes_rewritten: Some("1000".to_string()),
                object_size: Some("3000".to_string()),
                ..Default::default()
            })
            .unwrap(),
        )
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let mut token: Option<String> = None;
    let mut done = false;
    let mut iterations = 0;

    while !done {
        let resp = storage
            .rewrite_object(
                "src",
                "large.zip",
                "dst",
                "large.zip",
                token.as_deref(),
                &Object::default(),
            )
            .await
            .unwrap();

        iterations += 1;
        done = resp.done == Some(true);
        token = resp.rewrite_token.clone();

        if done {
            let resource = resp.resource.unwrap();
            assert_eq!(resource.name, "large.zip");
            assert_eq!(resource.size.as_deref(), Some("3000"));
        }
    }

    assert_eq!(iterations, 3);
}

#[tokio::test]
async fn test_bucket_iam_workflow() {
    let mut mock = MockClient::new();
    let bucket = "iam-test-bucket";

    let initial_policy = Policy {
        kind: Some("storage#policy".to_string()),
        version: Some(3),
        ..Default::default()
    };

    let updated_policy = Policy {
        kind: Some("storage#policy".to_string()),
        version: Some(3),
        bindings: vec![PolicyBinding {
            role: Some("roles/storage.objectViewer".to_string()),
            members: Some(vec!["allUsers".to_string()]),
            ..Default::default()
        }],
        ..Default::default()
    };

    mock.expect_get("/storage/v1/b/iam-test-bucket/iam")
        .returning_json(serde_json::to_value(&initial_policy).unwrap());

    mock.expect_put("/storage/v1/b/iam-test-bucket/iam")
        .returning_json(serde_json::to_value(&updated_policy).unwrap());

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let policy = storage.get_bucket_iam_policy(bucket).await.unwrap();
    assert!(policy.bindings.is_empty());
    assert_eq!(policy.version, Some(3));

    let mut new_policy = policy;
    new_policy.bindings.push(PolicyBinding {
        role: Some("roles/storage.objectViewer".to_string()),
        members: Some(vec!["allUsers".to_string()]),
        ..Default::default()
    });

    let result = storage
        .set_bucket_iam_policy(bucket, &new_policy)
        .await
        .unwrap();
    assert_eq!(result.bindings.len(), 1);
}

#[tokio::test]
async fn test_public_access_prevention_lifecycle() {
    let mut mock = MockClient::new();
    let bucket = "pap-lifecycle";

    // No GET needed — set_public_access_prevention sends a minimal PATCH body
    // with only iamConfiguration, avoiding read-modify-write race conditions.
    mock.expect_patch("/storage/v1/b/pap-lifecycle")
        .returning_json_sequence(vec![
            serde_json::to_value(Bucket {
                name: bucket.to_string(),
                iam_configuration: Some(BucketIamConfiguration {
                    public_access_prevention: Some("enforced".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .unwrap(),
            serde_json::to_value(Bucket {
                name: bucket.to_string(),
                iam_configuration: Some(BucketIamConfiguration {
                    public_access_prevention: Some("inherited".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .unwrap(),
        ])
        .times(2);

    let client = GcpHttpClient::from_mock(mock);
    let storage = client.storage();

    let enforced = storage
        .set_public_access_prevention(bucket, true)
        .await
        .unwrap();
    assert_eq!(
        enforced
            .iam_configuration
            .unwrap()
            .public_access_prevention
            .as_deref(),
        Some("enforced")
    );

    let inherited = storage
        .set_public_access_prevention(bucket, false)
        .await
        .unwrap();
    assert_eq!(
        inherited
            .iam_configuration
            .unwrap()
            .public_access_prevention
            .as_deref(),
        Some("inherited")
    );
}
