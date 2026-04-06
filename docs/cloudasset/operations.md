# Cloud Asset Operations

## Asset Inventory

### list_assets

**Signature**: `pub async fn list_assets(parent: &str, options: &ListAssetsOptions<'_>, page_token: Option<&str>) -> Result<ListAssetsResponse>`

Lists assets for a parent scope. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `parent` | `&str` | Parent scope: "projects/PROJECT", "folders/FOLDER", "organizations/ORG" |
| `options` | `&ListAssetsOptions` | Configuration for content type, asset types, and page size |
| `page_token` | `Option<&str>` | Token for pagination |

**Returns**: `Result<ListAssetsResponse>`

---

### list_assets_stream

**Signature**: `pub fn list_assets_stream(parent: &str, options: ListAssetsOptions<'_>) -> impl Stream<Item = Result<Asset>>`

Returns an async stream of assets, handling pagination automatically.

---

### list_assets_all

**Signature**: `pub async fn list_assets_all(parent: &str, options: &ListAssetsOptions<'_>) -> Result<Vec<Asset>>`

Collects all assets across all pages into a Vec. **Warning**: Can return a large result set.

---

## Resource Search

### search_all_resources

**Signature**: `pub async fn search_all_resources(scope: &str, options: &SearchOptions<'_>, page_token: Option<&str>) -> Result<SearchAllResourcesResponse>`

Searches all resources within a scope. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `scope` | `&str` | Search scope (project, folder, or org) |
| `options` | `&SearchOptions` | Configuration for query, asset types, and sorting |
| `page_token` | `Option<&str>` | Token for pagination |

**Returns**: `Result<SearchAllResourcesResponse>`

---

### search_all_resources_stream

**Signature**: `pub fn search_all_resources_stream(scope: &str, options: SearchOptions<'_>) -> impl Stream<Item = Result<ResourceSearchResult>>`

Returns an async stream of search results, handling pagination automatically.

---

### search_all_resources_all

**Signature**: `pub async fn search_all_resources_all(scope: &str, options: &SearchOptions<'_>) -> Result<Vec<ResourceSearchResult>>`

Collects all search results across all pages.

---

## IAM Policy Search

### search_all_iam_policies

**Signature**: `pub async fn search_all_iam_policies(scope: &str, options: &SearchIamPoliciesOptions<'_>, page_token: Option<&str>) -> Result<SearchAllIamPoliciesResponse>`

Searches all IAM policies within a scope. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `scope` | `&str` | Search scope |
| `options` | `&SearchIamPoliciesOptions` | Configuration for query and filters |
| `page_token` | `Option<&str>` | Token for pagination |

**Returns**: `Result<SearchAllIamPoliciesResponse>`

---

### search_all_iam_policies_stream

**Signature**: `pub fn search_all_iam_policies_stream(scope: &str, options: SearchIamPoliciesOptions<'_>) -> impl Stream<Item = Result<IamPolicySearchResult>>`

Returns an async stream of IAM policies, handling pagination automatically.

---

### search_all_iam_policies_all

**Signature**: `pub async fn search_all_iam_policies_all(scope: &str, options: &SearchIamPoliciesOptions<'_>) -> Result<Vec<IamPolicySearchResult>>`

Collects all IAM policies across all pages.
