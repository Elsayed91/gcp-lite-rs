//! Integration tests for Cloud SQL Admin API
//!
//! Run with:
//!   task test:integration:sqladmin
//!
//! Note: Cloud SQL does not allow reusing instance names for ~1 week after deletion.
//! All operation groups share a single instance to minimize cost and time.

use gcp_lite::GcpHttpClient;
use gcp_lite::types::sqladmin::{
    Database, DatabaseInstance, InstanceState, InstancesRotateServerCaRequest, Settings,
    SqlBackendType, SqlInstanceType, User,
};
use std::env;
use std::process::Command;
use std::time::Duration;

const INSTANCE_NAME: &str = "sql-integ-v2";
const TEST_DB_NAME: &str = "testdb";
const TEST_USER_NAME: &str = "testuser";
const TEST_LOCATION: &str = "us-central1";

fn project_id() -> String {
    env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set")
}

fn gcloud_delete_instance(project_id: &str, name: &str) {
    let _ = Command::new("gcloud")
        .args([
            "sql",
            "instances",
            "delete",
            name,
            "--project",
            project_id,
            "--quiet",
        ])
        .output();
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_sqladmin_full_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project = project_id();

    println!("\n=== Cloud SQL Full Lifecycle Test ===");
    println!("Project: {}", project);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    println!("\n[0/19] Pre-cleanup...");
    gcloud_delete_instance(&project, INSTANCE_NAME);
    tokio::time::sleep(Duration::from_secs(5)).await;

    let result = run_all_tests(&client, &project).await;

    // Always cleanup
    println!("\n[cleanup] Deleting instance via gcloud...");
    gcloud_delete_instance(&project, INSTANCE_NAME);

    result?;
    println!("\nAll Cloud SQL lifecycle tests passed!");
    Ok(())
}

