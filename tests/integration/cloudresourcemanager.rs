//! Integration tests for Cloud Resource Manager (Projects) API.
//!
//! These tests require:
//! - `GCLOUD_PROJECT_ID` environment variable
//! - ADC credentials configured (`GOOGLE_AUTH_USE_GCLOUD=1` or service account)
//! - Sufficient permissions (`roles/resourcemanager.projectIamAdmin`, `roles/iam.serviceAccountAdmin`)
//!
//! Run standard tests with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration cloudresourcemanager -- --ignored --nocapture
//! ```
//!
//! Run project creation/deletion test (SLOW, creates real GCP project):
//! ```sh
//! TEST_PROJECT_LIFECYCLE=1 GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration cloudresourcemanager::test_project_create_delete_lifecycle -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::cloudresourcemanager::ProjectState;
use std::env;
use std::process::Command;

const TEST_SA_NAME: &str = "projects-integ-test";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

fn sa_email(name: &str, project: &str) -> String {
    format!("{}@{}.iam.gserviceaccount.com", name, project)
}

/// Delete a service account via gcloud CLI (idempotent — ignores errors).
fn gcloud_delete_sa(project: &str, email: &str) {
    let _ = Command::new("gcloud")
        .args([
            "iam",
            "service-accounts",
            "delete",
            email,
            "--project",
            project,
            "--quiet",
        ])
        .output();
}

/// IAM has eventual consistency — wait briefly after mutations.
async fn wait_for_consistency() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

// =============================================================================
// Get & Search Projects
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_get_and_search_projects() {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let projects = client.projects();

    // === 1. GET ===
    println!("\n[1/3] Getting project...");
    let proj = projects
        .get_project(&project)
        .await
        .expect("get_project failed");

    assert_eq!(proj.project_id.as_deref(), Some(project.as_str()));
    assert_eq!(proj.state, Some(ProjectState::Active));
    println!("  project_id: {:?}", proj.project_id);
    println!("  display_name: {:?}", proj.display_name);
    println!("  state: {:?}", proj.state);
    println!("  parent: {:?}", proj.parent);

    // === 2. SEARCH ===
    println!("\n[2/3] Searching for project by ID...");
    let query = format!("id:{}", project);
    let results = projects
        .search_projects(&query)
        .await
        .expect("search_projects failed");

    assert!(
        !results.projects.is_empty(),
        "search_projects('id:{project}') returned no results"
    );
    let found = results
        .projects
        .iter()
        .any(|p| p.project_id.as_deref() == Some(project.as_str()));
    assert!(found, "Our project not found in search results");
    println!("  Found {} projects matching query", results.projects.len());

    // === 3. TEST IAM PERMISSIONS ===
    println!("\n[3/4] Testing IAM permissions...");
    let resp = projects
        .test_iam_permissions(
            &project,
            vec![
                "resourcemanager.projects.get".to_string(),
                "resourcemanager.projects.getIamPolicy".to_string(),
            ],
        )
        .await
        .expect("test_iam_permissions failed");

    assert!(
        !resp.permissions.is_empty(),
        "Should have at least one permission"
    );
    println!("  Granted permissions: {:?}", resp.permissions);

    // === 4. ERROR CASES ===
    println!("\n[4/4] Testing error cases...");

    // Get non-existent project
    let result = projects.get_project("nonexistent-project-xyz-99999").await;
    assert!(result.is_err(), "Non-existent project should return error");
    println!("  Non-existent project get: error (correct)");

    // Search with empty query
    let empty_search = projects
        .search_projects("")
        .await
        .expect("Empty search query should succeed");
    println!(
        "  Empty query search: {} results (valid response)",
        empty_search.projects.len()
    );

    println!("\nAll get/search/permissions tests passed!");
}

// =============================================================================
// IAM Policy Lifecycle (read-modify-write)
// =============================================================================

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_project_iam_policy_lifecycle() {
    let project = project_id();
    let email = sa_email(TEST_SA_NAME, &project);

    // Pre-cleanup
    gcloud_delete_sa(&project, &email);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let client = GcpHttpClient::from_adc().await.expect("ADC required");

    // Create a test SA to use as the IAM member
    println!("\n[setup] Creating test service account...");
    client
        .iam()
        .create_service_account(
            &project,
            TEST_SA_NAME,
            "Projects Integration Test SA",
            "Created by integration tests — safe to delete",
        )
        .await
        .expect("Failed to create test SA");
    println!("  Created: {}", email);

    wait_for_consistency().await;

    let result = run_iam_policy_tests(&client, &project, &email).await;

    // Always cleanup
    println!("\nCleaning up...");
    gcloud_delete_sa(&project, &email);

    result.expect("IAM policy lifecycle test failed");
    println!("\nAll IAM policy tests passed!");
}

