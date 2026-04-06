# Recommender Operations

## Recommendations

### list_recommendations

**Signature**: `pub async fn list_recommendations(project: &str, location: &str, recommender_id: &str, options: &ListRecommendationsOptions<'_>, page_token: Option<&str>) -> Result<ListRecommendationsResponse>`

Lists recommendations. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1", "global") |
| `recommender_id` | `&str` | Recommender ID (e.g., "google.compute.instance.MachineTypeRecommender") |
| `options` | `&ListRecommendationsOptions` | Filtering and page size options |
| `page_token` | `Option<&str>` | Pagination token |

**Returns**: `Result<ListRecommendationsResponse>`

---

### list_recommendations_stream

**Signature**: `pub fn list_recommendations_stream(project: &str, location: &str, recommender_id: &str, options: ListRecommendationsOptions<'_>) -> impl Stream<Item = Result<Recommendation>>`

Returns an async stream of recommendations, handling pagination automatically.

---

### list_recommendations_all

**Signature**: `pub async fn list_recommendations_all(project: &str, location: &str, recommender_id: &str, options: &ListRecommendationsOptions<'_>) -> Result<Vec<Recommendation>>`

Collects all recommendations across all pages into a Vec.
