//! Types for the Cloud Storage JSON API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/storage/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A bucket.
///
/// **GCP API**: `storage.v1.Bucket`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Bucket>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    /// Access controls on the bucket.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub acl: Vec<BucketAccessControl>,

    /// The bucket's Autoclass configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoclass: Option<serde_json::Value>,

    /// The bucket's billing configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<serde_json::Value>,

    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<serde_json::Value>,

    /// The bucket's custom placement configuration for Custom Dual Regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_placement_config: Option<serde_json::Value>,

    /// The default value for event-based hold on newly created objects in this bucket. Event-
    /// based hold is a way to retain objects indefinitely until an event occurs, signified by
    /// the hold's release. After being released, such objects will be subject to bucket-level
    /// retention (if any). One sample use case of this flag is for banks to hold loan documents
    /// for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years
    /// and the event is loan being paid in full. In this example, these objects will be held
    /// intact for any number of years until the event has occurred (event-based hold on the
    /// object is released) and then 3 more years after that. That means retention duration of
    /// the objects begins from the moment event-based hold transitioned from true to false.
    /// Objects under event-based hold cannot be deleted, overwritten or archived until the hold
    /// is removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_event_based_hold: Option<bool>,

    /// Default access controls to apply to new objects when no ACL is provided.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_object_acl: Vec<ObjectAccessControl>,

    /// Encryption configuration for a bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<serde_json::Value>,

    /// HTTP 1.1 Entity tag for the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The generation of this bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,

    /// The hard delete time of the bucket in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_delete_time: Option<String>,

    /// The bucket's hierarchical namespace configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchical_namespace: Option<serde_json::Value>,

    /// The bucket's IAM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_configuration: Option<BucketIamConfiguration>,

    /// The ID of the bucket. For buckets, the id and name properties are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The bucket's IP filter configuration. Specifies the network sources that are allowed to
    /// access the operations on the bucket, as well as its underlying objects. Only enforced
    /// when the mode is set to 'Enabled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<serde_json::Value>,

    /// The kind of item this is. For buckets, this is always storage#bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// User-provided labels, in key/value pairs.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// The bucket's lifecycle configuration. See [Lifecycle
    /// Management](https://cloud.google.com/storage/docs/lifecycle) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<serde_json::Value>,

    /// The location of the bucket. Object data for objects in the bucket resides in physical
    /// storage within this region. Defaults to US. See the [Developer's
    /// Guide](https://cloud.google.com/storage/docs/locations) for the authoritative list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// The type of the bucket location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,

    /// The bucket's logging configuration, which defines the destination bucket and optional
    /// name prefix for the current bucket's logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<serde_json::Value>,

    /// The metadata generation of this bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,

    /// The name of the bucket.
    pub name: String,

    /// The bucket's object retention config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_retention: Option<serde_json::Value>,

    /// The owner of the bucket. This is always the project team's owner group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,

    /// The project number of the project the bucket belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,

    /// The bucket's retention policy. The retention policy enforces a minimum retention time
    /// for all objects contained in the bucket, based on their creation time. Any attempt to
    /// overwrite or delete objects younger than the retention period will result in a
    /// PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from
    /// the bucket via a storage.buckets.update operation. A locked retention policy cannot be
    /// removed or shortened in duration for the lifetime of the bucket. Attempting to remove or
    /// decrease period of a locked retention policy will result in a PERMISSION_DENIED error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<serde_json::Value>,

    /// The Recovery Point Objective (RPO) of this bucket. Set to ASYNC_TURBO to turn on Turbo
    /// Replication on a bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpo: Option<String>,

    /// Reserved for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfies_pzi: Option<bool>,

    /// Reserved for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,

    /// The URI of this bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// The bucket's soft delete policy, which defines the period of time that soft-deleted
    /// objects will be retained, and cannot be permanently deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_delete_policy: Option<BucketSoftDeletePolicy>,

    /// The soft delete time of the bucket in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_delete_time: Option<String>,

    /// The bucket's default storage class, used whenever no storageClass is specified for a
    /// newly-created object. This defines how objects in the bucket are stored and determines
    /// the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD,
    /// NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not
    /// specified when the bucket is created, it will default to STANDARD. For more information,
    /// see [Storage Classes](https://cloud.google.com/storage/docs/storage-classes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,

    /// The creation time of the bucket in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,

    /// The modification time of the bucket in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    /// The bucket's versioning configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning: Option<serde_json::Value>,

    /// The bucket's website configuration, controlling how the service behaves when accessing
    /// bucket contents as a web site. See the [Static Website
    /// Examples](https://cloud.google.com/storage/docs/static-website) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<serde_json::Value>,
}

