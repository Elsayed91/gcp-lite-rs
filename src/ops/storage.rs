//! Operation contracts for the Cloud Storage JSON API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/storage.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::storage::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Cloud Storage JSON API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::storage::StorageClient`] instead.
pub struct StorageOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> StorageOps<'a> {
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
        "https://storage.googleapis.com/storage/v1"
    }

    /// Returns metadata for the specified bucket.
    ///
    /// **GCP API**: `GET b/{bucket}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/get>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `generation` — If present, specifies the generation of the bucket. This is required if softDeleted is true.
    /// - `ifMetagenerationMatch` — Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given val
    /// - `ifMetagenerationNotMatch` — Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the gi
    /// - `projection` — Set of properties to return. Defaults to noAcl.
    /// - `softDeleted` — If true, return the soft-deleted version of this bucket. The default is false. For more information, see [Soft Delete](h
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Response
    /// [`Bucket`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket(&self, bucket: &str) -> Result<Bucket> {
        let url = format!("{}/b/{}", self.base_url(), encode(bucket),);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_bucket response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a list of buckets for a given project.
    ///
    /// **GCP API**: `GET b`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/list>
    ///
    /// # Query Parameters
    /// - `maxResults` — Maximum number of buckets to return in a single response. The service will use this parameter or 1,000 items, whichever
    /// - `pageToken` — A previously-returned page token representing part of the larger set of results to view.
    /// - `prefix` — Filter results to buckets whose names begin with this prefix.
    /// - `project` — A valid API project identifier.
    /// - `projection` — Set of properties to return. Defaults to noAcl.
    /// - `returnPartialSuccess` — If true, return a list of bucket resource names for buckets that are in unreachable locations.
    /// - `softDeleted` — If true, only soft-deleted bucket versions will be returned. The default is false. For more information, see [Soft Delet
    /// - `userProject` — The project to be billed for this request.
    ///
    /// # Response
    /// [`Buckets`]
    #[allow(dead_code)]
    pub(crate) async fn list_buckets(
        &self,
        project: &str,
        prefix: &str,
        page_token: &str,
    ) -> Result<Buckets> {
        let url = format!("{}/b", self.base_url(),);
        let url = crate::append_query_params(
            url,
            &[
                ("project", project),
                ("prefix", prefix),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_buckets response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new bucket.
    ///
    /// **GCP API**: `POST b`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/insert>
    ///
    /// # Query Parameters
    /// - `enableObjectRetention` — When set to true, object retention is enabled for this bucket.
    /// - `predefinedAcl` — Apply a predefined set of access controls to this bucket.
    /// - `predefinedDefaultObjectAcl` — Apply a predefined set of default object access controls to this bucket.
    /// - `project` — A valid API project identifier.
    /// - `projection` — Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties,
    /// - `userProject` — The project to be billed for this request.
    ///
    /// # Request Body
    /// [`Bucket`]
    ///
    /// # Response
    /// [`Bucket`]
    #[allow(dead_code)]
    pub(crate) async fn create_bucket(&self, project: &str, body: &Bucket) -> Result<Bucket> {
        let url = format!("{}/b", self.base_url(),);
        let url = crate::append_query_params(url, &[("project", project)]);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_bucket response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Patches a bucket. Changes to the bucket will be readable immediately after writing, but
    /// configuration changes may take time to propagate.
    ///
    /// **GCP API**: `PATCH b/{bucket}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/patch>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `ifMetagenerationMatch` — Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given val
    /// - `ifMetagenerationNotMatch` — Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the gi
    /// - `predefinedAcl` — Apply a predefined set of access controls to this bucket.
    /// - `predefinedDefaultObjectAcl` — Apply a predefined set of default object access controls to this bucket.
    /// - `projection` — Set of properties to return. Defaults to full.
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`Bucket`]
    ///
    /// # Response
    /// [`Bucket`]
    #[allow(dead_code)]
    pub(crate) async fn patch_bucket(&self, bucket: &str, body: &Bucket) -> Result<Bucket> {
        let url = format!("{}/b/{}", self.base_url(), encode(bucket),);
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_bucket response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes an empty bucket. Deletions are permanent unless soft delete is enabled on the
    /// bucket.
    ///
    /// **GCP API**: `DELETE b/{bucket}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/delete>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `ifMetagenerationMatch` — If set, only deletes the bucket if its metageneration matches this value.
    /// - `ifMetagenerationNotMatch` — If set, only deletes the bucket if its metageneration does not match this value.
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    #[allow(dead_code)]
    pub(crate) async fn delete_bucket(&self, bucket: &str) -> Result<()> {
        let url = format!("{}/b/{}", self.base_url(), encode(bucket),);
        let _ = self.client.delete(&url).await?;
        Ok(())
    }

    /// Returns an IAM policy for the specified bucket.
    ///
    /// **GCP API**: `GET b/{bucket}/iam`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/getIamPolicy>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `optionsRequestedPolicyVersion` — The IAM policy format version to be returned. If the optionsRequestedPolicyVersion is for an older version that doesn't
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn get_bucket_iam_policy(&self, bucket: &str) -> Result<Policy> {
        let url = format!("{}/b/{}/iam", self.base_url(), encode(bucket),);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_bucket_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates an IAM policy for the specified bucket.
    ///
    /// **GCP API**: `PUT b/{bucket}/iam`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/setIamPolicy>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`Policy`]
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn set_bucket_iam_policy(
        &self,
        bucket: &str,
        body: &Policy,
    ) -> Result<Policy> {
        let url = format!("{}/b/{}/iam", self.base_url(), encode(bucket),);
        let response = self.client.put(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_bucket_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves an object or its metadata.
    ///
    /// **GCP API**: `GET b/{bucket}/o/{object}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/get>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of the bucket in which the object resides. *(required)*
    /// - `object` — Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts]( *(required)*
    ///
    /// # Query Parameters
    /// - `generation` — If present, selects a specific revision of this object (as opposed to the latest version, the default).
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes t
    /// - `ifGenerationNotMatch` — Makes the operation conditional on whether the object's current generation does not match the given value. If no live ob
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the object's current metageneration matches the given value.
    /// - `ifMetagenerationNotMatch` — Makes the operation conditional on whether the object's current metageneration does not match the given value.
    /// - `projection` — Set of properties to return. Defaults to noAcl.
    /// - `restoreToken` — Restore token used to differentiate soft-deleted objects with the same name and generation. Only applicable for hierarch
    /// - `softDeleted` — If true, only soft-deleted object versions will be listed. The default is false. For more information, see [Soft Delete]
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Response
    /// [`Object`]
    #[allow(dead_code)]
    pub(crate) async fn get_object(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}",
            self.base_url(),
            encode(bucket),
            encode(object),
        );
        let url = crate::append_query_params(url, &[("generation", generation)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_object response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a list of objects matching the criteria.
    ///
    /// **GCP API**: `GET b/{bucket}/o`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/list>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of the bucket in which to look for objects. *(required)*
    ///
    /// # Query Parameters
    /// - `delimiter` — Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not con
    /// - `endOffset` — Filter results to objects whose names are lexicographically before endOffset. If startOffset is also set, the objects li
    /// - `filter` — Filter the returned objects. Currently only supported for the contexts field. If delimiter is set, the returned prefixes
    /// - `includeFoldersAsPrefixes` — Only applicable if delimiter is set to '/'. If true, will also include folders and managed folders (besides objects) in
    /// - `includeTrailingDelimiter` — If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to
    /// - `matchGlob` — Filter results to objects and prefixes that match this glob pattern.
    /// - `maxResults` — Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer
    /// - `pageToken` — A previously-returned page token representing part of the larger set of results to view.
    /// - `prefix` — Filter results to objects whose names begin with this prefix.
    /// - `projection` — Set of properties to return. Defaults to noAcl.
    /// - `softDeleted` — If true, only soft-deleted object versions will be listed. The default is false. For more information, see [Soft Delete]
    /// - `startOffset` — Filter results to objects whose names are lexicographically equal to or after startOffset. If endOffset is also set, the
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    /// - `versions` — If true, lists all versions of an object as distinct results. The default is false. For more information, see [Object Ve
    ///
    /// # Response
    /// [`Objects`]
    #[allow(dead_code)]
    pub(crate) async fn list_objects(
        &self,
        bucket: &str,
        prefix: &str,
        delimiter: &str,
        page_token: &str,
        versions: &str,
    ) -> Result<Objects> {
        let url = format!("{}/b/{}/o", self.base_url(), encode(bucket),);
        let url = crate::append_query_params(
            url,
            &[
                ("prefix", prefix),
                ("delimiter", delimiter),
                ("pageToken", page_token),
                ("versions", versions),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_objects response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Stores a new object and metadata.
    ///
    /// **GCP API**: `POST b/{bucket}/o`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/insert>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any. *(required)*
    ///
    /// # Query Parameters
    /// - `contentEncoding` — If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to set
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes t
    /// - `ifGenerationNotMatch` — Makes the operation conditional on whether the object's current generation does not match the given value. If no live ob
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the object's current metageneration matches the given value.
    /// - `ifMetagenerationNotMatch` — Makes the operation conditional on whether the object's current metageneration does not match the given value.
    /// - `kmsKeyName` — Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, t
    /// - `name` — Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name va
    /// - `predefinedAcl` — Apply a predefined set of access controls to this object.
    /// - `projection` — Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`Object`]
    ///
    /// # Response
    /// [`Object`]
    #[allow(dead_code)]
    pub(crate) async fn create_object(
        &self,
        bucket: &str,
        name: &str,
        body: &Object,
    ) -> Result<Object> {
        let url = format!(
            "{}/b/{}/o",
            "https://storage.googleapis.com/upload/storage/v1",
            encode(bucket),
        );
        let url = crate::append_query_params(url, &[("name", name)]);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_object response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled
    /// for the bucket, or if the generation parameter is used.
    ///
    /// **GCP API**: `DELETE b/{bucket}/o/{object}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/delete>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of the bucket in which the object resides. *(required)*
    /// - `object` — Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts]( *(required)*
    ///
    /// # Query Parameters
    /// - `generation` — If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default).
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes t
    /// - `ifGenerationNotMatch` — Makes the operation conditional on whether the object's current generation does not match the given value. If no live ob
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the object's current metageneration matches the given value.
    /// - `ifMetagenerationNotMatch` — Makes the operation conditional on whether the object's current metageneration does not match the given value.
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    #[allow(dead_code)]
    pub(crate) async fn delete_object(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/b/{}/o/{}",
            self.base_url(),
            encode(bucket),
            encode(object),
        );
        let url = crate::append_query_params(url, &[("generation", generation)]);
        let _ = self.client.delete(&url).await?;
        Ok(())
    }

    /// Concatenates a list of existing objects into a new object in the same bucket.
    ///
    /// **GCP API**: `POST b/{destinationBucket}/o/{destinationObject}/compose`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/compose>
    ///
    /// # Path Parameters
    /// - `destinationBucket` — Name of the bucket containing the source objects. The destination object is stored in this bucket. *(required)*
    /// - `destinationObject` — Name of the new object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Par *(required)*
    ///
    /// # Query Parameters
    /// - `destinationPredefinedAcl` — Apply a predefined set of access controls to the destination object.
    /// - `dropContextGroups` — Specifies which groups of Object Contexts from the source object(s) should be dropped from the destination object.
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes t
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the object's current metageneration matches the given value.
    /// - `kmsKeyName` — Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, t
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`ComposeRequest`]
    ///
    /// # Response
    /// [`Object`]
    #[allow(dead_code)]
    pub(crate) async fn compose_object(
        &self,
        destination_bucket: &str,
        destination_object: &str,
        body: &ComposeRequest,
    ) -> Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}/compose",
            self.base_url(),
            encode(destination_bucket),
            encode(destination_object),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse compose_object response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Copies a source object to a destination object. Optionally overrides metadata.
    ///
    /// **GCP API**: `POST b/{sourceBucket}/o/{sourceObject}/copyTo/b/{destinationBucket}/o/{destinationObject}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/copy>
    ///
    /// # Path Parameters
    /// - `sourceBucket` — Name of the bucket in which to find the source object. *(required)*
    /// - `sourceObject` — Name of the source object. For information about how to URL encode object names to be path safe, see [Encoding URI Path  *(required)*
    /// - `destinationBucket` — Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For i *(required)*
    /// - `destinationObject` — Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's nam *(required)*
    ///
    /// # Query Parameters
    /// - `destinationKmsKeyName` — Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, t
    /// - `destinationPredefinedAcl` — Apply a predefined set of access controls to the destination object.
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the destination object's current generation matches the given value. Setting
    /// - `ifGenerationNotMatch` — Makes the operation conditional on whether the destination object's current generation does not match the given value. I
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    /// - `ifMetagenerationNotMatch` — Makes the operation conditional on whether the destination object's current metageneration does not match the given valu
    /// - `ifSourceGenerationMatch` — Makes the operation conditional on whether the source object's current generation matches the given value.
    /// - `ifSourceGenerationNotMatch` — Makes the operation conditional on whether the source object's current generation does not match the given value.
    /// - `ifSourceMetagenerationMatch` — Makes the operation conditional on whether the source object's current metageneration matches the given value.
    /// - `ifSourceMetagenerationNotMatch` — Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    /// - `projection` — Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults
    /// - `sourceGeneration` — If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`Object`]
    ///
    /// # Response
    /// [`Object`]
    #[allow(dead_code)]
    pub(crate) async fn copy_object(
        &self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        body: &Object,
    ) -> Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}/copyTo/b/{}/o/{}",
            self.base_url(),
            encode(source_bucket),
            encode(source_object),
            encode(destination_bucket),
            encode(destination_object),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse copy_object response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Rewrites a source object to a destination object. Optionally overrides metadata.
    ///
    /// **GCP API**: `POST b/{sourceBucket}/o/{sourceObject}/rewriteTo/b/{destinationBucket}/o/{destinationObject}`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//objects/rewrite>
    ///
    /// # Path Parameters
    /// - `sourceBucket` — Name of the bucket in which to find the source object. *(required)*
    /// - `sourceObject` — Name of the source object. For information about how to URL encode object names to be path safe, see [Encoding URI Path  *(required)*
    /// - `destinationBucket` — Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any. *(required)*
    /// - `destinationObject` — Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's nam *(required)*
    ///
    /// # Query Parameters
    /// - `destinationKmsKeyName` — Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, t
    /// - `destinationPredefinedAcl` — Apply a predefined set of access controls to the destination object.
    /// - `dropContextGroups` — Specifies which groups of Object Contexts from the source object should be dropped from the destination object.
    /// - `ifGenerationMatch` — Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes t
    /// - `ifGenerationNotMatch` — Makes the operation conditional on whether the object's current generation does not match the given value. If no live ob
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    /// - `ifMetagenerationNotMatch` — Makes the operation conditional on whether the destination object's current metageneration does not match the given valu
    /// - `ifSourceGenerationMatch` — Makes the operation conditional on whether the source object's current generation matches the given value.
    /// - `ifSourceGenerationNotMatch` — Makes the operation conditional on whether the source object's current generation does not match the given value.
    /// - `ifSourceMetagenerationMatch` — Makes the operation conditional on whether the source object's current metageneration matches the given value.
    /// - `ifSourceMetagenerationNotMatch` — Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    /// - `maxBytesRewrittenPerCall` — The maximum number of bytes that will be rewritten per rewrite request. Most callers shouldn't need to specify this para
    /// - `projection` — Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults
    /// - `rewriteToken` — Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite r
    /// - `sourceGeneration` — If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Request Body
    /// [`Object`]
    ///
    /// # Response
    /// [`RewriteResponse`]
    #[allow(dead_code)]
    pub(crate) async fn rewrite_object(
        &self,
        source_bucket: &str,
        source_object: &str,
        destination_bucket: &str,
        destination_object: &str,
        rewrite_token: &str,
        body: &Object,
    ) -> Result<RewriteResponse> {
        let url = format!(
            "{}/b/{}/o/{}/rewriteTo/b/{}/o/{}",
            self.base_url(),
            encode(source_bucket),
            encode(source_object),
            encode(destination_bucket),
            encode(destination_object),
        );
        let url = crate::append_query_params(url, &[("rewriteToken", rewrite_token)]);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse rewrite_object response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Locks retention policy on a bucket.
    ///
    /// **GCP API**: `POST b/{bucket}/lockRetentionPolicy`
    /// **Reference**: <https://developers.google.com/storage/docs/json_api//buckets/lockRetentionPolicy>
    ///
    /// # Path Parameters
    /// - `bucket` — Name of a bucket. *(required)*
    ///
    /// # Query Parameters
    /// - `ifMetagenerationMatch` — Makes the operation conditional on whether bucket's current metageneration matches the given value.
    /// - `userProject` — The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// # Response
    /// [`Bucket`]
    #[allow(dead_code)]
    pub(crate) async fn lock_bucket_retention_policy(
        &self,
        bucket: &str,
        if_metageneration_match: &str,
    ) -> Result<Bucket> {
        let url = format!(
            "{}/b/{}/lockRetentionPolicy",
            self.base_url(),
            encode(bucket),
        );
        let url =
            crate::append_query_params(url, &[("ifMetagenerationMatch", if_metageneration_match)]);
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse lock_bucket_retention_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bucket() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/storage/v1/b/test-bucket")
            .returning_json(serde_json::to_value(Bucket::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops.get_bucket("test-bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_buckets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/storage/v1/b?project=test-project&prefix=test-prefix&pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(Buckets::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .list_buckets("test-project", "test-prefix", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_bucket() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/storage/v1/b?project=test-project")
            .returning_json(serde_json::to_value(Bucket::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Bucket::fixture();
        let result = ops.create_bucket("test-project", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_bucket() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/storage/v1/b/test-bucket")
            .returning_json(serde_json::to_value(Bucket::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Bucket::fixture();
        let result = ops.patch_bucket("test-bucket", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/storage/v1/b/test-bucket")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops.delete_bucket("test-bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bucket_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/storage/v1/b/test-bucket/iam")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops.get_bucket_iam_policy("test-bucket").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_bucket_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/storage/v1/b/test-bucket/iam")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Policy::fixture();
        let result = ops.set_bucket_iam_policy("test-bucket", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/storage/v1/b/test-bucket/o/test-object?generation=test-generation")
            .returning_json(serde_json::to_value(Object::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .get_object("test-bucket", "test-object", "test-generation")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_objects() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/storage/v1/b/test-bucket/o?prefix=test-prefix&delimiter=test-delimiter&pageToken=test-pageToken&versions=test-versions")
            .returning_json(serde_json::to_value(Objects::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .list_objects(
                "test-bucket",
                "test-prefix",
                "test-delimiter",
                "test-pageToken",
                "test-versions",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/upload/storage/v1/b/test-bucket/o?name=test-name")
            .returning_json(serde_json::to_value(Object::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Object::fixture();
        let result = ops.create_object("test-bucket", "test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/storage/v1/b/test-bucket/o/test-object?generation=test-generation")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .delete_object("test-bucket", "test-object", "test-generation")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_compose_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/storage/v1/b/test-destinationBucket/o/test-destinationObject/compose")
            .returning_json(serde_json::to_value(Object::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = ComposeRequest::fixture();
        let result = ops
            .compose_object("test-destinationBucket", "test-destinationObject", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_copy_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/storage/v1/b/test-sourceBucket/o/test-sourceObject/copyTo/b/test-destinationBucket/o/test-destinationObject")
            .returning_json(serde_json::to_value(Object::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Object::fixture();
        let result = ops
            .copy_object(
                "test-sourceBucket",
                "test-sourceObject",
                "test-destinationBucket",
                "test-destinationObject",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rewrite_object() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/storage/v1/b/test-sourceBucket/o/test-sourceObject/rewriteTo/b/test-destinationBucket/o/test-destinationObject?rewriteToken=test-rewriteToken")
            .returning_json(serde_json::to_value(RewriteResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = Object::fixture();
        let result = ops
            .rewrite_object(
                "test-sourceBucket",
                "test-sourceObject",
                "test-destinationBucket",
                "test-destinationObject",
                "test-rewriteToken",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_lock_bucket_retention_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/storage/v1/b/test-bucket/lockRetentionPolicy?ifMetagenerationMatch=test-ifMetagenerationMatch")
            .returning_json(serde_json::to_value(Bucket::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .lock_bucket_retention_policy("test-bucket", "test-ifMetagenerationMatch")
            .await;
        assert!(result.is_ok());
    }
}
