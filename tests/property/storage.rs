use gcp_lite::types::storage::*;
use proptest::prelude::*;

fn arb_optional_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), "[a-z0-9_-]{1,32}".prop_map(Some)]
}

fn arb_bucket() -> impl Strategy<Value = Bucket> {
    (
        "[a-z][a-z0-9-]{2,20}",
        arb_optional_string(),
        arb_optional_string(),
    )
        .prop_map(|(name, location, storage_class)| Bucket {
            name,
            location,
            storage_class,
            ..Default::default()
        })
}

fn arb_object() -> impl Strategy<Value = Object> {
    (
        "[a-z][a-z0-9/_.-]{1,30}",
        arb_optional_string(),
        arb_optional_string(),
    )
        .prop_map(|(name, bucket, content_type)| Object {
            name,
            bucket,
            content_type,
            ..Default::default()
        })
}

proptest! {
    #[test]
    fn bucket_serde_roundtrip(bucket in arb_bucket()) {
        let json = serde_json::to_string(&bucket).unwrap();
        let deserialized: Bucket = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(bucket.name, deserialized.name);
        prop_assert_eq!(bucket.location, deserialized.location);
        prop_assert_eq!(bucket.storage_class, deserialized.storage_class);
    }

    #[test]
    fn object_serde_roundtrip(object in arb_object()) {
        let json = serde_json::to_string(&object).unwrap();
        let deserialized: Object = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(object.name, deserialized.name);
        prop_assert_eq!(object.bucket, deserialized.bucket);
        prop_assert_eq!(object.content_type, deserialized.content_type);
    }

    #[test]
    fn policy_serde_roundtrip(version in prop::option::of(0i32..10)) {
        let policy = Policy {
            version,
            kind: Some("storage#policy".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: Policy = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(policy.version, deserialized.version);
        prop_assert_eq!(policy.kind, deserialized.kind);
    }

    #[test]
    fn compose_request_serde_roundtrip(delete in prop::option::of(prop::bool::ANY)) {
        let req = ComposeRequest {
            delete_source_objects: delete,
            kind: Some("storage#composeRequest".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ComposeRequest = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(req.delete_source_objects, deserialized.delete_source_objects);
    }

    #[test]
    fn rewrite_response_serde_roundtrip(done in prop::option::of(prop::bool::ANY)) {
        let resp = RewriteResponse {
            done,
            kind: Some("storage#rewriteResponse".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: RewriteResponse = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(resp.done, deserialized.done);
    }

    #[test]
    fn buckets_list_serde_roundtrip(count in 0usize..5) {
        let buckets = Buckets {
            items: (0..count).map(|i| Bucket {
                name: format!("bucket-{i}"),
                ..Default::default()
            }).collect(),
            next_page_token: if count > 0 { Some("tok".to_string()) } else { None },
            ..Default::default()
        };
        let json = serde_json::to_string(&buckets).unwrap();
        let deserialized: Buckets = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(buckets.items.len(), deserialized.items.len());
        prop_assert_eq!(buckets.next_page_token, deserialized.next_page_token);
    }

    #[test]
    fn objects_list_serde_roundtrip(count in 0usize..5) {
        let objects = Objects {
            items: (0..count).map(|i| Object {
                name: format!("obj-{i}"),
                ..Default::default()
            }).collect(),
            prefixes: vec!["prefix/".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&objects).unwrap();
        let deserialized: Objects = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(objects.items.len(), deserialized.items.len());
        prop_assert_eq!(objects.prefixes, deserialized.prefixes);
    }
}

#[test]
fn fixture_bucket_roundtrip() {
    let bucket = Bucket::fixture();
    let json = serde_json::to_string(&bucket).unwrap();
    let deserialized: Bucket = serde_json::from_str(&json).unwrap();
    assert_eq!(bucket.name, deserialized.name);
}

#[test]
fn fixture_object_roundtrip() {
    let object = Object::fixture();
    let json = serde_json::to_string(&object).unwrap();
    let deserialized: Object = serde_json::from_str(&json).unwrap();
    assert_eq!(object.name, deserialized.name);
}

#[test]
fn bucket_unknown_fields_ignored() {
    let json = r#"{"name":"test","unknownField":"value","anotherUnknown":42}"#;
    let bucket: Bucket = serde_json::from_str(json).unwrap();
    assert_eq!(bucket.name, "test");
}

#[test]
fn object_unknown_fields_ignored() {
    let json = r#"{"name":"test.txt","unknownField":"value"}"#;
    let object: Object = serde_json::from_str(json).unwrap();
    assert_eq!(object.name, "test.txt");
}

#[test]
fn bucket_camel_case_serialization() {
    let bucket = Bucket {
        name: "test".to_string(),
        storage_class: Some("STANDARD".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&bucket).unwrap();
    assert!(json.contains("storageClass"));
    assert!(!json.contains("storage_class"));
}

#[test]
fn empty_vecs_not_serialized() {
    let bucket = Bucket {
        name: "test".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&bucket).unwrap();
    assert!(!json.contains("\"acl\""));
    assert!(!json.contains("\"defaultObjectAcl\""));
}

#[test]
fn none_fields_not_serialized() {
    let bucket = Bucket {
        name: "test".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&bucket).unwrap();
    assert!(!json.contains("\"location\""));
    assert!(!json.contains("\"storageClass\""));
    assert!(!json.contains("\"etag\""));
}