impl Bucket {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            acl: vec![],
            autoclass: Default::default(),
            billing: Default::default(),
            cors: vec![],
            custom_placement_config: Default::default(),
            default_event_based_hold: Some(false),
            default_object_acl: vec![],
            encryption: Default::default(),
            etag: Some("test-etag".into()),
            generation: Some("test-generation".into()),
            hard_delete_time: Some("test-hard_delete_time".into()),
            hierarchical_namespace: Default::default(),
            iam_configuration: Default::default(),
            id: Some("test-id".into()),
            ip_filter: Default::default(),
            kind: Some("test-kind".into()),
            labels: Default::default(),
            lifecycle: Default::default(),
            location: Some("test-location".into()),
            location_type: Some("test-location_type".into()),
            logging: Default::default(),
            metageneration: Some("test-metageneration".into()),
            name: "test-bucket".into(),
            object_retention: Default::default(),
            owner: Default::default(),
            project_number: Some("test-project_number".into()),
            retention_policy: Default::default(),
            rpo: Some("test-rpo".into()),
            satisfies_pzi: Some(false),
            satisfies_pzs: Some(false),
            self_link: Some("test-self_link".into()),
            soft_delete_policy: Default::default(),
            soft_delete_time: Some("test-soft_delete_time".into()),
            storage_class: Some("test-storage_class".into()),
            time_created: Some("test-time_created".into()),
            updated: Some("test-updated".into()),
            versioning: Default::default(),
            website: Default::default(),
        }
    }
}

/// A list of buckets.
///
/// **GCP API**: `storage.v1.Buckets`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Buckets>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buckets {
    /// The list of items.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Bucket>,

    /// The kind of item this is. For lists of buckets, this is always storage#buckets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The continuation token, used to page through large result sets. Provide this value in a
    /// subsequent request to return the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Buckets {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// An access-control entry.
///
/// **GCP API**: `storage.v1.BucketAccessControl`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//BucketAccessControl>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketAccessControl {
    /// The name of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// The domain associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The email address associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The entity holding the permission, in one of the following forms:
    /// - user-userId
    /// - user-email
    /// - group-groupId
    /// - group-email
    /// - domain-domain
    /// - project-team-projectId
    /// - allUsers
    /// - allAuthenticatedUsers Examples:
    /// - The user liz@example.com would be user-liz@example.com.
    /// - The group example@googlegroups.com would be group-example@googlegroups.com.
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity
    ///   would be domain-example.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// The ID for the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,

    /// HTTP 1.1 Entity tag for the access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The ID of the access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The kind of item this is. For bucket access control entries, this is always
    /// storage#bucketAccessControl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The project team associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_team: Option<serde_json::Value>,

    /// The access permission for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The link to this access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl BucketAccessControl {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bucket: Some("test-bucket".into()),
            domain: Some("test-domain".into()),
            email: Some("test-email".into()),
            entity: Some("test-entity".into()),
            entity_id: Some("test-entity_id".into()),
            etag: Some("test-etag".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            project_team: Default::default(),
            role: Some("test-role".into()),
            self_link: Some("test-self_link".into()),
        }
    }
}