async fn run_all_tests(
    client: &GcpHttpClient,
    project: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let sql = client.sqladmin();

    // ── Group 1: Instance Lifecycle ─────────────────────────────────

    // [1/19] CREATE INSTANCE (skip if already RUNNABLE to handle previous test failures)
    println!("\n[1/19] Ensuring instance exists: {}...", INSTANCE_NAME);
    let needs_create = match sql.get_instance(project, INSTANCE_NAME).await {
        Ok(inst) if inst.state == Some(InstanceState::Runnable) => {
            println!("  Using existing RUNNABLE instance");
            false
        }
        _ => true,
    };
    if needs_create {
        println!("  Creating instance (this takes ~5 min)...");
        let instance = DatabaseInstance {
            name: INSTANCE_NAME.to_string(),
            project: Some(project.to_string()),
            settings: Some(Settings {
                tier: Some("db-f1-micro".to_string()),
                ..Default::default()
            }),
            instance_type: Some(SqlInstanceType::CloudSqlInstance),
            backend_type: Some(SqlBackendType::SecondGen),
            database_version: Some("POSTGRES_14".to_string()),
            region: Some(TEST_LOCATION.to_string()),
            ..Default::default()
        };
        sql.create_instance(project, &instance).await?;
        println!("  Instance created");
    }

    // [2/19] GET INSTANCE
    println!("\n[2/19] Getting instance...");
    let retrieved = sql.get_instance(project, INSTANCE_NAME).await?;
    assert_eq!(retrieved.name, INSTANCE_NAME);
    assert_eq!(retrieved.database_version, Some("POSTGRES_14".to_string()));
    assert_eq!(retrieved.region, Some(TEST_LOCATION.to_string()));
    println!(
        "  Verified: name={}, version={:?}, region={:?}",
        retrieved.name, retrieved.database_version, retrieved.region,
    );

    // [3/19] LIST INSTANCES
    println!("\n[3/19] Listing instances...");
    let list = sql.list_instances(project).await?;
    assert!(
        list.iter().any(|i| i.name == INSTANCE_NAME),
        "Instance should appear in list"
    );
    println!("  Found instance in list ({} total)", list.len());

    // ── Group 2: Database CRUD ──────────────────────────────────────
    // Note: update_database is skipped because PostgreSQL does not allow
    // changing charset/collation after creation. The API method is covered
    // by unit tests.

    // [4/19] CREATE DATABASE
    println!("\n[4/19] Creating database: {}...", TEST_DB_NAME);
    let db = Database {
        name: TEST_DB_NAME.to_string(),
        project: Some(project.to_string()),
        instance: Some(INSTANCE_NAME.to_string()),
        ..Default::default()
    };
    sql.create_database(project, INSTANCE_NAME, &db).await?;
    println!("  Database created");

    // [5/19] GET DATABASE
    println!("\n[5/19] Getting database...");
    let retrieved_db = sql
        .get_database(project, INSTANCE_NAME, TEST_DB_NAME)
        .await?;
    assert_eq!(retrieved_db.name, TEST_DB_NAME);
    println!("  Verified: name={}", retrieved_db.name);

    // [6/19] LIST DATABASES
    println!("\n[6/19] Listing databases...");
    let dbs = sql.list_databases(project, INSTANCE_NAME).await?;
    assert!(
        dbs.iter().any(|d| d.name == TEST_DB_NAME),
        "Database should appear in list"
    );
    println!("  Found database in list ({} total)", dbs.len());

    // [7/19] DELETE DATABASE
    println!("\n[7/19] Deleting database...");
    sql.delete_database(project, INSTANCE_NAME, TEST_DB_NAME)
        .await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let db_result = sql.get_database(project, INSTANCE_NAME, TEST_DB_NAME).await;
    assert!(db_result.is_err(), "Deleted database should return error");
    println!("  Verified: database deleted");

    // ── Group 3: User CRUD ──────────────────────────────────────────

    // [8/19] CREATE USER
    println!("\n[8/19] Creating user: {}...", TEST_USER_NAME);
    let user = User {
        name: TEST_USER_NAME.to_string(),
        instance: Some(INSTANCE_NAME.to_string()),
        password: Some("test-password-123".to_string()),
        ..Default::default()
    };
    sql.create_user(project, INSTANCE_NAME, &user).await?;
    println!("  User created");

    // [9/19] LIST USERS
    println!("\n[9/19] Listing users...");
    let users = sql.list_users(project, INSTANCE_NAME).await?;
    assert!(
        users.iter().any(|u| u.name == TEST_USER_NAME),
        "User should appear in list"
    );
    println!("  Found user in list ({} total)", users.len());

    // [10/19] GET USER
    println!("\n[10/19] Getting user...");
    let retrieved_user = sql.get_user(project, INSTANCE_NAME, TEST_USER_NAME).await?;
    assert_eq!(retrieved_user.name, TEST_USER_NAME);
    println!("  Verified: name={}", retrieved_user.name);

    // [11/19] UPDATE USER
    println!("\n[11/19] Updating user...");
    let update_user_body = User {
        name: TEST_USER_NAME.to_string(),
        password: Some("updated-password-456".to_string()),
        ..Default::default()
    };
    sql.update_user(
        project,
        INSTANCE_NAME,
        TEST_USER_NAME,
        "",
        &update_user_body,
    )
    .await?;
    println!("  User updated");

    // [12/19] DELETE USER
    println!("\n[12/19] Deleting user...");
    sql.delete_user(project, INSTANCE_NAME, TEST_USER_NAME, "")
        .await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let users_after = sql.list_users(project, INSTANCE_NAME).await?;
    assert!(
        !users_after.iter().any(|u| u.name == TEST_USER_NAME),
        "Deleted user should not appear in list"
    );
    println!("  Verified: user deleted");

    // ── Group 4: Backup Runs ────────────────────────────────────────

    // [13/19] CREATE ON-DEMAND BACKUP (blocking)
    println!("\n[13/19] Creating on-demand backup (this may take a few minutes)...");
    sql.create_backup(project, INSTANCE_NAME, Some("integration-test-backup"))
        .await?;
    println!("  Backup completed successfully");

    // ── Group 5: SSL CA Rotation ─────────────────────────────────────

    // [14/19] ADD SERVER CA (prerequisite for rotateServerCa)
    println!("\n[14/19] Staging new server CA...");
    sql.add_server_ca(project, INSTANCE_NAME).await?;
    println!("  New server CA staged");

    // [15/19] ROTATE SERVER CA
    println!("\n[15/19] Rotating server CA...");
    let rotate_body = InstancesRotateServerCaRequest::default();
    sql.rotate_server_ca(project, INSTANCE_NAME, &rotate_body)
        .await?;
    println!("  Server CA rotated successfully");

    // ── Group 6: Instance Actions + Cleanup ─────────────────────────

    // [16/19] RESTART + DELETE
    println!("\n[16/19] Restarting instance...");
    sql.restart_instance(project, INSTANCE_NAME).await?;
    println!("  Instance restarted");

    println!("  Deleting instance...");
    sql.delete_instance(project, INSTANCE_NAME).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let inst_result = sql.get_instance(project, INSTANCE_NAME).await;
    assert!(
        inst_result.is_err(),
        "Deleted instance should return error on get"
    );
    println!("  Verified: instance deleted");

    // ── Error Cases ───────────────────────────────────────────────────

    // [17/19] ERROR: Get non-existent instance
    println!("\n[17/19] Getting non-existent instance (expect error)...");
    let result = sql
        .get_instance(project, "nonexistent-instance-xyz-99999")
        .await;
    assert!(result.is_err(), "Non-existent instance should return error");
    println!("  Non-existent instance: error (correct)");

    // [18/19] ERROR: Get non-existent database on deleted instance
    println!("\n[18/19] Getting database on deleted instance (expect error)...");
    let result = sql
        .get_database(project, INSTANCE_NAME, "nonexistent-db")
        .await;
    assert!(
        result.is_err(),
        "Database on deleted instance should return error"
    );
    println!("  Deleted instance database: error (correct)");

    // [19/19] ERROR: List users on deleted instance
    println!("\n[19/19] Listing users on deleted instance (expect error)...");
    let result = sql.list_users(project, INSTANCE_NAME).await;
    assert!(
        result.is_err(),
        "List users on deleted instance should return error"
    );
    println!("  Deleted instance list users: error (correct)");

    Ok(())
}
