//! Cloud Storage JSON API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::storage::StorageOps`. This layer adds:
//! - Ergonomic method signatures
//! - Convenience methods (e.g. set_public_access_prevention)

use crate::{
    GcpHttpClient, Result,
    ops::storage::StorageOps,
    types::storage::{
        Bucket, BucketIamConfiguration, Buckets, ComposeRequest, Object, Objects, Policy,
        RewriteResponse,
    },
};

/// Client for the Cloud Storage JSON API
pub struct StorageClient<'a> {
    ops: StorageOps<'a>,
}

impl<'a> StorageClient<'a> {
    /// Create a new Cloud Storage JSON API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: StorageOps::new(client),
        }
    }

    // ── Buckets ───────────────────────────────────────────────────────────

    /// Returns metadata for the specified bucket.
    pub async fn get_bucket(&self, bucket: &str) -> Result<Bucket> {
        self.ops.get_bucket(bucket).await
    }

    /// Retrieves a list of buckets for a given project.
    ///
    /// `prefix` filters results to buckets whose names begin with this prefix.
    /// `page_token` specifies a page token to use (from previous response).
    pub async fn list_buckets(
        &self,
        project: &str,
        prefix: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<Buckets> {
        self.ops
            .list_buckets(project, prefix.unwrap_or(""), page_token.unwrap_or(""))
            .await
    }

    /// Stream all buckets in a project, automatically handling pagination.
    pub fn list_buckets_stream(
        &self,
        project: &str,
        prefix: Option<&str>,
    ) -> impl futures::Stream<Item = Result<Bucket>> + '_ {
        let project = project.to_string();
        let prefix = prefix.unwrap_or("").to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_buckets(&project, &prefix, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Creates a new bucket.
    pub async fn create_bucket(&self, project: &str, body: &Bucket) -> Result<Bucket> {
        self.ops.create_bucket(project, body).await
    }

    /// Patches a bucket.
    ///
    /// The bucket `name` field in the body is automatically set to match the
    /// `bucket` parameter to avoid conflicts with the URL path.
    pub async fn patch_bucket(&self, bucket: &str, body: &Bucket) -> Result<Bucket> {
        let mut body = body.clone();
        body.name = bucket.to_string();
        self.ops.patch_bucket(bucket, &body).await
    }

    /// Deletes an empty bucket.
    pub async fn delete_bucket(&self, bucket: &str) -> Result<()> {
        self.ops.delete_bucket(bucket).await
    }

    /// Returns an IAM policy for the specified bucket.
    pub async fn get_bucket_iam_policy(&self, bucket: &str) -> Result<Policy> {
        self.ops.get_bucket_iam_policy(bucket).await
    }

    /// Updates an IAM policy for the specified bucket.
    pub async fn set_bucket_iam_policy(&self, bucket: &str, body: &Policy) -> Result<Policy> {
        self.ops.set_bucket_iam_policy(bucket, body).await
    }

    /// Sets the public access prevention status for a bucket.
    ///
    /// # Arguments
    /// * `bucket` - The bucket name.
    /// * `enforced` - If true, sets "enforced"; if false, sets "inherited".
    pub async fn set_public_access_prevention(
        &self,
        bucket: &str,
        enforced: bool,
    ) -> Result<Bucket> {
        let pap = if enforced { "enforced" } else { "inherited" };

        // Send a minimal patch body with only the field we want to update.
        // PATCH semantics mean unset fields are left unchanged, preventing
        // lost updates from concurrent modifications to other bucket fields.
        let patch = Bucket {
            iam_configuration: Some(BucketIamConfiguration {
                public_access_prevention: Some(pap.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        self.patch_bucket(bucket, &patch).await
    }

    // ── Bucket Retention Lock ─────────────────────────────────────────────

    /// Locks the retention policy on a bucket.
    ///
    /// **This operation is permanent and irreversible.** Once a retention policy is
    /// locked, it cannot be removed or shortened — it can only be lengthened.
    /// The bucket can still be deleted as long as it is empty.
    ///
    /// The `if_metageneration_match` parameter must equal the bucket's current
    /// metageneration (from `get_bucket().metageneration`). This provides optimistic
    /// concurrency control.
    ///
    /// Used to satisfy CIS 2.3: ensure bucket retention policies are locked
    /// (GCS retention lock prevents log tampering).
    pub async fn lock_bucket_retention_policy(
        &self,
        bucket: &str,
        if_metageneration_match: &str,
    ) -> Result<Bucket> {
        self.ops
            .lock_bucket_retention_policy(bucket, if_metageneration_match)
            .await
    }

    // ── Objects ───────────────────────────────────────────────────────────

    /// Retrieves an object or its metadata.
    pub async fn get_object(&self, bucket: &str, object: &str) -> Result<Object> {
        self.ops.get_object(bucket, object, "").await
    }

    /// Retrieves an object generation.
    pub async fn get_object_generation(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> Result<Object> {
        self.ops.get_object(bucket, object, generation).await
    }

    /// Retrieves a list of objects matching the criteria.
    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<Objects> {
        self.ops
            .list_objects(
                bucket,
                prefix.unwrap_or(""),
                "",
                page_token.unwrap_or(""),
                "",
            )
            .await
    }

    /// Retrieves a list of objects with full options.
    pub async fn list_objects_with_opts(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        delimiter: Option<&str>,
        page_token: Option<&str>,
        versions: bool,
    ) -> Result<Objects> {
        let v_str = if versions { "true" } else { "false" };
        self.ops
            .list_objects(
                bucket,
                prefix.unwrap_or(""),
                delimiter.unwrap_or(""),
                page_token.unwrap_or(""),
                v_str,
            )
            .await
    }

    /// Stream all objects in a bucket, automatically handling pagination.
    pub fn list_objects_stream(
        &self,
        bucket: &str,
        prefix: Option<&str>,
    ) -> impl futures::Stream<Item = Result<Object>> + '_ {
        let bucket = bucket.to_string();
        let prefix = prefix.unwrap_or("").to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_objects(&bucket, &prefix, "", page_token.as_deref().unwrap_or(""), "")
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Stores a new object and metadata.
    pub async fn create_object(&self, bucket: &str, body: &Object) -> Result<Object> {
        self.ops.create_object(bucket, &body.name, body).await
    }

    /// Deletes an object and its metadata.
    pub async fn delete_object(&self, bucket: &str, object: &str) -> Result<()> {
        self.ops.delete_object(bucket, object, "").await
    }

    /// Deletes a specific generation of an object.
    pub async fn delete_object_generation(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> Result<()> {
        self.ops.delete_object(bucket, object, generation).await
    }

    /// Concatenates a list of existing objects into a new object in the same bucket.
    pub async fn compose_object(
        &self,
        destination_bucket: &str,
        destination_object: &str,
        body: &ComposeRequest,
    ) -> Result<Object> {
        self.ops
            .compose_object(destination_bucket, destination_object, body)
            .await
    }

    /// Copies a source object to a destination object.
    pub async fn copy_object(
        &self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        body: &Object,
    ) -> Result<Object> {
        let mut body = body.clone();
        if body.name.is_empty() {
            body.name = destination_object.to_string();
        }
        if body.bucket.as_deref().unwrap_or("").is_empty() {
            body.bucket = Some(destination_bucket.to_string());
        }
        self.ops
            .copy_object(
                source_bucket,
                source_object,
                destination_bucket,
                destination_object,
                &body,
            )
            .await
    }

    /// Rewrites a source object to a destination object.
    pub async fn rewrite_object(
        &self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        rewrite_token: Option<&str>,
        body: &Object,
    ) -> Result<RewriteResponse> {
        let mut body = body.clone();
        if body.name.is_empty() {
            body.name = destination_object.to_string();
        }
        if body.bucket.as_deref().unwrap_or("").is_empty() {
            body.bucket = Some(destination_bucket.to_string());
        }
        self.ops
            .rewrite_object(
                source_bucket,
                source_object,
                destination_bucket,
                destination_object,
                rewrite_token.unwrap_or(""),
                &body,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    fn bucket_json(name: &str) -> serde_json::Value {
        serde_json::to_value(Bucket {
            name: name.to_string(),
            kind: Some("storage#bucket".to_string()),
            ..Default::default()
        })
        .unwrap()
    }

    fn object_json(bucket: &str, name: &str) -> serde_json::Value {
        serde_json::to_value(Object {
            name: name.to_string(),
            bucket: Some(bucket.to_string()),
            kind: Some("storage#object".to_string()),
            ..Default::default()
        })
        .unwrap()
    }

    #[tokio::test]
    async fn test_bucket_lifecycle() {
        let mut mock = MockClient::new();

        mock.expect_post("/storage/v1/b?project=test-project")
            .returning_json(bucket_json("test-bucket"));
        mock.expect_get("/storage/v1/b/test-bucket")
            .returning_json(bucket_json("test-bucket"));
        mock.expect_delete("/storage/v1/b/test-bucket")
            .returning_json(serde_json::json!({}));

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let b = storage
            .create_bucket("test-project", &Bucket::default())
            .await
            .unwrap();
        assert_eq!(b.name, "test-bucket");

        let b2 = storage.get_bucket("test-bucket").await.unwrap();
        assert_eq!(b2.name, "test-bucket");

        storage.delete_bucket("test-bucket").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_buckets() {
        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b?project=my-project&prefix=app-")
            .returning_json(
                serde_json::to_value(Buckets {
                    items: vec![
                        Bucket {
                            name: "app-data".to_string(),
                            ..Default::default()
                        },
                        Bucket {
                            name: "app-logs".to_string(),
                            ..Default::default()
                        },
                    ],
                    next_page_token: Some("token2".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let buckets = storage
            .list_buckets("my-project", Some("app-"), None)
            .await
            .unwrap();
        assert_eq!(buckets.items.len(), 2);
        assert_eq!(buckets.items[0].name, "app-data");
        assert_eq!(buckets.next_page_token.as_deref(), Some("token2"));
    }

    #[tokio::test]
    async fn test_list_buckets_pagination() {
        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b?project=proj&pageToken=tok1")
            .returning_json(
                serde_json::to_value(Buckets {
                    items: vec![Bucket {
                        name: "page2-bucket".to_string(),
                        ..Default::default()
                    }],
                    next_page_token: None,
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let buckets = storage
            .list_buckets("proj", None, Some("tok1"))
            .await
            .unwrap();
        assert_eq!(buckets.items.len(), 1);
        assert!(buckets.next_page_token.is_none());
    }

    #[tokio::test]
    async fn test_patch_bucket() {
        let mut mock = MockClient::new();

        mock.expect_patch("/storage/v1/b/my-bucket").returning_json(
            serde_json::to_value(Bucket {
                name: "my-bucket".to_string(),
                storage_class: Some("NEARLINE".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let patched = storage
            .patch_bucket(
                "my-bucket",
                &Bucket {
                    storage_class: Some("NEARLINE".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(patched.storage_class.as_deref(), Some("NEARLINE"));
    }

    #[tokio::test]
    async fn test_bucket_iam_policy() {
        let mut mock = MockClient::new();

        let policy = Policy {
            kind: Some("storage#policy".to_string()),
            version: Some(3),
            ..Default::default()
        };

        mock.expect_get("/storage/v1/b/iam-bucket/iam")
            .returning_json(serde_json::to_value(&policy).unwrap());
        mock.expect_put("/storage/v1/b/iam-bucket/iam")
            .returning_json(serde_json::to_value(&policy).unwrap());

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let got = storage.get_bucket_iam_policy("iam-bucket").await.unwrap();
        assert_eq!(got.version, Some(3));

        let set = storage
            .set_bucket_iam_policy("iam-bucket", &policy)
            .await
            .unwrap();
        assert_eq!(set.version, Some(3));
    }

    #[tokio::test]
    async fn test_set_public_access_prevention_enforced() {
        let mut mock = MockClient::new();

        // Only a PATCH is needed — no GET. The method sends a minimal patch body
        // with only iamConfiguration, avoiding read-modify-write race conditions.
        mock.expect_patch("/storage/v1/b/pap-bucket")
            .returning_json(
                serde_json::to_value(Bucket {
                    name: "pap-bucket".to_string(),
                    iam_configuration: Some(BucketIamConfiguration {
                        public_access_prevention: Some("enforced".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let b = storage
            .set_public_access_prevention("pap-bucket", true)
            .await
            .unwrap();
        assert_eq!(
            b.iam_configuration
                .unwrap()
                .public_access_prevention
                .as_deref(),
            Some("enforced")
        );
    }

    #[tokio::test]
    async fn test_set_public_access_prevention_inherited() {
        let mut mock = MockClient::new();

        mock.expect_patch("/storage/v1/b/pap-bucket")
            .returning_json(
                serde_json::to_value(Bucket {
                    name: "pap-bucket".to_string(),
                    iam_configuration: Some(BucketIamConfiguration {
                        public_access_prevention: Some("inherited".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let b = storage
            .set_public_access_prevention("pap-bucket", false)
            .await
            .unwrap();
        assert_eq!(
            b.iam_configuration
                .unwrap()
                .public_access_prevention
                .as_deref(),
            Some("inherited")
        );
    }

    #[tokio::test]
    async fn test_object_lifecycle() {
        let mut mock = MockClient::new();

        mock.expect_post("/upload/storage/v1/b/my-bucket/o?name=file.txt")
            .returning_json(object_json("my-bucket", "file.txt"));
        mock.expect_get("/storage/v1/b/my-bucket/o/file.txt")
            .returning_json(object_json("my-bucket", "file.txt"));
        mock.expect_delete("/storage/v1/b/my-bucket/o/file.txt")
            .returning_json(serde_json::json!({}));

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let obj = storage
            .create_object(
                "my-bucket",
                &Object {
                    name: "file.txt".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(obj.name, "file.txt");
        assert_eq!(obj.bucket.as_deref(), Some("my-bucket"));

        let got = storage.get_object("my-bucket", "file.txt").await.unwrap();
        assert_eq!(got.name, "file.txt");

        storage
            .delete_object("my-bucket", "file.txt")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_object_generation() {
        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b/my-bucket/o/file.txt?generation=12345")
            .returning_json(
                serde_json::to_value(Object {
                    name: "file.txt".to_string(),
                    generation: Some("12345".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let obj = storage
            .get_object_generation("my-bucket", "file.txt", "12345")
            .await
            .unwrap();
        assert_eq!(obj.generation.as_deref(), Some("12345"));
    }

    #[tokio::test]
    async fn test_delete_object_generation() {
        let mut mock = MockClient::new();

        mock.expect_delete("/storage/v1/b/my-bucket/o/file.txt?generation=99")
            .returning_json(serde_json::json!({}));

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        storage
            .delete_object_generation("my-bucket", "file.txt", "99")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_list_objects() {
        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b/my-bucket/o?prefix=logs%2F")
            .returning_json(
                serde_json::to_value(Objects {
                    items: vec![
                        Object {
                            name: "logs/a.log".to_string(),
                            ..Default::default()
                        },
                        Object {
                            name: "logs/b.log".to_string(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let objs = storage
            .list_objects("my-bucket", Some("logs/"), None)
            .await
            .unwrap();
        assert_eq!(objs.items.len(), 2);
        assert_eq!(objs.items[0].name, "logs/a.log");
    }

    #[tokio::test]
    async fn test_list_objects_with_opts() {
        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b/my-bucket/o?delimiter=%2F&versions=true")
            .returning_json(
                serde_json::to_value(Objects {
                    items: vec![],
                    prefixes: vec!["dir1/".to_string(), "dir2/".to_string()],
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let objs = storage
            .list_objects_with_opts("my-bucket", None, Some("/"), None, true)
            .await
            .unwrap();
        assert!(objs.items.is_empty());
        assert_eq!(objs.prefixes.len(), 2);
        assert_eq!(objs.prefixes[0], "dir1/");
    }

    #[tokio::test]
    async fn test_compose_object() {
        let mut mock = MockClient::new();

        mock.expect_post("/storage/v1/b/my-bucket/o/combined.txt/compose")
            .returning_json(
                serde_json::to_value(Object {
                    name: "combined.txt".to_string(),
                    bucket: Some("my-bucket".to_string()),
                    component_count: Some(2),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let result = storage
            .compose_object(
                "my-bucket",
                "combined.txt",
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
        assert_eq!(result.name, "combined.txt");
        assert_eq!(result.component_count, Some(2));
    }

    #[tokio::test]
    async fn test_copy_object() {
        let mut mock = MockClient::new();

        mock.expect_post("/storage/v1/b/src-bucket/o/src.txt/copyTo/b/dst-bucket/o/dst.txt")
            .returning_json(
                serde_json::to_value(Object {
                    name: "dst.txt".to_string(),
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
                "src.txt",
                "dst-bucket",
                "dst.txt",
                &Object::default(),
            )
            .await
            .unwrap();
        assert_eq!(copied.name, "dst.txt");
        assert_eq!(copied.bucket.as_deref(), Some("dst-bucket"));
    }

    #[tokio::test]
    async fn test_rewrite_object_single_step() {
        let mut mock = MockClient::new();

        mock.expect_post("/storage/v1/b/src-bucket/o/src.txt/rewriteTo/b/dst-bucket/o/dst.txt")
            .returning_json(
                serde_json::to_value(RewriteResponse {
                    done: Some(true),
                    resource: Some(Object {
                        name: "dst.txt".to_string(),
                        bucket: Some("dst-bucket".to_string()),
                        ..Default::default()
                    }),
                    total_bytes_rewritten: Some("1024".to_string()),
                    object_size: Some("1024".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let resp = storage
            .rewrite_object(
                "src-bucket",
                "src.txt",
                "dst-bucket",
                "dst.txt",
                None,
                &Object::default(),
            )
            .await
            .unwrap();
        assert_eq!(resp.done, Some(true));
        assert_eq!(resp.resource.unwrap().name, "dst.txt");
    }

    // ── lock_bucket_retention_policy Tests ──────────────────────────

    #[tokio::test]
    async fn test_lock_bucket_retention_policy() {
        let mut mock = MockClient::new();

        // Proven URL pattern from integration test:
        // POST /storage/v1/b/{bucket}/lockRetentionPolicy?ifMetagenerationMatch={metageneration}
        mock.expect_post("/storage/v1/b/my-log-bucket/lockRetentionPolicy?ifMetagenerationMatch=5")
            .returning_json(
                serde_json::to_value(Bucket {
                    name: "my-log-bucket".to_string(),
                    metageneration: Some("6".to_string()),
                    retention_policy: Some(serde_json::json!({
                        "retentionPeriod": "2678400",
                        "effectiveTime": "2024-01-01T00:00:00Z",
                        "isLocked": true
                    })),
                    ..Default::default()
                })
                .unwrap(),
            )
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let result = storage
            .lock_bucket_retention_policy("my-log-bucket", "5")
            .await
            .unwrap();

        assert_eq!(result.name, "my-log-bucket");
        assert_eq!(result.metageneration.as_deref(), Some("6"));

        let retention = result.retention_policy.as_ref().unwrap();
        assert_eq!(
            retention.get("isLocked").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            retention.get("retentionPeriod").and_then(|v| v.as_str()),
            Some("2678400")
        );
    }

    #[tokio::test]
    async fn test_lock_bucket_retention_policy_url_encoding() {
        let mut mock = MockClient::new();

        // Bucket names with hyphens are common — ensure URL encoding is correct
        mock.expect_post(
            "/storage/v1/b/cloud-lite-test-logs-bucket/lockRetentionPolicy?ifMetagenerationMatch=1",
        )
        .returning_json(
            serde_json::to_value(Bucket {
                name: "cloud-lite-test-logs-bucket".to_string(),
                retention_policy: Some(serde_json::json!({"isLocked": true})),
                ..Default::default()
            })
            .unwrap(),
        )
        .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let result = storage
            .lock_bucket_retention_policy("cloud-lite-test-logs-bucket", "1")
            .await
            .unwrap();

        assert_eq!(result.name, "cloud-lite-test-logs-bucket");
        let retention = result.retention_policy.as_ref().unwrap();
        assert_eq!(
            retention.get("isLocked").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[tokio::test]
    async fn test_list_buckets_stream_paginates() {
        use futures::StreamExt;

        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b?project=proj&pageToken=tok2")
            .returning_json(
                serde_json::to_value(Buckets {
                    items: vec![Bucket {
                        name: "bucket-3".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .unwrap(),
            );

        mock.expect_get("/storage/v1/b?project=proj")
            .returning_json(
                serde_json::to_value(Buckets {
                    items: vec![
                        Bucket {
                            name: "bucket-1".to_string(),
                            ..Default::default()
                        },
                        Bucket {
                            name: "bucket-2".to_string(),
                            ..Default::default()
                        },
                    ],
                    next_page_token: Some("tok2".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();
        let stream = storage.list_buckets_stream("proj", None);
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(bucket)) = stream.next().await {
            names.push(bucket.name);
        }
        assert_eq!(names, vec!["bucket-1", "bucket-2", "bucket-3"]);
    }

    #[tokio::test]
    async fn test_list_objects_stream_paginates() {
        use futures::StreamExt;

        let mut mock = MockClient::new();

        mock.expect_get("/storage/v1/b/my-bucket/o?prefix=data%2F&pageToken=otok")
            .returning_json(
                serde_json::to_value(Objects {
                    items: vec![Object {
                        name: "data/c.txt".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .unwrap(),
            );

        mock.expect_get("/storage/v1/b/my-bucket/o?prefix=data%2F")
            .returning_json(
                serde_json::to_value(Objects {
                    items: vec![
                        Object {
                            name: "data/a.txt".to_string(),
                            ..Default::default()
                        },
                        Object {
                            name: "data/b.txt".to_string(),
                            ..Default::default()
                        },
                    ],
                    next_page_token: Some("otok".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();
        let stream = storage.list_objects_stream("my-bucket", Some("data/"));
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(obj)) = stream.next().await {
            names.push(obj.name);
        }
        assert_eq!(names, vec!["data/a.txt", "data/b.txt", "data/c.txt"]);
    }

    #[tokio::test]
    async fn test_rewrite_object_multi_step() {
        let mut mock = MockClient::new();

        // Register more specific path first (StartsWith matching)
        mock.expect_post(
            "/storage/v1/b/src/o/big.bin/rewriteTo/b/dst/o/big.bin?rewriteToken=abc123",
        )
        .returning_json(
            serde_json::to_value(RewriteResponse {
                done: Some(true),
                resource: Some(Object {
                    name: "big.bin".to_string(),
                    bucket: Some("dst".to_string()),
                    ..Default::default()
                }),
                total_bytes_rewritten: Some("1000".to_string()),
                object_size: Some("1000".to_string()),
                ..Default::default()
            })
            .unwrap(),
        );

        mock.expect_post("/storage/v1/b/src/o/big.bin/rewriteTo/b/dst/o/big.bin")
            .returning_json(
                serde_json::to_value(RewriteResponse {
                    done: Some(false),
                    rewrite_token: Some("abc123".to_string()),
                    total_bytes_rewritten: Some("500".to_string()),
                    object_size: Some("1000".to_string()),
                    ..Default::default()
                })
                .unwrap(),
            );

        let client = GcpHttpClient::from_mock(mock);
        let storage = client.storage();

        let resp1 = storage
            .rewrite_object("src", "big.bin", "dst", "big.bin", None, &Object::default())
            .await
            .unwrap();
        assert_eq!(resp1.done, Some(false));
        let token = resp1.rewrite_token.as_deref().unwrap();

        let resp2 = storage
            .rewrite_object(
                "src",
                "big.bin",
                "dst",
                "big.bin",
                Some(token),
                &Object::default(),
            )
            .await
            .unwrap();
        assert_eq!(resp2.done, Some(true));
        assert_eq!(resp2.total_bytes_rewritten.as_deref(), Some("1000"));
    }
}