/// An object.
///
/// **GCP API**: `storage.v1.Object`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Object>
///
/// ## Coverage
/// 37 of 38 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    /// Access controls on the object.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub acl: Vec<ObjectAccessControl>,

    /// The name of the bucket containing this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// Cache-Control directive for the object data. If omitted, and the object is accessible to
    /// all anonymous users, the default will be public, max-age=3600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,

    /// Number of underlying components that make up this object. Components are accumulated by
    /// compose operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_count: Option<i32>,

    /// Content-Disposition of the object data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,

    /// Content-Encoding of the object data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,

    /// Content-Language of the object data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_language: Option<String>,

    /// Content-Type of the object data. If an object is stored without a Content-Type, it is
    /// served as application/octet-stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-
    /// endian byte order. For more information about using the CRC32c checksum, see [Data
    /// Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32c: Option<String>,

    /// A timestamp in RFC 3339 format specified by the user for an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_time: Option<String>,

    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption: Option<serde_json::Value>,

    /// HTTP 1.1 Entity tag for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Whether an object is under event-based hold. Event-based hold is a way to retain objects
    /// until an event occurs, which is signified by the hold's release (i.e. this value is set
    /// to false). After being released (set to false), such objects will be subject to bucket-
    /// level retention (if any). One sample use case of this flag is for banks to hold loan
    /// documents for at least 3 years after loan is paid in full. Here, bucket-level retention
    /// is 3 years and the event is the loan being paid in full. In this example, these objects
    /// will be held intact for any number of years until the event has occurred (event-based
    /// hold on the object is released) and then 3 more years after that. That means retention
    /// duration of the objects begins from the moment event-based hold transitioned from true
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_based_hold: Option<bool>,

    /// The content generation of this object. Used for object versioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,

    /// This is the time (in the future) when the soft-deleted object will no longer be
    /// restorable. It is equal to the soft delete time plus the current soft delete retention
    /// duration of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_delete_time: Option<String>,

    /// The ID of the object, including the bucket name, object name, and generation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The kind of item this is. For objects, this is always storage#object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Not currently supported. Specifying the parameter causes the request to fail with status
    /// code 400
    /// - Bad Request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,

    /// MD5 hash of the data; encoded using base64. For more information about using the MD5
    /// hash, see [Data Validation and Change
    /// Detection](https://cloud.google.com/storage/docs/data-validation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5_hash: Option<String>,

    /// Media download link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_link: Option<String>,

    /// User-provided metadata, in key/value pairs.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,

    /// The version of the metadata for this object at this generation. Used for preconditions
    /// and for detecting changes in metadata. A metageneration number is only meaningful in the
    /// context of a particular generation of a particular object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,

    /// The name of the object. Required if not specified by URL parameter.
    pub name: String,

    /// The owner of the object. This will always be the uploader of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,

    /// Restore token used to differentiate deleted objects with the same name and generation.
    /// This field is only returned for deleted objects in hierarchical namespace buckets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_token: Option<String>,

    /// A collection of object level retention parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<serde_json::Value>,

    /// A server-determined value that specifies the earliest time that the object's retention
    /// period expires. This value is in RFC 3339 format. Note 1: This field is not provided for
    /// objects with an active event-based hold, since retention expiration is unknown until the
    /// hold is removed. Note 2: This value can be provided even when temporary hold is set (so
    /// that the user can reason about policy without having to first unset the temporary hold).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_expiration_time: Option<String>,

    /// The link to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Content-Length of the data in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// The time at which the object became soft-deleted in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_delete_time: Option<String>,

    /// Storage class of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,

    /// Whether an object is under temporary hold. While this flag is set to true, the object is
    /// protected against deletion and overwrites. A common use case of this flag is regulatory
    /// investigations where objects need to be retained while the investigation is ongoing.
    /// Note that unlike event-based hold, temporary hold does not impact retention expiration
    /// time of an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_hold: Option<bool>,

    /// The creation time of the object in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,

    /// The time at which the object became noncurrent in RFC 3339 format. Will be returned if
    /// and only if this version of the object has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<String>,

    /// The time when the object was finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_finalized: Option<String>,

    /// The time at which the object's storage class was last changed. When the object is
    /// initially created, it will be set to timeCreated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_storage_class_updated: Option<String>,

    /// The modification time of the object metadata in RFC 3339 format. Set initially to object
    /// creation time and then updated whenever any metadata of the object changes. This
    /// includes changes made by a requester, such as modifying custom metadata, as well as
    /// changes made by Cloud Storage on behalf of a requester, such as changing the storage
    /// class based on an Object Lifecycle Configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl Object {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            acl: vec![],
            bucket: Some("test-bucket".into()),
            cache_control: Some("test-cache_control".into()),
            component_count: Some(100),
            content_disposition: Some("test-content_disposition".into()),
            content_encoding: Some("test-content_encoding".into()),
            content_language: Some("test-content_language".into()),
            content_type: Some("test-content_type".into()),
            crc32c: Some("test-crc32c".into()),
            custom_time: Some("test-custom_time".into()),
            customer_encryption: Default::default(),
            etag: Some("test-etag".into()),
            event_based_hold: Some(false),
            generation: Some("test-generation".into()),
            hard_delete_time: Some("test-hard_delete_time".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            kms_key_name: Some("test-kms_key_name".into()),
            md5_hash: Some("test-md5_hash".into()),
            media_link: Some("test-media_link".into()),
            metadata: Default::default(),
            metageneration: Some("test-metageneration".into()),
            name: "test-object".into(),
            owner: Default::default(),
            restore_token: Some("test-restore_token".into()),
            retention: Default::default(),
            retention_expiration_time: Some("test-retention_expiration_time".into()),
            self_link: Some("test-self_link".into()),
            size: Some("test-size".into()),
            soft_delete_time: Some("test-soft_delete_time".into()),
            storage_class: Some("test-storage_class".into()),
            temporary_hold: Some(false),
            time_created: Some("test-time_created".into()),
            time_deleted: Some("test-time_deleted".into()),
            time_finalized: Some("test-time_finalized".into()),
            time_storage_class_updated: Some("test-time_storage_class_updated".into()),
            updated: Some("test-updated".into()),
        }
    }
}

