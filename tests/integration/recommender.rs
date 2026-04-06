//! Integration tests for Recommender API
//!
//! Run with:
//!   task test:integration:recommender

use gcp_lite::GcpHttpClient;
use std::env;

/// Test 1: Basic list that succeeds with empty results.
///
/// The Recommender API is enabled but there may be no active recommendations
/// for the test recommender. This test just verifies the API call succeeds
/// and returns a valid (possibly empty) response.
#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_recommender_list_empty() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Recommender List (Empty) Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;
    let rec = client.recommender();

    // Use a recommender that's unlikely to have active recommendations
    // in a test project: google.compute.instance.IdleResourceRecommender
    let recommender_id = "google.compute.instance.IdleResourceRecommender";

    // [1/2] List recommendations (single page)
    println!("\n[1/2] Listing recommendations (single page)...");
    let options = gcp_lite::api::recommender::ListRecommendationsOptions::default();
    let response = rec
        .list_recommendations(&project_id, "us-central1", recommender_id, &options, None)
        .await?;
    println!(
        "  Got {} recommendations (page)",
        response.recommendations.len()
    );

    // [2/2] List all recommendations (collect_all)
    println!("\n[2/2] Listing all recommendations (collect_all)...");
    let all = rec
        .list_recommendations_all(&project_id, "us-central1", recommender_id, &options)
        .await?;
    println!("  Got {} recommendations (all)", all.len());

    // Verify each recommendation has a valid name if any exist
    for r in &all {
        assert!(
            r.name.contains("recommendations/"),
            "Recommendation name should contain 'recommendations/': {}",
            r.name
        );
    }

    println!("\nRecommender list (empty) test passed!");
    Ok(())
}

/// Test 2: List recommendations against a project with actual recommendations.
///
/// Gated on RECOMMENDER_PROJECT env var. Only runs when explicitly configured
/// with a project that has active recommendations.
#[tokio::test]
#[ignore = "requires RECOMMENDER_PROJECT env var with a project that has active recommendations"]
async fn test_recommender_list_with_data() -> Result<(), Box<dyn std::error::Error>> {
    let project_id =
        env::var("RECOMMENDER_PROJECT").expect("RECOMMENDER_PROJECT must be set for this test");

    println!("\n=== Recommender List (With Data) Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;
    let rec = client.recommender();

    // Try multiple common recommenders to find one with data
    let recommender_ids = [
        "google.iam.policy.Recommender",
        "google.compute.instance.MachineTypeRecommender",
        "google.compute.instance.IdleResourceRecommender",
    ];

    let locations = ["global", "us-central1"];

    let mut found_any = false;

    for recommender_id in &recommender_ids {
        for location in &locations {
            // [1/3] List all recommendations for this recommender
            println!(
                "\n  Trying {}/{}/{}...",
                project_id, location, recommender_id
            );
            let options = gcp_lite::api::recommender::ListRecommendationsOptions {
                filter: Some("stateInfo.state=ACTIVE"),
                page_size: Some(10),
            };

            let all = rec
                .list_recommendations_all(&project_id, location, recommender_id, &options)
                .await?;

            if !all.is_empty() {
                found_any = true;
                println!("  Found {} active recommendations!", all.len());

                // [2/3] Verify recommendation structure
                let first = &all[0];
                assert!(
                    first.name.contains("recommendations/"),
                    "Name should contain 'recommendations/'"
                );
                println!("  First: {}", first.name);
                if let Some(desc) = &first.description {
                    println!("  Description: {}", desc);
                }
                if let Some(state) = &first.state_info {
                    println!("  State: {:?}", state.state);
                }
                if let Some(impact) = &first.primary_impact {
                    println!("  Impact category: {:?}", impact.category);
                }

                // [3/3] Verify stream works too
                use futures::StreamExt;
                let stream = rec.list_recommendations_stream(
                    &project_id,
                    location,
                    recommender_id,
                    gcp_lite::api::recommender::ListRecommendationsOptions {
                        filter: Some("stateInfo.state=ACTIVE"),
                        page_size: Some(5),
                    },
                );
                futures::pin_mut!(stream);
                let mut count = 0;
                while let Some(result) = stream.next().await {
                    let _ = result?;
                    count += 1;
                }
                println!("  Stream yielded {} items", count);
                assert_eq!(
                    count,
                    all.len(),
                    "Stream count should match collect_all count"
                );

                break;
            }
        }
        if found_any {
            break;
        }
    }

    if !found_any {
        println!("\n  WARNING: No active recommendations found in any recommender.");
        println!(
            "  The test passed (API calls succeeded) but couldn't verify recommendation data."
        );
    }

    println!("\nRecommender list (with data) test passed!");
    Ok(())
}
