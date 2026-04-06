//! Integration tests for Cloud Asset API.
//!
//! These tests require:
//! - `GCLOUD_PROJECT_ID` environment variable
//! - ADC credentials configured (`GOOGLE_AUTH_USE_GCLOUD=1`)
//! - Cloud Asset API enabled in the project
//! - `cloudasset.assets.searchAllResources` and `cloudasset.assets.listAssets` permissions
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration cloudasset -- --ignored --test-threads=1 --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::api::cloudasset::{ListAssetsOptions, SearchIamPoliciesOptions, SearchOptions};
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

// =============================================================================
// List Assets Tests
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_assets() {
    let project = project_id();

    println!("\n=== Cloud Asset: List Assets Test ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    run_list_assets_tests(&client, &project)
        .await
        .expect("list assets tests failed");

    println!("\nAll list assets tests passed!");
}

async fn run_list_assets_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_asset = client.cloud_asset();
    let parent = format!("projects/{}", project);

    // === 1. List assets (single page) ===
    println!("\n[1/3] Listing assets (single page, small page size)...");
    let options = ListAssetsOptions {
        page_size: Some(10),
        content_type: Some("RESOURCE"),
        ..Default::default()
    };

    let response = cloud_asset.list_assets(&parent, &options, None).await?;
    println!("  Returned {} assets", response.assets.len());
    assert!(
        !response.assets.is_empty(),
        "Expected at least one asset in the project"
    );

    // Verify structure
    for asset in response.assets.iter().take(3) {
        let name = asset.name.as_deref().unwrap_or("<no name>");
        let asset_type = asset.asset_type.as_deref().unwrap_or("<no type>");
        println!("    - {} ({})", name, asset_type);
    }

    // === 2. List assets with pagination ===
    println!("\n[2/3] Testing pagination...");
    let options_small = ListAssetsOptions {
        page_size: Some(5),
        content_type: Some("RESOURCE"),
        ..Default::default()
    };

    let page1 = cloud_asset
        .list_assets(&parent, &options_small, None)
        .await?;
    println!("  Page 1: {} assets", page1.assets.len());

    if let Some(ref token) = page1.next_page_token {
        let page2 = cloud_asset
            .list_assets(&parent, &options_small, Some(token))
            .await?;
        println!("  Page 2: {} assets", page2.assets.len());
    } else {
        println!("  (no more pages)");
    }

    // === 3. List assets with multiple asset types (repeated params) ===
    println!("\n[3/5] Listing assets filtered by 2+ asset types (repeated params edge case)...");
    let asset_types = [
        "compute.googleapis.com/Instance",
        "compute.googleapis.com/Disk",
    ];
    let options_types = ListAssetsOptions {
        page_size: Some(20),
        content_type: Some("RESOURCE"),
        asset_types: Some(&asset_types),
        ..Default::default()
    };

    let response_types = cloud_asset
        .list_assets(&parent, &options_types, None)
        .await?;
    println!(
        "  Returned {} assets with 2 asset type filters",
        response_types.assets.len()
    );

    // Every returned asset must match one of our filter types
    for asset in &response_types.assets {
        let at = asset.asset_type.as_deref().unwrap_or("");
        assert!(
            at == "compute.googleapis.com/Instance" || at == "compute.googleapis.com/Disk",
            "Asset type '{}' should be Instance or Disk",
            at
        );
        println!("    - {} ({})", asset.name.as_deref().unwrap_or("?"), at);
    }

    // === 4. List assets with single asset type (should also work) ===
    println!("\n[4/5] Listing assets filtered by 1 asset type...");
    let single_type = ["compute.googleapis.com/Instance"];
    let options_single = ListAssetsOptions {
        page_size: Some(10),
        content_type: Some("RESOURCE"),
        asset_types: Some(&single_type),
        ..Default::default()
    };

    let response_single = cloud_asset
        .list_assets(&parent, &options_single, None)
        .await?;
    println!(
        "  Returned {} assets with 1 asset type filter",
        response_single.assets.len()
    );
    for asset in &response_single.assets {
        assert_eq!(
            asset.asset_type.as_deref(),
            Some("compute.googleapis.com/Instance"),
            "All assets should be Instance type"
        );
    }

    // === 5. List assets all ===
    println!("\n[5/5] Collecting all assets (with limit)...");
    let options_limited = ListAssetsOptions {
        page_size: Some(50),
        content_type: Some("RESOURCE"),
        ..Default::default()
    };

    // Use stream to count without collecting everything in memory
    use futures::StreamExt;
    let stream = cloud_asset.list_assets_stream(&parent, options_limited);
    futures::pin_mut!(stream);

    let mut count = 0;
    let limit = 100; // Don't fetch more than 100 for testing
    while let Some(result) = stream.next().await {
        let _asset = result?;
        count += 1;
        if count >= limit {
            println!("  Stopping at {} assets (limit reached)", count);
            break;
        }
    }

    if count < limit {
        println!("  Total assets found: {}", count);
    }

    Ok(())
}