async fn run_iam_policy_tests(
    client: &GcpHttpClient,
    project: &str,
    sa_email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let projects = client.projects();
    let member = format!("serviceAccount:{}", sa_email);
    let role = "roles/viewer";

    // === 1. GET IAM POLICY ===
    println!("\n[1/7] Getting IAM policy...");
    let policy = projects.get_iam_policy(project).await?;

    assert!(!policy.bindings.is_empty(), "Policy should have bindings");
    assert!(policy.etag.is_some(), "Policy should have an etag");
    println!(
        "  {} bindings, etag: {:?}",
        policy.bindings.len(),
        policy.etag
    );

    // === 2. ADD BINDING ===
    println!("\n[2/7] Adding IAM binding ({} -> {})...", role, member);
    let updated = projects
        .add_iam_policy_binding(project, role, &member)
        .await?;

    let binding = updated
        .bindings
        .iter()
        .find(|b| b.role.as_deref() == Some(role));
    assert!(binding.is_some(), "roles/viewer binding should exist");
    assert!(
        binding.unwrap().members.contains(&member),
        "Member should be in roles/viewer binding"
    );
    println!("  Binding added successfully");

    // === 3. VERIFY PERSISTENCE ===
    println!("\n[3/7] Verifying binding persists...");
    let policy = projects.get_iam_policy(project).await?;

    let binding = policy
        .bindings
        .iter()
        .find(|b| b.role.as_deref() == Some(role));
    assert!(binding.is_some(), "roles/viewer should still exist");
    assert!(
        binding.unwrap().members.contains(&member),
        "Member should still be in binding"
    );
    println!("  Binding persisted");

    // === 4. ADD AGAIN (NO-OP) ===
    println!("\n[4/7] Adding same binding again (should be no-op)...");
    let updated = projects
        .add_iam_policy_binding(project, role, &member)
        .await?;

    let member_count = updated
        .bindings
        .iter()
        .find(|b| b.role.as_deref() == Some(role))
        .map(|b| b.members.iter().filter(|m| m.as_str() == member).count())
        .unwrap_or(0);
    assert_eq!(member_count, 1, "Member should not be duplicated");
    println!("  No duplicate — correct");

    // === 5. REMOVE BINDING ===
    println!("\n[5/7] Removing IAM binding...");
    let updated = projects
        .remove_iam_policy_binding(project, role, &member)
        .await?;

    let has_member = updated
        .bindings
        .iter()
        .find(|b| b.role.as_deref() == Some(role))
        .map(|b| b.members.contains(&member))
        .unwrap_or(false);
    assert!(!has_member, "Member should be removed from binding");
    println!("  Binding removed successfully");

    // === 6. VERIFY REMOVAL ===
    println!("\n[6/7] Verifying binding removed...");
    let policy = projects.get_iam_policy(project).await?;

    let has_member = policy
        .bindings
        .iter()
        .find(|b| b.role.as_deref() == Some(role))
        .map(|b| b.members.contains(&member))
        .unwrap_or(false);
    assert!(!has_member, "Member should not be in policy");
    println!("  Confirmed: member not in policy");

    // === 7. REMOVE AGAIN (NO-OP) ===
    println!("\n[7/7] Removing same binding again (should be no-op)...");
    let _policy = projects
        .remove_iam_policy_binding(project, role, &member)
        .await?;
    println!("  No error — correct");

    Ok(())
}

// =============================================================================
// Project Create / Delete / Undelete Lifecycle (SLOW — env var gated)
// =============================================================================

/// Delete a project via gcloud CLI (idempotent — ignores errors).
fn gcloud_delete_project(project_id: &str) {
    let _ = Command::new("gcloud")
        .args(["projects", "delete", project_id, "--quiet"])
        .output();
}

