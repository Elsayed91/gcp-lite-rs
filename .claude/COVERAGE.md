# Integration Test Coverage Matrix

Per-API coverage of required edge cases from `CLAUDE.md` and testing guide.

## Legend

| Symbol | Meaning |
|--------|---------|
| Y | Covered |
| - | Not applicable |
| N | Not yet covered (future work) |

## GCP Coverage Table

| API | CRUD Lifecycle | Error: Non-existent | Error: Deleted Resource | List: No Filter | List: 1 Filter | List: 2+ Filters | Non-interference | Already-done LRO | Pagination | Streaming | Special Chars |
|-----|---------------|---------------------|------------------------|-----------------|---------------|------------------|-----------------|-----------------|------------|-----------|---------------|
| **compute** (disk) | Y | Y | Y | Y | - | - | - | Y (unit) | - | Y | N |
| **compute** (address) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **compute** (router/NAT) | Y | Y | - | - | - | - | Y | - | - | - | N |
| **compute** (instance) | Y | Y | - | - | - | - | - | - | - | - | N |
| **compute** (backend svc) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **storage** (bucket) | Y | Y | - | Y | Y | - | Y | - | - | Y | N |
| **storage** (object) | Y | Y | - | Y | Y | - | - | - | - | Y | N |
| **cloudscheduler** | Y | Y | - | Y | - | - | - | - | - | - | N |
| **cloudscheduler_extra** | Y | Y | - | Y | - | - | - | - | - | - | N |
| **secretmanager** | Y | Y | Y | Y | Y | - | - | - | - | - | N |
| **sqladmin** (instance) | Y | Y | Y | Y | - | - | - | Y (unit) | - | - | N |
| **sqladmin** (database) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **sqladmin** (user) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **container** | Y | Y | - | Y | Y | - | - | Y (unit) | - | - | N |
| **gkebackup** | Y | Y | - | Y | - | - | - | Y (unit) | - | - | N |
| **monitoring** (descriptors) | Y | Y | - | Y | Y | - | - | - | - | Y | N |
| **monitoring** (time series) | Y | Y | - | - | Y | Y | - | - | Y | - | N |
| **iam** (SA) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **iam** (SA keys) | Y | - | - | Y | - | - | - | - | - | - | N |
| **cloudresourcemanager** | Y | Y | - | Y | Y | - | - | Y (unit) | - | - | N |
| **cloudasset** (list) | Y | - | - | Y | Y | Y | - | - | Y | Y | N |
| **cloudasset** (search) | Y | - | - | Y | Y | Y | - | - | - | Y | N |
| **cloudasset** (IAM search) | Y | - | - | Y | Y | Y | - | - | - | Y | N |
| **bigquery** (dataset/table) | Y | Y | - | Y | - | - | - | - | - | - | N |
| **bigquery** (jobs) | Y | - | - | Y | Y | Y | - | - | - | Y | N |
| **cloudbilling** | Y | Y | - | - | - | - | - | - | - | - | N |
| **recommender** | Y | - | - | Y | Y | - | - | - | - | Y | N |

## Edge Case Details

### Non-interference Tests

| Test | Field A | Method Under Test | Status |
|------|---------|------------------|--------|
| `compute::test_router_nat_lifecycle` | router description | `delete_nat_gateway` | Covered |
| `storage::test_public_access_prevention` | storage_class | `set_public_access_prevention` | Covered |

### Repeated Query Params (0/1/2+ values)

| API | Param | 0 values | 1 value | 2+ values |
|-----|-------|----------|---------|-----------|
| **cloudasset** list_assets | `assetTypes` | Y | Y | Y (2 + 3) |
| **cloudasset** search_all_resources | `assetTypes` | Y | Y | Y (2 + 3) |
| **cloudasset** search_all_iam_policies | `assetTypes` | Y | - | Y (2) |
| **bigquery** list_jobs | `stateFilter` | Y | Y | Y (2) |
| **monitoring** list_time_series | `groupByFields` | - | Y | - |

### Error Cases by API

| API | Non-existent GET | Deleted GET | Delete-deleted | Invalid input |
|-----|-----------------|-------------|----------------|---------------|
| **compute** | Y (disk, router) | Y (disk, instance) | - | - |
| **storage** | Y (bucket, object) | Y (bucket, object) | - | - |
| **cloudscheduler** | Y | Y | - | - |
| **secretmanager** | Y | Y | Y | - |
| **sqladmin** | Y (instance, db) | Y (instance, db, users) | - | - |
| **container** | Y | Y | - | - |
| **gkebackup** | Y | Y | - | - |
| **monitoring** | Y (descriptor, resource) | - | - | Y (filter, interval) |
| **iam** | Y (SA) | Y (SA) | - | - |
| **cloudresourcemanager** | Y (project) | - | - | - |
| **cloudbilling** | Y (project) | - | - | - |
| **bigquery** | Y (dataset) | - | - | - |

## Remaining Gaps (Future Work)

1. **Special characters in resource names** — No API tests URL encoding with spaces, slashes, or unicode. Low risk since codegen handles `urlencoding::encode`, but a single smoke test per API would increase confidence.
2. **Already-done LRO integration tests** — Covered in unit tests for all LRO APIs, but not yet tested against real GCP (e.g., stopping an already-stopped instance). The unit test coverage is sufficient for correctness; integration tests would validate GCP's actual response format.
3. **Permission denied scenarios** — Would require a separate test project with restricted IAM. Not practical for standard test runs.