/// A list of objects.
///
/// **GCP API**: `storage.v1.Objects`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Objects>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objects {
    /// The list of items.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Object>,

    /// The kind of item this is. For lists of objects, this is always storage#objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The continuation token, used to page through large result sets. Provide this value in a
    /// subsequent request to return the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The list of prefixes of objects matching-but-not-listed up to and including the
    /// requested delimiter.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prefixes: Vec<String>,
}

impl Objects {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
            prefixes: vec![],
        }
    }
}

/// An access-control entry.
///
/// **GCP API**: `storage.v1.ObjectAccessControl`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//ObjectAccessControl>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectAccessControl {
    /// The name of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// The domain associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// The email address associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The entity holding the permission, in one of the following forms:
    /// - user-userId
    /// - user-email
    /// - group-groupId
    /// - group-email
    /// - domain-domain
    /// - project-team-projectId
    /// - allUsers
    /// - allAuthenticatedUsers Examples:
    /// - The user liz@example.com would be user-liz@example.com.
    /// - The group example@googlegroups.com would be group-example@googlegroups.com.
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity
    ///   would be domain-example.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// The ID for the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,

    /// HTTP 1.1 Entity tag for the access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The content generation of the object, if applied to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,

    /// The ID of the access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The kind of item this is. For object access control entries, this is always
    /// storage#objectAccessControl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of the object, if applied to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// The project team associated with the entity, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_team: Option<serde_json::Value>,

    /// The access permission for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The link to this access-control entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl ObjectAccessControl {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bucket: Some("test-bucket".into()),
            domain: Some("test-domain".into()),
            email: Some("test-email".into()),
            entity: Some("test-entity".into()),
            entity_id: Some("test-entity_id".into()),
            etag: Some("test-etag".into()),
            generation: Some("test-generation".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            object: Some("test-object".into()),
            project_team: Default::default(),
            role: Some("test-role".into()),
            self_link: Some("test-self_link".into()),
        }
    }
}