// =============================================================================
// Search All Resources Tests
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_search_all_resources() {
    let project = project_id();

    println!("\n=== Cloud Asset: Search All Resources Test ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    run_search_tests(&client, &project)
        .await
        .expect("search tests failed");

    println!("\nAll search tests passed!");
}

async fn run_search_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_asset = client.cloud_asset();
    let scope = format!("projects/{}", project);

    // === 1. Basic search (no filters) ===
    println!("\n[1/4] Searching all resources (no filter)...");
    let options = SearchOptions {
        page_size: Some(20),
        ..Default::default()
    };

    let response = cloud_asset
        .search_all_resources(&scope, &options, None)
        .await?;
    println!("  Returned {} results", response.results.len());
    assert!(
        !response.results.is_empty(),
        "Expected at least one resource in the project"
    );

    // Show sample results
    for result in response.results.iter().take(5) {
        let name = result.name.as_deref().unwrap_or("<no name>");
        let asset_type = result.asset_type.as_deref().unwrap_or("<no type>");
        let location = result.location.as_deref().unwrap_or("global");
        println!("    - {} ({}) @ {}", name, asset_type, location);
    }

    // === 2. Search with query ===
    println!("\n[2/4] Searching with query (state:ACTIVE)...");
    let options_query = SearchOptions {
        query: Some("state:ACTIVE"),
        page_size: Some(10),
        ..Default::default()
    };

    let response_query = cloud_asset
        .search_all_resources(&scope, &options_query, None)
        .await?;
    println!("  Returned {} results", response_query.results.len());

    // === 3. Search with single asset type filter ===
    println!("\n[3/8] Searching for compute instances (single type)...");
    let asset_types = ["compute.googleapis.com/Instance"];
    let options_type = SearchOptions {
        asset_types: Some(&asset_types),
        page_size: Some(10),
        ..Default::default()
    };

    let response_type = cloud_asset
        .search_all_resources(&scope, &options_type, None)
        .await?;
    println!("  Found {} compute instances", response_type.results.len());

    for result in response_type.results.iter().take(3) {
        println!("    - {}", result.name.as_deref().unwrap_or("<no name>"));
    }

    // === 4. Search with 2 asset types (repeated params) ===
    println!("\n[4/8] Searching with 2 asset types (repeated params)...");
    let multi_types = [
        "compute.googleapis.com/Instance",
        "compute.googleapis.com/Disk",
    ];
    let options_multi = SearchOptions {
        asset_types: Some(&multi_types),
        page_size: Some(20),
        ..Default::default()
    };

    let response_multi = cloud_asset
        .search_all_resources(&scope, &options_multi, None)
        .await?;
    println!(
        "  Found {} results with 2 asset type filters",
        response_multi.results.len()
    );

    // Verify returned types match our filter
    for result in &response_multi.results {
        let at = result.asset_type.as_deref().unwrap_or("");
        assert!(
            at == "compute.googleapis.com/Instance" || at == "compute.googleapis.com/Disk",
            "Asset type '{}' should be Instance or Disk",
            at
        );
    }
    for result in response_multi.results.iter().take(5) {
        println!(
            "    - {} ({})",
            result.name.as_deref().unwrap_or("?"),
            result.asset_type.as_deref().unwrap_or("?")
        );
    }

    // === 5. Search with 3+ asset types (stress test repeated params) ===
    println!("\n[5/8] Searching with 3 asset types (repeated params stress)...");
    let triple_types = [
        "compute.googleapis.com/Instance",
        "compute.googleapis.com/Disk",
        "compute.googleapis.com/Network",
    ];
    let options_triple = SearchOptions {
        asset_types: Some(&triple_types),
        page_size: Some(20),
        ..Default::default()
    };

    let response_triple = cloud_asset
        .search_all_resources(&scope, &options_triple, None)
        .await?;
    println!(
        "  Found {} results with 3 asset type filters",
        response_triple.results.len()
    );

    for result in &response_triple.results {
        let at = result.asset_type.as_deref().unwrap_or("");
        assert!(
            at == "compute.googleapis.com/Instance"
                || at == "compute.googleapis.com/Disk"
                || at == "compute.googleapis.com/Network",
            "Asset type '{}' should be Instance, Disk, or Network",
            at
        );
    }

    // === 6. Search with order_by ===
    println!("\n[6/8] Searching with order_by (createTime desc)...");
    let options_ordered = SearchOptions {
        page_size: Some(10),
        order_by: Some("createTime desc"),
        ..Default::default()
    };

    let response_ordered = cloud_asset
        .search_all_resources(&scope, &options_ordered, None)
        .await?;
    println!(
        "  Found {} results with order_by",
        response_ordered.results.len()
    );
    assert!(
        !response_ordered.results.is_empty(),
        "order_by search should return results"
    );

    // === 7. Search with read_mask (partial fields) ===
    println!("\n[7/8] Searching with read_mask (name,assetType only)...");
    let options_mask = SearchOptions {
        page_size: Some(5),
        read_mask: Some("name,assetType"),
        ..Default::default()
    };

    let response_mask = cloud_asset
        .search_all_resources(&scope, &options_mask, None)
        .await?;
    println!(
        "  Found {} results with read_mask",
        response_mask.results.len()
    );
    assert!(
        !response_mask.results.is_empty(),
        "read_mask search should return results"
    );
    // With read_mask limiting to name+assetType, location should be absent
    for result in response_mask.results.iter().take(3) {
        println!(
            "    - {} ({}) location={:?}",
            result.name.as_deref().unwrap_or("?"),
            result.asset_type.as_deref().unwrap_or("?"),
            result.location
        );
    }

    // === 8. Search all with stream ===
    println!("\n[8/8] Streaming search results (with limit)...");
    use futures::StreamExt;

    let stream_options = SearchOptions {
        page_size: Some(50),
        ..Default::default()
    };

    let stream = cloud_asset.search_all_resources_stream(&scope, stream_options);
    futures::pin_mut!(stream);

    let mut count = 0;
    let limit = 100;
    while let Some(result) = stream.next().await {
        let _resource = result?;
        count += 1;
        if count >= limit {
            println!("  Stopping at {} results (limit reached)", count);
            break;
        }
    }

    if count < limit {
        println!("  Total results found: {}", count);
    }

    Ok(())
}

