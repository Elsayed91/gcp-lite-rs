//! Recommender API client.
//!
//! Provides access to Google Cloud Recommender recommendations.
//!
//! # Pagination
//!
//! `list_recommendations` returns paginated results. This client provides
//! three patterns:
//!
//! 1. **Single page** — `list_recommendations()` — returns one page
//! 2. **Stream** — `list_recommendations_stream()` — async stream of items
//! 3. **Collect all** — `list_recommendations_all()` — collects all pages into Vec

use crate::{
    GcpHttpClient, Result,
    ops::recommender::RecommenderOps,
    types::recommender::{ListRecommendationsResponse, Recommendation},
};

/// Options for listing recommendations.
#[derive(Debug, Clone, Default)]
pub struct ListRecommendationsOptions<'a> {
    /// Filter expression (e.g., `state_info.state=ACTIVE`).
    pub filter: Option<&'a str>,
    /// Maximum number of results per page.
    pub page_size: Option<u32>,
}

/// Client for the Recommender API.
pub struct RecommenderClient<'a> {
    ops: RecommenderOps<'a>,
}

impl<'a> RecommenderClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: RecommenderOps::new(client),
        }
    }

    // ── Recommendations ────────────────────────────────────────────────

    /// Lists recommendations for the specified recommender.
    ///
    /// Returns a single page of results. Use `list_recommendations_stream`
    /// or `list_recommendations_all` for automatic pagination.
    ///
    /// # Arguments
    ///
    /// * `project` — GCP project ID or number
    /// * `location` — Location (e.g., "us-central1", "global")
    /// * `recommender_id` — Recommender ID (e.g., "google.compute.instance.MachineTypeRecommender")
    /// * `options` — Optional filter and page size
    /// * `page_token` — Token from previous response for pagination
    pub async fn list_recommendations(
        &self,
        project: &str,
        location: &str,
        recommender_id: &str,
        options: &ListRecommendationsOptions<'_>,
        page_token: Option<&str>,
    ) -> Result<ListRecommendationsResponse> {
        let parent = format!(
            "projects/{}/locations/{}/recommenders/{}",
            project, location, recommender_id
        );
        let page_size = options.page_size.map(|n| n.to_string()).unwrap_or_default();
        self.ops
            .list_recommendations(
                &parent,
                options.filter.unwrap_or(""),
                &page_size,
                page_token.unwrap_or(""),
            )
            .await
    }

    /// Returns an async stream of recommendations, automatically handling pagination.
    ///
    /// # Arguments
    ///
    /// * `project` — GCP project ID or number
    /// * `location` — Location (e.g., "us-central1", "global")
    /// * `recommender_id` — Recommender ID (e.g., "google.compute.instance.MachineTypeRecommender")
    /// * `options` — Optional filter and page size
    pub fn list_recommendations_stream(
        &self,
        project: &str,
        location: &str,
        recommender_id: &str,
        options: ListRecommendationsOptions<'_>,
    ) -> impl futures::Stream<Item = Result<Recommendation>> + '_ {
        let parent = format!(
            "projects/{}/locations/{}/recommenders/{}",
            project, location, recommender_id
        );
        let page_size = options.page_size.map(|n| n.to_string()).unwrap_or_default();
        let filter = options.filter.unwrap_or("").to_string();

        async_stream::try_stream! {
            let mut page_token: Option<String> = None;

            loop {
                let response = self.ops
                    .list_recommendations(
                        &parent,
                        &filter,
                        &page_size,
                        page_token.as_deref().unwrap_or(""),
                    )
                    .await?;

                for rec in response.recommendations {
                    yield rec;
                }

                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Collects all recommendations across all pages into a Vec.
    ///
    /// # Warning
    ///
    /// This can return a large number of results. Consider using
    /// `list_recommendations_stream` for memory-efficient processing.
    ///
    /// # Arguments
    ///
    /// * `project` — GCP project ID or number
    /// * `location` — Location (e.g., "us-central1", "global")
    /// * `recommender_id` — Recommender ID (e.g., "google.compute.instance.MachineTypeRecommender")
    /// * `options` — Optional filter and page size
    pub async fn list_recommendations_all(
        &self,
        project: &str,
        location: &str,
        recommender_id: &str,
        options: &ListRecommendationsOptions<'_>,
    ) -> Result<Vec<Recommendation>> {
        use futures::StreamExt;

        let stream =
            self.list_recommendations_stream(project, location, recommender_id, options.clone());
        futures::pin_mut!(stream);

        let mut items = Vec::new();
        while let Some(result) = stream.next().await {
            items.push(result?);
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_recommendations_single_page() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations")
            .returning_json(json!({
                "recommendations": [
                    {
                        "name": "projects/123/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations/rec-1",
                        "description": "Save cost by changing machine type",
                        "priority": "P2",
                        "stateInfo": { "state": "ACTIVE" },
                        "primaryImpact": {
                            "category": "COST",
                            "costProjection": {
                                "cost": { "currencyCode": "USD", "units": "-10" },
                                "duration": "2592000s"
                            }
                        },
                        "etag": "\"abc123\""
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let options = super::ListRecommendationsOptions::default();
        let response = client
            .recommender()
            .list_recommendations(
                "test-project",
                "us-central1",
                "google.compute.instance.MachineTypeRecommender",
                &options,
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.recommendations.len(), 1);
        let rec = &response.recommendations[0];
        assert!(rec.name.contains("rec-1"));
        assert_eq!(
            rec.description,
            Some("Save cost by changing machine type".to_string())
        );
        assert_eq!(rec.etag, Some("\"abc123\"".to_string()));

        let state = rec.state_info.as_ref().unwrap();
        assert_eq!(
            state.state,
            Some(super::super::super::types::recommender::RecommendationState::Active)
        );

        let impact = rec.primary_impact.as_ref().unwrap();
        assert_eq!(
            impact.category,
            Some(super::super::super::types::recommender::ImpactCategory::Cost)
        );
    }

    #[tokio::test]
    async fn test_list_recommendations_with_filter() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/global/recommenders/google.iam.policy.Recommender/recommendations?filter=state_info.state%3DACTIVE")
            .returning_json(json!({
                "recommendations": [
                    {
                        "name": "projects/123/locations/global/recommenders/google.iam.policy.Recommender/recommendations/rec-iam-1",
                        "description": "Remove unused IAM role binding",
                        "stateInfo": { "state": "ACTIVE" }
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let options = super::ListRecommendationsOptions {
            filter: Some("state_info.state=ACTIVE"),
            ..Default::default()
        };
        let response = client
            .recommender()
            .list_recommendations(
                "test-project",
                "global",
                "google.iam.policy.Recommender",
                &options,
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.recommendations.len(), 1);
        assert!(response.recommendations[0].name.contains("rec-iam-1"));
    }

    #[tokio::test]
    async fn test_list_recommendations_auto_paginates() {
        let mut mock = crate::MockClient::new();

        // Page 2 (more specific URL — register first due to StartsWith matching)
        mock.expect_get("/v1/projects/test-project/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations?pageToken=page2")
            .returning_json(json!({
                "recommendations": [
                    { "name": "projects/123/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations/rec-2" }
                ]
            }))
            .times(1);

        // Page 1 (less specific — register second)
        mock.expect_get("/v1/projects/test-project/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations")
            .returning_json(json!({
                "recommendations": [
                    { "name": "projects/123/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations/rec-1" }
                ],
                "nextPageToken": "page2"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let options = super::ListRecommendationsOptions::default();
        let all = client
            .recommender()
            .list_recommendations_all(
                "test-project",
                "us-central1",
                "google.compute.instance.MachineTypeRecommender",
                &options,
            )
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        assert!(all[0].name.contains("rec-1"));
        assert!(all[1].name.contains("rec-2"));
    }

    #[tokio::test]
    async fn test_list_recommendations_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/recommenders/google.compute.instance.MachineTypeRecommender/recommendations")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let options = super::ListRecommendationsOptions::default();
        let all = client
            .recommender()
            .list_recommendations_all(
                "test-project",
                "us-central1",
                "google.compute.instance.MachineTypeRecommender",
                &options,
            )
            .await
            .unwrap();

        assert!(all.is_empty());
    }

    #[tokio::test]
    async fn test_list_recommendations_stream() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/global/recommenders/google.iam.policy.Recommender/recommendations")
            .returning_json(json!({
                "recommendations": [
                    { "name": "projects/123/locations/global/recommenders/google.iam.policy.Recommender/recommendations/r1" },
                    { "name": "projects/123/locations/global/recommenders/google.iam.policy.Recommender/recommendations/r2" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let rec = client.recommender();
        let options = super::ListRecommendationsOptions::default();
        let stream = rec.list_recommendations_stream(
            "test-project",
            "global",
            "google.iam.policy.Recommender",
            options,
        );
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(result) = stream.next().await {
            names.push(result.unwrap().name);
        }
        assert_eq!(names.len(), 2);
        assert!(names[0].contains("r1"));
        assert!(names[1].contains("r2"));
    }
}
