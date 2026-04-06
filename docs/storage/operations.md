# Cloud Storage Operations

## Buckets

### create_bucket

**Signature**: `pub async fn create_bucket(project: &str, body: &Bucket) -> Result<Bucket>`

Create a new bucket.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `body` | `&Bucket` | Bucket configuration (name, location, etc.) |

**Returns**: `Result<Bucket>`

---

### get_bucket

**Signature**: `pub async fn get_bucket(bucket: &str) -> Result<Bucket>`

Get bucket metadata.

---

### list_buckets

**Signature**: `pub async fn list_buckets(project: &str, prefix: Option<&str>, page_token: Option<&str>) -> Result<Buckets>`

List buckets in a project.

---

### delete_bucket

**Signature**: `pub async fn delete_bucket(bucket: &str) -> Result<()>`

Delete an empty bucket.

---

### patch_bucket

**Signature**: `pub async fn patch_bucket(bucket: &str, body: &Bucket) -> Result<Bucket>`

Update bucket metadata (e.g., storage class, labels).

---

### set_public_access_prevention

**Signature**: `pub async fn set_public_access_prevention(bucket: &str, enforced: bool) -> Result<Bucket>`

Helper to enable/disable public access prevention.

---

## Objects

### create_object

**Signature**: `pub async fn create_object(bucket: &str, body: &Object) -> Result<Object>`

Store a new object (metadata only - see note below).

**Note**: This client currently supports metadata-only uploads. For file content uploads, use signed URLs or the upload API directly.

---

### get_object

**Signature**: `pub async fn get_object(bucket: &str, object: &str) -> Result<Object>`

Get object metadata.

---

### list_objects

**Signature**: `pub async fn list_objects(bucket: &str, prefix: Option<&str>, page_token: Option<&str>) -> Result<Objects>`

List objects in a bucket.

---

### delete_object

**Signature**: `pub async fn delete_object(bucket: &str, object: &str) -> Result<()>`

Delete an object.

---

### copy_object

**Signature**: `pub async fn copy_object(source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str, body: &Object) -> Result<Object>`

Copy an object within the same location/storage class.

---

### rewrite_object

**Signature**: `pub async fn rewrite_object(...) -> Result<RewriteResponse>`

Rewrite an object (supports cross-location/class copies). Requires a loop if `done` is false.
