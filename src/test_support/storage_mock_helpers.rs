//! MockClient helpers for Cloud Storage JSON API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Storage JSON API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait StorageMockHelpers {
    /// Helper to expect `get_bucket`: Returns metadata for the specified bucket.
    fn expect_get_bucket(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_buckets`: Retrieves a list of buckets for a given project.
    fn expect_list_buckets(
        &mut self,
        project: &str,
        prefix: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_bucket`: Creates a new bucket.
    fn expect_create_bucket(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_bucket`: Patches a bucket. Changes to the bucket will be readable
    /// immediately after writing, but configuration changes may take time to propagate.
    fn expect_patch_bucket(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_bucket`: Deletes an empty bucket. Deletions are permanent unless
    /// soft delete is enabled on the bucket.
    fn expect_delete_bucket(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_bucket_iam_policy`: Returns an IAM policy for the specified bucket.
    fn expect_get_bucket_iam_policy(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_bucket_iam_policy`: Updates an IAM policy for the specified bucket.
    fn expect_set_bucket_iam_policy(&mut self, bucket: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_object`: Retrieves an object or its metadata.
    fn expect_get_object(
        &mut self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_objects`: Retrieves a list of objects matching the criteria.
    fn expect_list_objects(
        &mut self,
        bucket: &str,
        prefix: &str,
        delimiter: &str,
        page_token: &str,
        versions: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_object`: Stores a new object and metadata.
    fn expect_create_object(&mut self, bucket: &str, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_object`: Deletes an object and its metadata. Deletions are
    /// permanent if versioning is not enabled for the bucket, or if the generation parameter is
    /// used.
    fn expect_delete_object(
        &mut self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `compose_object`: Concatenates a list of existing objects into a new object
    /// in the same bucket.
    fn expect_compose_object(
        &mut self,
        destination_bucket: &str,
        destination_object: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `copy_object`: Copies a source object to a destination object. Optionally
    /// overrides metadata.
    fn expect_copy_object(
        &mut self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `rewrite_object`: Rewrites a source object to a destination object.
    /// Optionally overrides metadata.
    fn expect_rewrite_object(
        &mut self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        rewrite_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `lock_bucket_retention_policy`: Locks retention policy on a bucket.
    fn expect_lock_bucket_retention_policy(
        &mut self,
        bucket: &str,
        if_metageneration_match: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl StorageMockHelpers for MockClient {
    /// Helper to expect `get_bucket`: Returns metadata for the specified bucket.
    fn expect_get_bucket(&mut self, bucket: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{bucket}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_buckets`: Retrieves a list of buckets for a given project.
    fn expect_list_buckets(
        &mut self,
        project: &str,
        prefix: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/storage/v1/b".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !project.is_empty() {
            __qp.push(format!("project={}", project));
        }
        if !prefix.is_empty() {
            __qp.push(format!("prefix={}", prefix));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_bucket`: Creates a new bucket.
    fn expect_create_bucket(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/storage/v1/b".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !project.is_empty() {
            __qp.push(format!("project={}", project));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `patch_bucket`: Patches a bucket. Changes to the bucket will be readable
    /// immediately after writing, but configuration changes may take time to propagate.
    fn expect_patch_bucket(&mut self, bucket: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{bucket}");
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_bucket`: Deletes an empty bucket. Deletions are permanent unless
    /// soft delete is enabled on the bucket.
    fn expect_delete_bucket(&mut self, bucket: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{bucket}");
        self.expect_delete(&path)
    }

    /// Helper to expect `get_bucket_iam_policy`: Returns an IAM policy for the specified bucket.
    fn expect_get_bucket_iam_policy(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{bucket}/iam");
        self.expect_get(&path)
    }

    /// Helper to expect `set_bucket_iam_policy`: Updates an IAM policy for the specified bucket.
    fn expect_set_bucket_iam_policy(
        &mut self,
        bucket: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{bucket}/iam");
        self.expect_put(&path)
    }

    /// Helper to expect `get_object`: Retrieves an object or its metadata.
    fn expect_get_object(
        &mut self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/storage/v1/b/{bucket}/o/{object}");
        let mut __qp: Vec<String> = Vec::new();
        if !generation.is_empty() {
            __qp.push(format!("generation={}", generation));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `list_objects`: Retrieves a list of objects matching the criteria.
    fn expect_list_objects(
        &mut self,
        bucket: &str,
        prefix: &str,
        delimiter: &str,
        page_token: &str,
        versions: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/storage/v1/b/{bucket}/o");
        let mut __qp: Vec<String> = Vec::new();
        if !prefix.is_empty() {
            __qp.push(format!("prefix={}", prefix));
        }
        if !delimiter.is_empty() {
            __qp.push(format!("delimiter={}", delimiter));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !versions.is_empty() {
            __qp.push(format!("versions={}", versions));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_object`: Stores a new object and metadata.
    fn expect_create_object(
        &mut self,
        bucket: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/upload/storage/v1/b/{bucket}/o");
        let mut __qp: Vec<String> = Vec::new();
        if !name.is_empty() {
            __qp.push(format!("name={}", name));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `delete_object`: Deletes an object and its metadata. Deletions are
    /// permanent if versioning is not enabled for the bucket, or if the generation parameter is
    /// used.
    fn expect_delete_object(
        &mut self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/storage/v1/b/{bucket}/o/{object}");
        let mut __qp: Vec<String> = Vec::new();
        if !generation.is_empty() {
            __qp.push(format!("generation={}", generation));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_delete(&path)
    }

    /// Helper to expect `compose_object`: Concatenates a list of existing objects into a new object
    /// in the same bucket.
    fn expect_compose_object(
        &mut self,
        destination_bucket: &str,
        destination_object: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/storage/v1/b/{destination_bucket}/o/{destination_object}/compose");
        self.expect_post(&path)
    }

    /// Helper to expect `copy_object`: Copies a source object to a destination object. Optionally
    /// overrides metadata.
    fn expect_copy_object(
        &mut self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/storage/v1/b/{source_bucket}/o/{source_object}/copyTo/b/{destination_bucket}/o/{destination_object}"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `rewrite_object`: Rewrites a source object to a destination object.
    /// Optionally overrides metadata.
    fn expect_rewrite_object(
        &mut self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        rewrite_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/storage/v1/b/{source_bucket}/o/{source_object}/rewriteTo/b/{destination_bucket}/o/{destination_object}"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !rewrite_token.is_empty() {
            __qp.push(format!("rewriteToken={}", rewrite_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `lock_bucket_retention_policy`: Locks retention policy on a bucket.
    fn expect_lock_bucket_retention_policy(
        &mut self,
        bucket: &str,
        if_metageneration_match: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/storage/v1/b/{bucket}/lockRetentionPolicy");
        let mut __qp: Vec<String> = Vec::new();
        if !if_metageneration_match.is_empty() {
            __qp.push(format!("ifMetagenerationMatch={}", if_metageneration_match));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }
}