// =============================================================================
// IAM Policy Tests
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_list_assets_iam_policy() {
    let project = project_id();

    println!("\n=== Cloud Asset: List Assets with IAM Policy ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    run_list_assets_iam_policy_tests(&client, &project)
        .await
        .expect("IAM policy list tests failed");

    println!("\nAll IAM policy list tests passed!");
}

async fn run_list_assets_iam_policy_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_asset = client.cloud_asset();
    let parent = format!("projects/{}", project);

    // === 1. List assets with IAM_POLICY content type ===
    println!("\n[1/2] Listing assets with IAM_POLICY content type...");
    let options = ListAssetsOptions {
        page_size: Some(10),
        content_type: Some("IAM_POLICY"),
        ..Default::default()
    };

    let response = cloud_asset.list_assets(&parent, &options, None).await?;
    println!("  Returned {} assets", response.assets.len());
    assert!(
        !response.assets.is_empty(),
        "Expected at least one asset with IAM policy in the project"
    );

    // Verify IAM policy is populated
    let mut found_policy = false;
    for asset in response.assets.iter().take(5) {
        let name = asset.name.as_deref().unwrap_or("<no name>");
        let asset_type = asset.asset_type.as_deref().unwrap_or("<no type>");
        let has_policy = asset.iam_policy.is_some();
        let binding_count = asset
            .iam_policy
            .as_ref()
            .map(|p| p.bindings.len())
            .unwrap_or(0);

        println!(
            "    - {} ({}) — policy: {}, bindings: {}",
            name, asset_type, has_policy, binding_count
        );

        if let Some(ref policy) = asset.iam_policy {
            found_policy = true;
            for binding in policy.bindings.iter().take(3) {
                println!(
                    "      role: {}, members: {}",
                    binding.role.as_deref().unwrap_or("<no role>"),
                    binding.members.len()
                );
            }
        }
    }

    assert!(
        found_policy,
        "Expected at least one asset with a non-empty IAM policy"
    );

    // === 2. Stream IAM policy assets ===
    println!("\n[2/2] Streaming IAM policy assets (with limit)...");
    use futures::StreamExt;

    let stream_options = ListAssetsOptions {
        page_size: Some(50),
        content_type: Some("IAM_POLICY"),
        ..Default::default()
    };

    let stream = cloud_asset.list_assets_stream(&parent, stream_options);
    futures::pin_mut!(stream);

    let mut count = 0;
    let mut with_policy = 0;
    let limit = 50;
    while let Some(result) = stream.next().await {
        let asset = result?;
        count += 1;
        if asset.iam_policy.is_some() {
            with_policy += 1;
        }
        if count >= limit {
            break;
        }
    }

    println!(
        "  Processed {} assets, {} with IAM policies",
        count, with_policy
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_search_all_iam_policies() {
    let project = project_id();

    println!("\n=== Cloud Asset: Search All IAM Policies ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    run_search_iam_policies_tests(&client, &project)
        .await
        .expect("IAM policy search tests failed");

    println!("\nAll IAM policy search tests passed!");
}

async fn run_search_iam_policies_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cloud_asset = client.cloud_asset();
    let scope = format!("projects/{}", project);

    // === 1. Basic IAM policy search ===
    println!("\n[1/3] Searching all IAM policies (no filter)...");
    let options = SearchIamPoliciesOptions {
        page_size: Some(20),
        ..Default::default()
    };

    let response = cloud_asset
        .search_all_iam_policies(&scope, &options, None)
        .await?;
    println!("  Returned {} results", response.results.len());
    assert!(
        !response.results.is_empty(),
        "Expected at least one IAM policy in the project"
    );

    for result in response.results.iter().take(5) {
        let resource = result.resource.as_deref().unwrap_or("<no resource>");
        let asset_type = result.asset_type.as_deref().unwrap_or("<no type>");
        let binding_count = result
            .policy
            .as_ref()
            .map(|p| p.bindings.len())
            .unwrap_or(0);
        println!(
            "    - {} ({}) — {} bindings",
            resource, asset_type, binding_count
        );
    }

    // === 2. Search with query ===
    println!("\n[2/3] Searching IAM policies with query (policy:roles/editor)...");
    let query_options = SearchIamPoliciesOptions {
        query: Some("policy:roles/editor"),
        page_size: Some(10),
        ..Default::default()
    };

    let query_response = cloud_asset
        .search_all_iam_policies(&scope, &query_options, None)
        .await?;
    println!("  Found {} results", query_response.results.len());

    // === 3. Search with 2 asset types (repeated params) ===
    println!("\n[3/5] Searching IAM policies with 2 asset types (repeated params)...");
    let multi_types = [
        "cloudresourcemanager.googleapis.com/Project",
        "storage.googleapis.com/Bucket",
    ];
    let multi_type_options = SearchIamPoliciesOptions {
        asset_types: Some(&multi_types),
        page_size: Some(20),
        ..Default::default()
    };

    let multi_response = cloud_asset
        .search_all_iam_policies(&scope, &multi_type_options, None)
        .await?;
    println!(
        "  Found {} results with 2 asset type filters",
        multi_response.results.len()
    );

    // Verify returned types match our filter
    for result in &multi_response.results {
        let at = result.asset_type.as_deref().unwrap_or("");
        assert!(
            at == "cloudresourcemanager.googleapis.com/Project"
                || at == "storage.googleapis.com/Bucket",
            "Asset type '{}' should be Project or Bucket",
            at
        );
    }
    for result in multi_response.results.iter().take(5) {
        println!(
            "    - {} ({})",
            result.resource.as_deref().unwrap_or("?"),
            result.asset_type.as_deref().unwrap_or("?")
        );
    }

    // === 4. Search with order_by ===
    println!("\n[4/5] Searching IAM policies with order_by (resource desc)...");
    let ordered_options = SearchIamPoliciesOptions {
        page_size: Some(10),
        order_by: Some("resource desc"),
        ..Default::default()
    };

    let ordered_response = cloud_asset
        .search_all_iam_policies(&scope, &ordered_options, None)
        .await?;
    println!(
        "  Found {} results with order_by",
        ordered_response.results.len()
    );
    assert!(
        !ordered_response.results.is_empty(),
        "order_by search should return results"
    );

    // === 5. Stream search results ===
    println!("\n[5/5] Streaming IAM policy search results (with limit)...");
    use futures::StreamExt;

    let stream_options = SearchIamPoliciesOptions {
        page_size: Some(50),
        ..Default::default()
    };

    let stream = cloud_asset.search_all_iam_policies_stream(&scope, stream_options);
    futures::pin_mut!(stream);

    let mut count = 0;
    let limit = 100;
    while let Some(result) = stream.next().await {
        let _policy_result = result?;
        count += 1;
        if count >= limit {
            println!("  Stopping at {} results (limit reached)", count);
            break;
        }
    }

    if count < limit {
        println!("  Total results found: {}", count);
    }

    Ok(())
}