/// A bucket/object/managedFolder IAM policy.
///
/// **GCP API**: `storage.v1.Policy`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Policy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    /// An association between a role, which comes with a set of permissions, and members who
    /// may assume that role.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<PolicyBinding>,

    /// HTTP 1.1 Entity tag for the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The kind of item this is. For policies, this is always storage#policy. This field is
    /// ignored on input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The ID of the resource to which this policy belongs. Will be of the form
    /// projects/_/buckets/bucket for buckets, projects/_/buckets/bucket/objects/object for
    /// objects, and projects/_/buckets/bucket/managedFolders/managedFolder. A specific
    /// generation may be specified by appending #generationNumber to the end of the object
    /// name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can
    /// be denoted with #0. This field is ignored on input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// The IAM policy format version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl Policy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bindings: vec![],
            etag: Some("test-etag".into()),
            kind: Some("test-kind".into()),
            resource_id: Some("test-resource_id".into()),
            version: Some(100),
        }
    }
}

/// Represents an expression text. Example: title: "User account presence" description:
/// "Determines whether the request has a user account" expression: "size(request.user) > 0"
///
/// **GCP API**: `storage.v1.Expr`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//Expr>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expr {
    /// An optional description of the expression. This is a longer text which describes the
    /// expression, e.g. when hovered over it in a UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Textual representation of an expression in Common Expression Language syntax. The
    /// application context of the containing message determines which well-known feature set of
    /// CEL is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An optional string indicating the location of the expression for error reporting, e.g. a
    /// file name and a position in the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// An optional title for the expression, i.e. a short string describing its purpose. This
    /// can be used e.g. in UIs which allow to enter the expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Expr {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            description: Some("test-description".into()),
            expression: Some("test-expression".into()),
            location: Some("test-location".into()),
            title: Some("test-title".into()),
        }
    }
}

/// A Compose request.
///
/// **GCP API**: `storage.v1.ComposeRequest`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//ComposeRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeRequest {
    /// If true, the source objects will be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_source_objects: Option<bool>,

    /// Properties of the resulting object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Object>,

    /// The kind of item this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The list of source objects that will be concatenated into a single object.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_objects: Vec<serde_json::Value>,
}

impl ComposeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            delete_source_objects: Some(false),
            destination: Some(Object::fixture()),
            kind: Some("test-kind".into()),
            source_objects: vec![],
        }
    }
}

/// A rewrite response.
///
/// **GCP API**: `storage.v1.RewriteResponse`
/// **Reference**: <https://developers.google.com/storage/docs/json_api//RewriteResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteResponse {
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property
    /// is always present in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    /// The kind of item this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The total size of the object being copied in bytes. This property is always present in
    /// the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size: Option<String>,

    /// A resource containing the metadata for the copied-to object. This property is present in
    /// the response only when copying completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Object>,

    /// A token to use in subsequent requests to continue copying data. This token is present in
    /// the response only when there is more data to copy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_token: Option<String>,

    /// The total bytes written so far, which can be used to provide a waiting user with a
    /// progress indicator. This property is always present in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_rewritten: Option<String>,
}

impl RewriteResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            done: Some(false),
            kind: Some("test-kind".into()),
            object_size: Some("test-object_size".into()),
            resource: Some(Object::fixture()),
            rewrite_token: Some("test-rewrite_token".into()),
            total_bytes_rewritten: Some("test-total_bytes_rewritten".into()),
        }
    }
}

