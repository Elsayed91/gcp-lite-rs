//! Integration tests for Essential Contacts API.
//!
//! Needed for GCP CIS benchmark checks:
//!   - CIS 1.16 (iam_essential_contacts): list/create/delete notification contacts
//!
//! Run with:
//! ```sh
//! GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!   cargo test --test integration essentialcontacts -- --ignored --nocapture
//! ```

use gcp_lite::GcpHttpClient;
use gcp_lite::types::essentialcontacts::EssentialContact;
use std::env;

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

const TEST_EMAIL: &str = "cloud-lite-test-ralph@cloud-lite-test.invalid";

// ── Essential Contacts Lifecycle ─────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_essential_contacts_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();
    let client = GcpHttpClient::from_adc().await?;
    let ec = client.essential_contacts();

    println!("\n=== Essential Contacts Lifecycle ===");
    println!("Project: {}", project);

    // Pre-cleanup: delete any leftover test contact
    let existing = ec.list_contacts(&project).await?;
    for c in &existing {
        if c.email == TEST_EMAIL {
            let name = c.name.as_deref().unwrap_or("");
            if !name.is_empty() {
                let _ = ec.delete_contact(name).await;
                println!("[Setup] Cleaned up leftover test contact: {}", name);
            }
        }
    }

    // === 1. LIST (baseline) ===
    println!("\n[1/6] Listing essential contacts (baseline)...");
    let before = ec.list_contacts(&project).await?;
    println!("  Found {} existing contact(s)", before.len());

    // === 2. CREATE ===
    println!("\n[2/6] Creating contact for '{}'...", TEST_EMAIL);
    let contact = EssentialContact {
        email: TEST_EMAIL.to_string(),
        notification_category_subscriptions: vec!["SECURITY".to_string()],
        language_tag: Some("en".to_string()),
        ..Default::default()
    };
    let created = ec.create_contact(&project, &contact).await?;
    let contact_name = created
        .name
        .clone()
        .expect("created contact must have a name");
    assert_eq!(created.email, TEST_EMAIL, "email mismatch");
    assert!(
        created
            .notification_category_subscriptions
            .contains(&"SECURITY".to_string()),
        "notification categories mismatch"
    );
    println!("  Created: {}", contact_name);
    println!(
        "  Categories: {:?}",
        created.notification_category_subscriptions
    );

    // === 3. GET ===
    println!("\n[3/6] Getting contact '{}'...", contact_name);
    let fetched = ec.get_contact(&contact_name).await?;
    assert_eq!(fetched.email, TEST_EMAIL, "fetched email mismatch");
    assert_eq!(
        fetched.name.as_deref(),
        Some(contact_name.as_str()),
        "fetched name mismatch"
    );
    println!("  Got: {} (email={})", contact_name, fetched.email);

    // === 4. LIST (verify inclusion) ===
    println!("\n[4/6] Listing essential contacts (after create)...");
    let after = ec.list_contacts(&project).await?;
    assert!(
        after.len() >= before.len() + 1,
        "should have one more contact after create, before={} after={}",
        before.len(),
        after.len()
    );
    let found = after.iter().any(|c| c.email == TEST_EMAIL);
    assert!(found, "created contact should appear in list");
    println!("  Found {} contact(s), test contact present", after.len());

    // === 5. Verify email matches ===
    println!("\n[5/6] Verifying contact email in list...");
    let test_contact = after.iter().find(|c| c.email == TEST_EMAIL).unwrap();
    assert!(
        !test_contact.notification_category_subscriptions.is_empty(),
        "contact should have categories"
    );
    println!("  Contact email: {}", test_contact.email);
    println!(
        "  Contact categories: {:?}",
        test_contact.notification_category_subscriptions
    );

    // === 6. DELETE ===
    println!("\n[6/6] Deleting contact '{}'...", contact_name);
    ec.delete_contact(&contact_name).await?;
    println!("  Deleted: {}", contact_name);

    // Verify deletion
    let after_delete = ec.list_contacts(&project).await?;
    let still_exists = after_delete.iter().any(|c| c.email == TEST_EMAIL);
    assert!(!still_exists, "deleted contact should not appear in list");
    println!("  Confirmed: contact no longer in list");

    Ok(())
}