#[tokio::test]
#[ignore = "requires TEST_PROJECT_LIFECYCLE=1, GCLOUD_PROJECT_ID, and ADC — SLOW, creates real project"]
async fn test_project_create_delete_lifecycle() {
    // Gate this test behind an explicit env var since it creates real GCP projects (slow + quota)
    if env::var("TEST_PROJECT_LIFECYCLE").is_err() {
        println!("\nSkipping project lifecycle test — set TEST_PROJECT_LIFECYCLE=1 to run");
        return;
    }

    let parent_project = project_id();

    // We need an organization or folder parent to create a project under.
    // Get the parent from the test project.
    let client = GcpHttpClient::from_adc().await.expect("ADC required");
    let projects = client.projects();

    let parent_proj = projects
        .get_project(&parent_project)
        .await
        .expect("Failed to get parent project");
    let parent = parent_proj
        .parent
        .expect("Test project must have a parent (org or folder)");

    // Deterministic test project ID (max 30 chars, lowercase, hyphens)
    // Use a simple timestamp-based suffix to make it unique
    let timestamp_suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 100000;
    let test_project_id = format!("integ-test-crm-{}", timestamp_suffix);

    println!("\n=== Project Create/Delete/Undelete Lifecycle Test ===");
    println!("Test project ID: {}", test_project_id);
    println!("Parent: {}", parent);

    // Pre-cleanup in case a previous run left the project
    println!("\n[pre-cleanup] Attempting to delete any existing test project...");
    gcloud_delete_project(&test_project_id);
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let result = run_project_lifecycle_test(&client, &test_project_id, &parent).await;

    // Always cleanup — force delete via gcloud
    println!("\n[cleanup] Force deleting test project via gcloud...");
    gcloud_delete_project(&test_project_id);

    result.expect("Project lifecycle test failed");
    println!("\n=== All project lifecycle tests passed! ===");
}

async fn run_project_lifecycle_test(
    client: &GcpHttpClient,
    test_project_id: &str,
    parent: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use gcp_lite::types::Project;

    let projects = client.projects();

    // === 1. CREATE PROJECT ===
    println!("\n[1/6] Creating project (this takes 30-60 seconds)...");
    let new_project = Project {
        project_id: Some(test_project_id.to_string()),
        display_name: Some("Integration Test Project".to_string()),
        parent: Some(parent.to_string()),
        ..Default::default()
    };

    projects.create_project(&new_project).await?;
    println!("  Project created successfully");

    // Brief wait for propagation
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // === 2. GET PROJECT (verify Active) ===
    println!("\n[2/6] Getting project (should be Active)...");
    let proj = projects.get_project(test_project_id).await?;

    assert_eq!(proj.project_id.as_deref(), Some(test_project_id));
    assert_eq!(
        proj.state,
        Some(ProjectState::Active),
        "Project should be Active after creation"
    );
    println!("  State: {:?}", proj.state);

    // === 3. DELETE PROJECT ===
    println!("\n[3/6] Deleting project (marks for deletion, takes 10-30 seconds)...");
    projects.delete_project(test_project_id).await?;
    println!("  Delete operation completed");

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // === 4. GET PROJECT (verify DeleteRequested) ===
    println!("\n[4/6] Getting project (should be DeleteRequested)...");
    let proj = projects.get_project(test_project_id).await?;

    assert_eq!(
        proj.state,
        Some(ProjectState::DeleteRequested),
        "Project should be DeleteRequested after delete"
    );
    println!("  State: {:?}", proj.state);

    // === 5. UNDELETE PROJECT ===
    println!("\n[5/6] Undeleting project (restores from delete queue, takes 10-30 seconds)...");
    projects.undelete_project(test_project_id).await?;
    println!("  Undelete operation completed");

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // === 6. GET PROJECT (verify Active again) ===
    println!("\n[6/6] Getting project (should be Active after undelete)...");
    let proj = projects.get_project(test_project_id).await?;

    assert_eq!(
        proj.state,
        Some(ProjectState::Active),
        "Project should be Active after undelete"
    );
    println!("  State: {:?}", proj.state);

    // Final delete will be handled by cleanup in the test wrapper
    println!("\n[final] Marking project for deletion (final cleanup)...");
    projects.delete_project(test_project_id).await?;
    println!("  Project marked for deletion");

    Ok(())
}