// ======================================================================
// Inline struct types (from array/object fields)
// ======================================================================

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `storage.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketIamConfiguration {
    /// The bucket's uniform bucket-level access configuration. The feature was formerly known
    /// as Bucket Policy Only. For backward compatibility, this field will be populated with
    /// identical information as the uniformBucketLevelAccess field. We recommend using the
    /// uniformBucketLevelAccess field to enable and disable the feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy_only: Option<serde_json::Value>,

    /// The bucket's uniform bucket-level access configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_bucket_level_access: Option<serde_json::Value>,

    /// The bucket's Public Access Prevention configuration. Currently, 'inherited' and
    /// 'enforced' are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_prevention: Option<String>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `storage.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketSoftDeletePolicy {
    /// The duration in seconds that soft-deleted objects in the bucket will be retained and
    /// cannot be permanently deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_duration_seconds: Option<String>,

    /// Server-determined value that indicates the time from which the policy, or one with a
    /// greater retention, was effective. This value is in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `storage.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyBinding {
    /// The condition that is associated with this binding. NOTE: an unsatisfied condition will
    /// not allow user access via current binding. Different bindings, including their
    /// conditions, are examined independently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Expr>,

    /// A collection of identifiers for members who may assume the provided role. Recognized
    /// identifiers are as follows:
    /// - allUsers
    /// - A special identifier that represents anyone on the internet; with or without a Google
    ///   account.
    /// - allAuthenticatedUsers
    /// - A special identifier that represents anyone who is authenticated with a Google account
    ///   or a service account.
    /// - user:emailid
    /// - An email address that represents a specific account. For example, user:alice@gmail.com
    ///   or user:joe@example.com.
    /// - serviceAccount:emailid
    /// - An email address that represents a service account. For example, serviceAccount:my-
    ///   other-app@appspot.gserviceaccount.com .
    /// - group:emailid
    /// - An email address that represents a Google group. For example,
    ///   group:admins@example.com.
    /// - domain:domain
    /// - A Google Apps domain name that represents all the users of that domain. For example,
    ///   domain:google.com or domain:example.com.
    /// - projectOwner:projectid
    /// - Owners of the given project. For example, projectOwner:my-example-project
    /// - projectEditor:projectid
    /// - Editors of the given project. For example, projectEditor:my-example-project
    /// - projectViewer:projectid
    /// - Viewers of the given project. For example, projectViewer:my-example-project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,

    /// The role to which members belong. Two types of roles are supported: new IAM roles, which
    /// grant permissions that do not map directly to those provided by ACLs, and legacy IAM
    /// roles, which do map directly to ACL permissions. All roles are of the format
    /// roles/storage.specificRole. The new IAM roles are:
    /// - roles/storage.admin
    /// - Full control of Google Cloud Storage resources.
    /// - roles/storage.objectViewer
    /// - Read-Only access to Google Cloud Storage objects.
    /// - roles/storage.objectCreator
    /// - Access to create objects in Google Cloud Storage.
    /// - roles/storage.objectAdmin
    /// - Full control of Google Cloud Storage objects. The legacy IAM roles are:
    /// - roles/storage.legacyObjectReader
    /// - Read-only access to objects without listing. Equivalent to an ACL entry on an object
    ///   with the READER role.
    /// - roles/storage.legacyObjectOwner
    /// - Read/write access to existing objects without listing. Equivalent to an ACL entry on
    ///   an object with the OWNER role.
    /// - roles/storage.legacyBucketReader
    /// - Read access to buckets with object listing. Equivalent to an ACL entry on a bucket
    ///   with the READER role.
    /// - roles/storage.legacyBucketWriter
    /// - Read access to buckets with object listing/creation/deletion. Equivalent to an ACL
    ///   entry on a bucket with the WRITER role.
    /// - roles/storage.legacyBucketOwner
    /// - Read and write access to existing buckets with object listing/creation/deletion.
    ///   Equivalent to an ACL entry on a bucket with the OWNER role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
