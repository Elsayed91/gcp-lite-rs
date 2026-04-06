//! MockClient helpers for Cloud SQL Admin API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud SQL Admin API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SqladminMockHelpers {
    /// Helper to expect `list_instances`: Lists instances under a given project.
    fn expect_list_instances(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_instance`: Retrieves a resource containing information about a Cloud
    /// SQL instance.
    fn expect_get_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_instance`: Creates a new Cloud SQL instance.
    fn expect_create_instance(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_instance`: Deletes a Cloud SQL instance.
    fn expect_delete_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_instance`: Partially updates settings of a Cloud SQL instance by
    /// merging the request with the current configuration. This method supports patch semantics.
    fn expect_update_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `restart_instance`: Restarts a Cloud SQL instance.
    fn expect_restart_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `clone_instance`: Creates a Cloud SQL instance as a clone of the source
    /// instance. Using this operation might cause your instance to restart.
    fn expect_clone_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `failover_instance`: Initiates a manual failover of a high availability
    /// (HA) primary instance to a standby instance, which becomes the primary instance. Users are
    /// then rerouted to the new primary. For more information, see the [Overview of high
    /// availability](https://cloud.google.com/sql/docs/mysql/high-availability) page in the Cloud
    /// SQL documentation. If using Legacy HA (MySQL only), this causes the instance to failover to
    /// its failover replica instance.
    fn expect_failover_instance(&mut self, project: &str, instance: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `promote_replica`: Promotes the read replica instance to be an independent
    /// Cloud SQL primary instance. Using this operation might cause your instance to restart.
    fn expect_promote_replica(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `reset_ssl_config`: Deletes all client certificates and generates a new
    /// server SSL certificate for the instance.
    fn expect_reset_ssl_config(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `add_server_ca`: Adds a new trusted Certificate Authority (CA) version for
    /// the specified instance. Required to prepare for a certificate rotation. If a CA version was
    /// previously added but never used in a certificate rotation, this operation replaces that
    /// version. There cannot be more than one CA version waiting to be rotated in. For instances
    /// that have enabled Certificate Authority Service (CAS) based server CA, use
    /// AddServerCertificate to add a new server certificate.
    fn expect_add_server_ca(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `rotate_server_ca`: Rotates the server certificate to one signed by the
    /// Certificate Authority (CA) version previously added with the addServerCA method. For
    /// instances that have enabled Certificate Authority Service (CAS) based server CA, use
    /// RotateServerCertificate to rotate the server certificate.
    fn expect_rotate_server_ca(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `import_instance`: Imports data into a Cloud SQL instance from a SQL dump
    /// or CSV file in Cloud Storage.
    fn expect_import_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `export_instance`: Exports data from a Cloud SQL instance to a Cloud
    /// Storage bucket as a SQL dump or CSV file.
    fn expect_export_instance(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_backup_run`: Creates a new backup run on demand.
    fn expect_create_backup_run(&mut self, project: &str, instance: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `list_databases`: Lists databases in the specified Cloud SQL instance.
    fn expect_list_databases(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_database`: Retrieves a resource containing information about a
    /// database inside a Cloud SQL instance.
    fn expect_get_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_database`: Inserts a resource containing information about a
    /// database inside a Cloud SQL instance. **Note:** You can't modify the default character set
    /// and collation.
    fn expect_create_database(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_database`: Deletes a database from a Cloud SQL instance.
    fn expect_delete_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_database`: Partially updates a resource containing information
    /// about a database inside a Cloud SQL instance. This method supports patch semantics.
    fn expect_update_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_users`: Lists users in the specified Cloud SQL instance.
    fn expect_list_users(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_user`: Retrieves a resource containing information about a user.
    fn expect_get_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_user`: Creates a new user in a Cloud SQL instance.
    fn expect_create_user(&mut self, project: &str, instance: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_user`: Deletes a user from a Cloud SQL instance.
    fn expect_delete_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_user`: Updates an existing user in a Cloud SQL instance.
    fn expect_update_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_operations`: Lists all instance operations that have been performed
    /// on the given Cloud SQL instance in the reverse chronological order of the start time.
    fn expect_list_operations(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_operation`: Retrieves an instance operation that has been performed on
    /// an instance.
    fn expect_get_operation(&mut self, project: &str, operation: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SqladminMockHelpers for MockClient {
    /// Helper to expect `list_instances`: Lists instances under a given project.
    fn expect_list_instances(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances");
        self.expect_get(&path)
    }

    /// Helper to expect `get_instance`: Retrieves a resource containing information about a Cloud
    /// SQL instance.
    fn expect_get_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_instance`: Creates a new Cloud SQL instance.
    fn expect_create_instance(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_instance`: Deletes a Cloud SQL instance.
    fn expect_delete_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}");
        self.expect_delete(&path)
    }

    /// Helper to expect `update_instance`: Partially updates settings of a Cloud SQL instance by
    /// merging the request with the current configuration. This method supports patch semantics.
    fn expect_update_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}");
        self.expect_patch(&path)
    }

    /// Helper to expect `restart_instance`: Restarts a Cloud SQL instance.
    fn expect_restart_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/restart");
        self.expect_post(&path)
    }

    /// Helper to expect `clone_instance`: Creates a Cloud SQL instance as a clone of the source
    /// instance. Using this operation might cause your instance to restart.
    fn expect_clone_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/clone");
        self.expect_post(&path)
    }

    /// Helper to expect `failover_instance`: Initiates a manual failover of a high availability
    /// (HA) primary instance to a standby instance, which becomes the primary instance. Users are
    /// then rerouted to the new primary. For more information, see the [Overview of high
    /// availability](https://cloud.google.com/sql/docs/mysql/high-availability) page in the Cloud
    /// SQL documentation. If using Legacy HA (MySQL only), this causes the instance to failover to
    /// its failover replica instance.
    fn expect_failover_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/failover");
        self.expect_post(&path)
    }

    /// Helper to expect `promote_replica`: Promotes the read replica instance to be an independent
    /// Cloud SQL primary instance. Using this operation might cause your instance to restart.
    fn expect_promote_replica(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/promoteReplica");
        self.expect_post(&path)
    }

    /// Helper to expect `reset_ssl_config`: Deletes all client certificates and generates a new
    /// server SSL certificate for the instance.
    fn expect_reset_ssl_config(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/resetSslConfig");
        self.expect_post(&path)
    }

    /// Helper to expect `add_server_ca`: Adds a new trusted Certificate Authority (CA) version for
    /// the specified instance. Required to prepare for a certificate rotation. If a CA version was
    /// previously added but never used in a certificate rotation, this operation replaces that
    /// version. There cannot be more than one CA version waiting to be rotated in. For instances
    /// that have enabled Certificate Authority Service (CAS) based server CA, use
    /// AddServerCertificate to add a new server certificate.
    fn expect_add_server_ca(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/addServerCa");
        self.expect_post(&path)
    }

    /// Helper to expect `rotate_server_ca`: Rotates the server certificate to one signed by the
    /// Certificate Authority (CA) version previously added with the addServerCA method. For
    /// instances that have enabled Certificate Authority Service (CAS) based server CA, use
    /// RotateServerCertificate to rotate the server certificate.
    fn expect_rotate_server_ca(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/rotateServerCa");
        self.expect_post(&path)
    }

    /// Helper to expect `import_instance`: Imports data into a Cloud SQL instance from a SQL dump
    /// or CSV file in Cloud Storage.
    fn expect_import_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/import");
        self.expect_post(&path)
    }

    /// Helper to expect `export_instance`: Exports data from a Cloud SQL instance to a Cloud
    /// Storage bucket as a SQL dump or CSV file.
    fn expect_export_instance(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/export");
        self.expect_post(&path)
    }

    /// Helper to expect `create_backup_run`: Creates a new backup run on demand.
    fn expect_create_backup_run(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/backupRuns");
        self.expect_post(&path)
    }

    /// Helper to expect `list_databases`: Lists databases in the specified Cloud SQL instance.
    fn expect_list_databases(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/databases");
        self.expect_get(&path)
    }

    /// Helper to expect `get_database`: Retrieves a resource containing information about a
    /// database inside a Cloud SQL instance.
    fn expect_get_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/databases/{database}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_database`: Inserts a resource containing information about a
    /// database inside a Cloud SQL instance. **Note:** You can't modify the default character set
    /// and collation.
    fn expect_create_database(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/databases");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_database`: Deletes a database from a Cloud SQL instance.
    fn expect_delete_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/databases/{database}");
        self.expect_delete(&path)
    }

    /// Helper to expect `update_database`: Partially updates a resource containing information
    /// about a database inside a Cloud SQL instance. This method supports patch semantics.
    fn expect_update_database(
        &mut self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/databases/{database}");
        self.expect_patch(&path)
    }

    /// Helper to expect `list_users`: Lists users in the specified Cloud SQL instance.
    fn expect_list_users(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/users");
        self.expect_get(&path)
    }

    /// Helper to expect `get_user`: Retrieves a resource containing information about a user.
    fn expect_get_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/users/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_user`: Creates a new user in a Cloud SQL instance.
    fn expect_create_user(
        &mut self,
        project: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/instances/{instance}/users");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_user`: Deletes a user from a Cloud SQL instance.
    fn expect_delete_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/projects/{project}/instances/{instance}/users");
        let mut __qp: Vec<String> = Vec::new();
        if !name.is_empty() {
            __qp.push(format!("name={}", name));
        }
        if !host.is_empty() {
            __qp.push(format!("host={}", host));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_delete(&path)
    }

    /// Helper to expect `update_user`: Updates an existing user in a Cloud SQL instance.
    fn expect_update_user(
        &mut self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/projects/{project}/instances/{instance}/users");
        let mut __qp: Vec<String> = Vec::new();
        if !name.is_empty() {
            __qp.push(format!("name={}", name));
        }
        if !host.is_empty() {
            __qp.push(format!("host={}", host));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_put(&path)
    }

    /// Helper to expect `list_operations`: Lists all instance operations that have been performed
    /// on the given Cloud SQL instance in the reverse chronological order of the start time.
    fn expect_list_operations(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/operations");
        self.expect_get(&path)
    }

    /// Helper to expect `get_operation`: Retrieves an instance operation that has been performed on
    /// an instance.
    fn expect_get_operation(
        &mut self,
        project: &str,
        operation: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/projects/{project}/operations/{operation}");
        self.expect_get(&path)
    }
}
