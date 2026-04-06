//! Cloud SQL Admin API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::sqladmin::SqladminOps`. This layer adds:
//! - Ergonomic method signatures (project/instance/name instead of raw resource paths)
//! - Blocking variants that poll operations to completion
//! - Convenience list methods that unwrap response items

use crate::{
    GcpHttpClient, Result, SqlOperation,
    ops::sqladmin::SqladminOps,
    types::sqladmin::{
        BackupRun, Database, DatabaseInstance, InstancesCloneRequest, InstancesExportRequest,
        InstancesFailoverRequest, InstancesImportRequest, InstancesRotateServerCaRequest,
        OperationResponse, User,
    },
};

/// Client for the Cloud SQL Admin API.
pub struct SqladminClient<'a> {
    ops: SqladminOps<'a>,
}

impl<'a> SqladminClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: SqladminOps::new(client),
        }
    }

    // ── Instances ──────────────────────────────────────────────────────

    /// List all instances in a project.
    pub async fn list_instances(&self, project: &str) -> Result<Vec<DatabaseInstance>> {
        let response = self.ops.list_instances(project).await?;
        Ok(response.items)
    }

    /// Get details of a specific instance.
    pub async fn get_instance(&self, project: &str, instance: &str) -> Result<DatabaseInstance> {
        self.ops.get_instance(project, instance).await
    }

    /// Create a new Cloud SQL instance (blocks until complete).
    pub async fn create_instance(&self, project: &str, instance: &DatabaseInstance) -> Result<()> {
        let op = self.create_instance_start(project, instance).await?;
        op.wait().await
    }

    /// Start creating a new Cloud SQL instance (returns operation for manual polling).
    pub async fn create_instance_start(
        &self,
        project: &str,
        instance: &DatabaseInstance,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.create_instance(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Delete a Cloud SQL instance (blocks until complete).
    pub async fn delete_instance(&self, project: &str, instance: &str) -> Result<()> {
        let op = self.delete_instance_start(project, instance).await?;
        op.wait().await
    }

    /// Start deleting a Cloud SQL instance (returns operation for manual polling).
    pub async fn delete_instance_start(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.delete_instance(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Update a Cloud SQL instance (blocks until complete).
    pub async fn update_instance(
        &self,
        project: &str,
        instance: &str,
        body: &DatabaseInstance,
    ) -> Result<()> {
        let op = self.update_instance_start(project, instance, body).await?;
        op.wait().await
    }

    /// Start updating a Cloud SQL instance (returns operation for manual polling).
    pub async fn update_instance_start(
        &self,
        project: &str,
        instance: &str,
        body: &DatabaseInstance,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.update_instance(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Restart a Cloud SQL instance (blocks until complete).
    pub async fn restart_instance(&self, project: &str, instance: &str) -> Result<()> {
        let op = self.restart_instance_start(project, instance).await?;
        op.wait().await
    }

    /// Start restarting a Cloud SQL instance (returns operation for manual polling).
    pub async fn restart_instance_start(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.restart_instance(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    // ── Instance Actions ──────────────────────────────────────────────

    /// Clone a Cloud SQL instance (blocks until complete).
    pub async fn clone_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesCloneRequest,
    ) -> Result<()> {
        let op = self.clone_instance_start(project, instance, body).await?;
        op.wait().await
    }

    /// Start cloning a Cloud SQL instance (returns operation for manual polling).
    pub async fn clone_instance_start(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesCloneRequest,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.clone_instance(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Failover a high-availability instance (blocks until complete).
    pub async fn failover_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesFailoverRequest,
    ) -> Result<()> {
        let op = self
            .failover_instance_start(project, instance, body)
            .await?;
        op.wait().await
    }

    /// Start failing over an instance (returns operation for manual polling).
    pub async fn failover_instance_start(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesFailoverRequest,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.failover_instance(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Promote a read replica to primary (blocks until complete).
    pub async fn promote_replica(&self, project: &str, instance: &str) -> Result<()> {
        let op = self.promote_replica_start(project, instance).await?;
        op.wait().await
    }

    /// Start promoting a read replica (returns operation for manual polling).
    pub async fn promote_replica_start(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.promote_replica(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Reset SSL configuration for an instance (blocks until complete).
    pub async fn reset_ssl_config(&self, project: &str, instance: &str) -> Result<()> {
        let op = self.reset_ssl_config_start(project, instance).await?;
        op.wait().await
    }

    /// Start resetting SSL configuration (returns operation for manual polling).
    pub async fn reset_ssl_config_start(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.reset_ssl_config(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Add a new trusted CA version for an instance (blocks until complete).
    ///
    /// Must be called before [`rotate_server_ca`](Self::rotate_server_ca).
    pub async fn add_server_ca(&self, project: &str, instance: &str) -> Result<()> {
        let op = self.add_server_ca_start(project, instance).await?;
        op.wait().await
    }

    /// Start adding a new trusted CA version (returns operation for manual polling).
    pub async fn add_server_ca_start(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.add_server_ca(project, instance).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Rotate the server CA certificate for an instance (blocks until complete).
    pub async fn rotate_server_ca(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesRotateServerCaRequest,
    ) -> Result<()> {
        let op = self.rotate_server_ca_start(project, instance, body).await?;
        op.wait().await
    }

    /// Start rotating the server CA certificate (returns operation for manual polling).
    pub async fn rotate_server_ca_start(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesRotateServerCaRequest,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.rotate_server_ca(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Import data into a Cloud SQL instance (blocks until complete).
    pub async fn import_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesImportRequest,
    ) -> Result<()> {
        let op = self.import_instance_start(project, instance, body).await?;
        op.wait().await
    }

    /// Start importing data (returns operation for manual polling).
    pub async fn import_instance_start(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesImportRequest,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.import_instance(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Export data from a Cloud SQL instance (blocks until complete).
    pub async fn export_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesExportRequest,
    ) -> Result<()> {
        let op = self.export_instance_start(project, instance, body).await?;
        op.wait().await
    }

    /// Start exporting data (returns operation for manual polling).
    pub async fn export_instance_start(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesExportRequest,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.export_instance(project, instance, body).await?;
        Ok(self.sql_operation(project, &response))
    }

    // ── Backup Runs ───────────────────────────────────────────────────

    /// Create an on-demand backup (blocks until complete).
    pub async fn create_backup(
        &self,
        project: &str,
        instance: &str,
        description: Option<&str>,
    ) -> Result<()> {
        let op = self
            .create_backup_start(project, instance, description)
            .await?;
        op.wait().await
    }

    /// Start creating an on-demand backup (returns operation for manual polling).
    pub async fn create_backup_start(
        &self,
        project: &str,
        instance: &str,
        description: Option<&str>,
    ) -> Result<SqlOperation<'a>> {
        let body = BackupRun {
            description: description.map(|d| d.to_string()),
            ..Default::default()
        };
        let response = self.ops.create_backup_run(project, instance, &body).await?;
        Ok(self.sql_operation(project, &response))
    }

    // ── Databases ─────────────────────────────────────────────────────

    /// List databases in an instance.
    pub async fn list_databases(&self, project: &str, instance: &str) -> Result<Vec<Database>> {
        let response = self.ops.list_databases(project, instance).await?;
        Ok(response.items)
    }

    /// Get a specific database.
    pub async fn get_database(
        &self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> Result<Database> {
        self.ops.get_database(project, instance, database).await
    }

    /// Create a database in an instance (blocks until complete).
    pub async fn create_database(
        &self,
        project: &str,
        instance: &str,
        database: &Database,
    ) -> Result<()> {
        let op = self
            .create_database_start(project, instance, database)
            .await?;
        op.wait().await
    }

    /// Start creating a database (returns operation for manual polling).
    pub async fn create_database_start(
        &self,
        project: &str,
        instance: &str,
        database: &Database,
    ) -> Result<SqlOperation<'a>> {
        let response = self
            .ops
            .create_database(project, instance, database)
            .await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Delete a database from an instance (blocks until complete).
    pub async fn delete_database(
        &self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> Result<()> {
        let op = self
            .delete_database_start(project, instance, database)
            .await?;
        op.wait().await
    }

    /// Start deleting a database (returns operation for manual polling).
    pub async fn delete_database_start(
        &self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self
            .ops
            .delete_database(project, instance, database)
            .await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Update a database in an instance (blocks until complete).
    pub async fn update_database(
        &self,
        project: &str,
        instance: &str,
        database_name: &str,
        body: &Database,
    ) -> Result<()> {
        let op = self
            .update_database_start(project, instance, database_name, body)
            .await?;
        op.wait().await
    }

    /// Start updating a database (returns operation for manual polling).
    pub async fn update_database_start(
        &self,
        project: &str,
        instance: &str,
        database_name: &str,
        body: &Database,
    ) -> Result<SqlOperation<'a>> {
        let response = self
            .ops
            .update_database(project, instance, database_name, body)
            .await?;
        Ok(self.sql_operation(project, &response))
    }

    // ── Users ─────────────────────────────────────────────────────────

    /// List users in an instance.
    pub async fn list_users(&self, project: &str, instance: &str) -> Result<Vec<User>> {
        let response = self.ops.list_users(project, instance).await?;
        Ok(response.items)
    }

    /// Get a specific user.
    pub async fn get_user(&self, project: &str, instance: &str, name: &str) -> Result<User> {
        self.ops.get_user(project, instance, name).await
    }

    /// Create a user in an instance (blocks until complete).
    pub async fn create_user(&self, project: &str, instance: &str, user: &User) -> Result<()> {
        let op = self.create_user_start(project, instance, user).await?;
        op.wait().await
    }

    /// Start creating a user (returns operation for manual polling).
    pub async fn create_user_start(
        &self,
        project: &str,
        instance: &str,
        user: &User,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.create_user(project, instance, user).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Delete a user from an instance (blocks until complete).
    pub async fn delete_user(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> Result<()> {
        let op = self
            .delete_user_start(project, instance, name, host)
            .await?;
        op.wait().await
    }

    /// Start deleting a user (returns operation for manual polling).
    pub async fn delete_user_start(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> Result<SqlOperation<'a>> {
        let response = self.ops.delete_user(project, instance, name, host).await?;
        Ok(self.sql_operation(project, &response))
    }

    /// Update a user in an instance (blocks until complete).
    pub async fn update_user(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
        body: &User,
    ) -> Result<()> {
        let op = self
            .update_user_start(project, instance, name, host, body)
            .await?;
        op.wait().await
    }

    /// Start updating a user (returns operation for manual polling).
    pub async fn update_user_start(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
        body: &User,
    ) -> Result<SqlOperation<'a>> {
        let response = self
            .ops
            .update_user(project, instance, name, host, body)
            .await?;
        Ok(self.sql_operation(project, &response))
    }

    // ── Operations ────────────────────────────────────────────────────

    /// List operations for a project.
    pub async fn list_operations(&self, project: &str) -> Result<Vec<OperationResponse>> {
        let response = self.ops.list_operations(project).await?;
        Ok(response.items)
    }

    /// Get a specific operation.
    pub async fn get_operation(&self, project: &str, operation: &str) -> Result<OperationResponse> {
        self.ops.get_operation(project, operation).await
    }

    // ── Helpers ────────────────────────────────────────────────────────

    fn sql_operation(
        &self,
        project: &str,
        response: &crate::types::sqladmin::OperationResponse,
    ) -> SqlOperation<'a> {
        let initially_done = response.status.as_deref() == Some("DONE");
        let config = crate::PollConfig::sql_operation();
        SqlOperation::new(
            self.ops.client,
            project.to_string(),
            response.name.clone(),
            config.initial_interval(),
            config.timeout(),
            initially_done,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_support::SqladminMockHelpers;
    use serde_json::json;

    // ── Instance Tests ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_list_instances("test-project")
            .returning_json(json!({
                "items": [
                    { "name": "db-one", "databaseVersion": "POSTGRES_14", "region": "us-central1" },
                    { "name": "db-two", "databaseVersion": "MYSQL_8_0", "region": "us-east1" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.sqladmin().list_instances("test-project").await;

        assert!(result.is_ok());
        let instances = result.unwrap();
        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0].name, "db-one");
        assert_eq!(
            instances[0].database_version,
            Some("POSTGRES_14".to_string())
        );
        assert_eq!(instances[1].name, "db-two");
    }

    #[tokio::test]
    async fn test_get_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_get_instance("test-project", "my-instance")
            .returning_json(json!({
                "name": "my-instance",
                "databaseVersion": "POSTGRES_14",
                "region": "us-central1",
                "state": "RUNNABLE",
                "settings": { "tier": "db-f1-micro" }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .get_instance("test-project", "my-instance")
            .await;

        assert!(result.is_ok());
        let inst = result.unwrap();
        assert_eq!(inst.name, "my-instance");
        assert_eq!(inst.database_version, Some("POSTGRES_14".to_string()));
        assert_eq!(inst.region, Some("us-central1".to_string()));
        let settings = inst.settings.unwrap();
        assert_eq!(settings.tier, Some("db-f1-micro".to_string()));
    }

    #[tokio::test]
    async fn test_create_instance_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_instance("test-project")
            .returning_json(json!({
                "name": "op-create-123",
                "status": "PENDING",
                "operationType": "CREATE"
            }))
            .times(1);

        // Poll: pending -> done
        mock.expect_get_operation("test-project", "op-create-123")
            .returning_json(json!({
                "name": "op-create-123",
                "status": "DONE"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let sql = client.sqladmin();

        let instance = crate::types::sqladmin::DatabaseInstance {
            name: "new-instance".to_string(),
            ..Default::default()
        };
        let op = sql.create_instance_start("test-project", &instance).await;
        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    #[tokio::test]
    async fn test_create_instance_blocks_until_done() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_instance("test-project")
            .returning_json(json!({
                "name": "op-create-456",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-create-456")
            .returning_json_sequence(vec![
                json!({ "name": "op-create-456", "status": "RUNNING" }),
                json!({ "name": "op-create-456", "status": "DONE" }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let instance = crate::types::sqladmin::DatabaseInstance {
            name: "new-instance".to_string(),
            ..Default::default()
        };
        let result = client
            .sqladmin()
            .create_instance("test-project", &instance)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_instance_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete_instance("test-project", "doomed-instance")
            .returning_json(json!({
                "name": "op-delete-789",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-delete-789")
            .returning_json(json!({ "name": "op-delete-789", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .delete_instance_start("test-project", "doomed-instance")
            .await;
        assert!(op.is_ok());

        let result = op.unwrap().wait().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_restart_instance_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_restart_instance("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-restart-101",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-restart-101")
            .returning_json(json!({ "name": "op-restart-101", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .restart_instance_start("test-project", "my-instance")
            .await;
        assert!(op.is_ok());

        let result = op.unwrap().wait().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_server_ca_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_add_server_ca("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-add-ca-1",
                "status": "PENDING",
                "operationType": "UPDATE"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-add-ca-1")
            .returning_json(json!({ "name": "op-add-ca-1", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .add_server_ca_start("test-project", "my-instance")
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_rotate_server_ca_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_rotate_server_ca("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-rotate-ca-1",
                "status": "PENDING",
                "operationType": "UPDATE"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-rotate-ca-1")
            .returning_json(json!({ "name": "op-rotate-ca-1", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::sqladmin::InstancesRotateServerCaRequest::default();
        let op = client
            .sqladmin()
            .rotate_server_ca_start("test-project", "my-instance", &body)
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_rotate_server_ca_blocks_until_done() {
        let mut mock = crate::MockClient::new();

        mock.expect_rotate_server_ca("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-rotate-ca-2",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-rotate-ca-2")
            .returning_json_sequence(vec![
                json!({ "name": "op-rotate-ca-2", "status": "RUNNING" }),
                json!({ "name": "op-rotate-ca-2", "status": "DONE" }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::sqladmin::InstancesRotateServerCaRequest::default();
        let result = client
            .sqladmin()
            .rotate_server_ca("test-project", "my-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    // ── Database Tests ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_databases() {
        let mut mock = crate::MockClient::new();

        mock.expect_list_databases("test-project", "my-instance")
            .returning_json(json!({
                "items": [
                    { "name": "postgres", "charset": "UTF8" },
                    { "name": "mydb", "charset": "UTF8" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .list_databases("test-project", "my-instance")
            .await;

        assert!(result.is_ok());
        let dbs = result.unwrap();
        assert_eq!(dbs.len(), 2);
        assert_eq!(dbs[0].name, "postgres");
        assert_eq!(dbs[1].name, "mydb");
    }

    #[tokio::test]
    async fn test_get_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_get_database("test-project", "my-instance", "mydb")
            .returning_json(json!({
                "name": "mydb",
                "charset": "UTF8",
                "collation": "en_US.UTF8",
                "instance": "my-instance",
                "project": "test-project"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .get_database("test-project", "my-instance", "mydb")
            .await;

        assert!(result.is_ok());
        let db = result.unwrap();
        assert_eq!(db.name, "mydb");
        assert_eq!(db.charset, Some("UTF8".to_string()));
        assert_eq!(db.collation, Some("en_US.UTF8".to_string()));
    }

    #[tokio::test]
    async fn test_create_database_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_database("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-createdb-1",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-createdb-1")
            .returning_json(json!({ "name": "op-createdb-1", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let db = crate::types::sqladmin::Database {
            name: "newdb".to_string(),
            ..Default::default()
        };
        let op = client
            .sqladmin()
            .create_database_start("test-project", "my-instance", &db)
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_database_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete_database("test-project", "my-instance", "mydb")
            .returning_json(json!({
                "name": "op-deletedb-2",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-deletedb-2")
            .returning_json(json!({ "name": "op-deletedb-2", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .delete_database_start("test-project", "my-instance", "mydb")
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    // ── User Tests ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_users() {
        let mut mock = crate::MockClient::new();

        mock.expect_list_users("test-project", "my-instance")
            .returning_json(json!({
                "items": [
                    { "name": "postgres", "host": "" },
                    { "name": "appuser", "host": "%" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .list_users("test-project", "my-instance")
            .await;

        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "postgres");
        assert_eq!(users[1].name, "appuser");
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_get_user("test-project", "my-instance", "appuser")
            .returning_json(json!({
                "name": "appuser",
                "host": "%",
                "instance": "my-instance",
                "project": "test-project"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .get_user("test-project", "my-instance", "appuser")
            .await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "appuser");
        assert_eq!(user.host, Some("%".to_string()));
    }

    #[tokio::test]
    async fn test_create_user_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_user("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-createuser-1",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-createuser-1")
            .returning_json(json!({ "name": "op-createuser-1", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let user = crate::types::sqladmin::User {
            name: "newuser".to_string(),
            password: Some("secret".to_string()),
            ..Default::default()
        };
        let op = client
            .sqladmin()
            .create_user_start("test-project", "my-instance", &user)
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user_start() {
        let mut mock = crate::MockClient::new();

        // Empty host is stripped from query params by append_query_params
        mock.expect_delete("/v1/projects/test-project/instances/my-instance/users?name=appuser")
            .returning_json(json!({
                "name": "op-deleteuser-2",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-deleteuser-2")
            .returning_json(json!({ "name": "op-deleteuser-2", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .delete_user_start("test-project", "my-instance", "appuser", "")
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    // ── Operations Tests ────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_operations() {
        let mut mock = crate::MockClient::new();

        mock.expect_list_operations("test-project")
            .returning_json(json!({
                "items": [
                    { "name": "op-1", "status": "DONE", "operationType": "CREATE" },
                    { "name": "op-2", "status": "RUNNING", "operationType": "DELETE" }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.sqladmin().list_operations("test-project").await;

        assert!(result.is_ok());
        let ops = result.unwrap();
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].name, "op-1");
        assert_eq!(ops[0].status, Some("DONE".to_string()));
    }

    #[tokio::test]
    async fn test_get_operation() {
        let mut mock = crate::MockClient::new();

        mock.expect_get_operation("test-project", "op-123")
            .returning_json(json!({
                "name": "op-123",
                "status": "DONE",
                "operationType": "CREATE",
                "targetLink": "https://sqladmin.googleapis.com/v1/projects/test-project/instances/my-instance"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .get_operation("test-project", "op-123")
            .await;

        assert!(result.is_ok());
        let op = result.unwrap();
        assert_eq!(op.name, "op-123");
        assert_eq!(op.status, Some("DONE".to_string()));
        assert_eq!(op.operation_type, Some("CREATE".to_string()));
    }

    // ── Update Tests ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_instance_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_update_instance("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-update-inst",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-update-inst")
            .returning_json(json!({ "name": "op-update-inst", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::sqladmin::DatabaseInstance {
            name: "my-instance".to_string(),
            settings: Some(crate::types::sqladmin::Settings {
                tier: Some("db-g1-small".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let op = client
            .sqladmin()
            .update_instance_start("test-project", "my-instance", &body)
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_update_database_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_update_database("test-project", "my-instance", "mydb")
            .returning_json(json!({
                "name": "op-update-db",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-update-db")
            .returning_json(json!({ "name": "op-update-db", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::sqladmin::Database {
            name: "mydb".to_string(),
            charset: Some("UTF8".to_string()),
            ..Default::default()
        };
        let op = client
            .sqladmin()
            .update_database_start("test-project", "my-instance", "mydb", &body)
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    // ── Backup Run Tests ────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_backup_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_backup_run("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-backup-1",
                "status": "PENDING",
                "operationType": "BACKUP_VOLUME"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-backup-1")
            .returning_json(json!({ "name": "op-backup-1", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .sqladmin()
            .create_backup_start("test-project", "my-instance", Some("test backup"))
            .await;
        assert!(op.is_ok());
        assert!(op.unwrap().wait().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_backup_blocks_until_done() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_backup_run("test-project", "my-instance")
            .returning_json(json!({
                "name": "op-backup-2",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-backup-2")
            .returning_json_sequence(vec![
                json!({ "name": "op-backup-2", "status": "RUNNING" }),
                json!({ "name": "op-backup-2", "status": "DONE" }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .create_backup("test-project", "my-instance", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_backup_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_backup_run("test-project", "my-instance")
            .returning_json(json!({
                "name": "DONE_OPERATION",
                "status": "DONE"
            }))
            .times(1);

        // NO expect_get_operation — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .sqladmin()
            .create_backup("test-project", "my-instance", Some("quick backup"))
            .await;
        assert!(result.is_ok());
    }

    // ── Edge Case Tests ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_lro_operation_failure() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_instance("test-project")
            .returning_json(json!({
                "name": "op-fail",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-fail")
            .returning_json(json!({
                "name": "op-fail",
                "status": "DONE",
                "error": {
                    "errors": [{
                        "code": "QUOTA_EXCEEDED",
                        "message": "Insufficient quota"
                    }]
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let inst = crate::types::sqladmin::DatabaseInstance {
            name: "fail-instance".to_string(),
            ..Default::default()
        };
        let result = client
            .sqladmin()
            .create_instance("test-project", &inst)
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::GcpError::OperationFailed { .. }),
            "Expected OperationFailed, got: {:?}",
            err
        );
    }

    #[tokio::test]
    async fn test_lro_operation_failure_without_message() {
        let mut mock = crate::MockClient::new();

        mock.expect_create_instance("test-project")
            .returning_json(json!({
                "name": "op-fail-no-msg",
                "status": "PENDING"
            }))
            .times(1);

        mock.expect_get_operation("test-project", "op-fail-no-msg")
            .returning_json(json!({
                "name": "op-fail-no-msg",
                "status": "DONE",
                "error": {
                    "errors": [{
                        "code": "INTERNAL_ERROR"
                    }]
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let inst = crate::types::sqladmin::DatabaseInstance {
            name: "fail-instance".to_string(),
            ..Default::default()
        };
        let result = client
            .sqladmin()
            .create_instance("test-project", &inst)
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::GcpError::OperationFailed { .. }),
            "Expected OperationFailed, got: {:?}",
            err
        );
    }

    #[tokio::test]
    async fn test_list_instances_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_list_instances("test-project")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.sqladmin().list_instances("test-project").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    // ── Instance Lifecycle Test ─────────────────────────────────────

    #[tokio::test]
    async fn test_instance_lifecycle() {
        let mut mock = crate::MockClient::new();

        // Create
        mock.expect_create_instance("test-project")
            .returning_json(json!({ "name": "op-create", "status": "PENDING" }))
            .times(1);
        mock.expect_get_operation("test-project", "op-create")
            .returning_json(json!({ "name": "op-create", "status": "DONE" }))
            .times(1);

        // Get
        mock.expect_get_instance("test-project", "lifecycle-inst")
            .returning_json(json!({
                "name": "lifecycle-inst",
                "databaseVersion": "POSTGRES_14",
                "state": "RUNNABLE"
            }))
            .times(1);

        // List
        mock.expect_list_instances("test-project")
            .returning_json(json!({
                "items": [{ "name": "lifecycle-inst", "state": "RUNNABLE" }]
            }))
            .times(1);

        // Delete
        mock.expect_delete_instance("test-project", "lifecycle-inst")
            .returning_json(json!({ "name": "op-delete", "status": "PENDING" }))
            .times(1);
        mock.expect_get_operation("test-project", "op-delete")
            .returning_json(json!({ "name": "op-delete", "status": "DONE" }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let sql = client.sqladmin();

        // Create instance
        let inst = crate::types::sqladmin::DatabaseInstance {
            name: "lifecycle-inst".to_string(),
            ..Default::default()
        };
        sql.create_instance("test-project", &inst).await.unwrap();

        // Get instance
        let retrieved = sql
            .get_instance("test-project", "lifecycle-inst")
            .await
            .unwrap();
        assert_eq!(retrieved.name, "lifecycle-inst");
        assert_eq!(retrieved.database_version, Some("POSTGRES_14".to_string()));

        // List instances
        let list = sql.list_instances("test-project").await.unwrap();
        assert!(list.iter().any(|i| i.name == "lifecycle-inst"));

        // Delete instance
        sql.delete_instance("test-project", "lifecycle-inst")
            .await
            .unwrap();
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_create_instance_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        // GCP returns status:"DONE" immediately for a no-op create
        mock.expect_create_instance("test-project")
            .returning_json(json!({
                "name": "DONE_OPERATION",
                "status": "DONE"
            }))
            .times(1);

        // NO expect_get_operation — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);
        let inst = crate::types::sqladmin::DatabaseInstance {
            name: "already-exists".to_string(),
            ..Default::default()
        };

        let result = client
            .sqladmin()
            .create_instance("test-project", &inst)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_instance_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete_instance("test-project", "gone-instance")
            .returning_json(json!({
                "name": "DONE_OPERATION",
                "status": "DONE"
            }))
            .times(1);

        // NO expect_get_operation — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .sqladmin()
            .delete_instance("test-project", "gone-instance")
            .await;
        assert!(result.is_ok());
    }
}
