//! Operation contracts for the Cloud SQL Admin API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/sqladmin.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::sqladmin::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Cloud SQL Admin API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::sqladmin::SqladminClient`] instead.
pub struct SqladminOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> SqladminOps<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://sqladmin.googleapis.com"
    }

    /// Lists instances under a given project.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project for which to list Cloud SQL instances. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. The expression is in the form of field:value. For exa
    /// - `maxResults` — The maximum number of instances to return. The service may return fewer than this value. If unspecified, at most 500 ins
    /// - `pageToken` — A previously-returned page token representing part of the larger set of results to view.
    ///
    /// # Response
    /// [`InstancesListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_instances(&self, project: &str) -> Result<InstancesListResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances",
            self.base_url(),
            encode(project),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_instances response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a resource containing information about a Cloud SQL instance.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances/{instance}`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/get>
    ///
    /// # Path Parameters
    /// - `project` — Required. Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Required. Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Response
    /// [`DatabaseInstance`]
    #[allow(dead_code)]
    pub(crate) async fn get_instance(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<DatabaseInstance> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new Cloud SQL instance.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project to which the newly created Cloud SQL instances should belong. *(required)*
    ///
    /// # Request Body
    /// [`DatabaseInstance`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_instance(
        &self,
        project: &str,
        body: &DatabaseInstance,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances",
            self.base_url(),
            encode(project),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a Cloud SQL instance.
    ///
    /// **GCP API**: `DELETE v1/projects/{project}/instances/{instance}`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance to be deleted. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Query Parameters
    /// - `enableFinalBackup` — Flag to opt-in for final backup. By default, it is turned off.
    /// - `finalBackupDescription` — Optional. The description of the final backup.
    /// - `finalBackupExpiryTime` — Optional. Final Backup expiration time. Timestamp in UTC of when this resource is considered expired.
    /// - `finalBackupTtlDays` — Optional. Retention period of the final backup.
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_instance(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Partially updates settings of a Cloud SQL instance by merging the request with the
    /// current configuration. This method supports patch semantics.
    ///
    /// **GCP API**: `PATCH v1/projects/{project}/instances/{instance}`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`DatabaseInstance`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_instance(
        &self,
        project: &str,
        instance: &str,
        body: &DatabaseInstance,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Restarts a Cloud SQL instance.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/restart`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/restart>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance to be restarted. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn restart_instance(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/restart",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse restart_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a Cloud SQL instance as a clone of the source instance. Using this operation
    /// might cause your instance to restart.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/clone`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/clone>
    ///
    /// # Path Parameters
    /// - `project` — Required. Project ID of the source as well as the clone Cloud SQL instance. *(required)*
    /// - `instance` — Required. The ID of the Cloud SQL instance to be cloned (source). This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`InstancesCloneRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn clone_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesCloneRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/clone",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse clone_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Initiates a manual failover of a high availability (HA) primary instance to a standby
    /// instance, which becomes the primary instance. Users are then rerouted to the new
    /// primary. For more information, see the [Overview of high
    /// availability](https://cloud.google.com/sql/docs/mysql/high-availability) page in the
    /// Cloud SQL documentation. If using Legacy HA (MySQL only), this causes the instance to
    /// failover to its failover replica instance.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/failover`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/failover>
    ///
    /// # Path Parameters
    /// - `project` — ID of the project that contains the read replica. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`InstancesFailoverRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn failover_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesFailoverRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/failover",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse failover_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Promotes the read replica instance to be an independent Cloud SQL primary instance.
    /// Using this operation might cause your instance to restart.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/promoteReplica`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/promoteReplica>
    ///
    /// # Path Parameters
    /// - `project` — ID of the project that contains the read replica. *(required)*
    /// - `instance` — Cloud SQL read replica instance name. *(required)*
    ///
    /// # Query Parameters
    /// - `failover` — Set to true to invoke a replica failover to the DR replica. As part of replica failover, the promote operation attempts
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn promote_replica(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/promoteReplica",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse promote_replica response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes all client certificates and generates a new server SSL certificate for the
    /// instance.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/resetSslConfig`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/resetSslConfig>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Query Parameters
    /// - `mode` — Optional. Reset SSL mode to use.
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn reset_ssl_config(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/resetSslConfig",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse reset_ssl_config response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Adds a new trusted Certificate Authority (CA) version for the specified instance.
    /// Required to prepare for a certificate rotation. If a CA version was previously added but
    /// never used in a certificate rotation, this operation replaces that version. There cannot
    /// be more than one CA version waiting to be rotated in. For instances that have enabled
    /// Certificate Authority Service (CAS) based server CA, use AddServerCertificate to add a
    /// new server certificate.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/addServerCa`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/addServerCa>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn add_server_ca(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/addServerCa",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse add_server_ca response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Rotates the server certificate to one signed by the Certificate Authority (CA) version
    /// previously added with the addServerCA method. For instances that have enabled
    /// Certificate Authority Service (CAS) based server CA, use RotateServerCertificate to
    /// rotate the server certificate.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/rotateServerCa`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/rotateServerCa>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`InstancesRotateServerCaRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn rotate_server_ca(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesRotateServerCaRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/rotateServerCa",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse rotate_server_ca response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Imports data into a Cloud SQL instance from a SQL dump or CSV file in Cloud Storage.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/import`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/import>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`InstancesImportRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn import_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesImportRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/import",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse import_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL dump or CSV
    /// file.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/export`
    /// **Reference**: <https://cloud.google.com/sql/docs/instances/export>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance to be exported. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`InstancesExportRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn export_instance(
        &self,
        project: &str,
        instance: &str,
        body: &InstancesExportRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/export",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse export_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new backup run on demand.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/backupRuns`
    /// **Reference**: <https://cloud.google.com/sql/docs/backupRuns/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`BackupRun`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_backup_run(
        &self,
        project: &str,
        instance: &str,
        body: &BackupRun,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/backupRuns",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_backup_run response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists databases in the specified Cloud SQL instance.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances/{instance}/databases`
    /// **Reference**: <https://cloud.google.com/sql/docs/databases/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID. *(required)*
    ///
    /// # Response
    /// [`DatabasesListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_databases(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<DatabasesListResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/databases",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_databases response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a resource containing information about a database inside a Cloud SQL
    /// instance.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances/{instance}/databases/{database}`
    /// **Reference**: <https://cloud.google.com/sql/docs/databases/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    /// - `database` — Name of the database in the instance. *(required)*
    ///
    /// # Response
    /// [`Database`]
    #[allow(dead_code)]
    pub(crate) async fn get_database(
        &self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> Result<Database> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/databases/{}",
            self.base_url(),
            encode(project),
            encode(instance),
            encode(database),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Inserts a resource containing information about a database inside a Cloud SQL instance.
    /// **Note:** You can't modify the default character set and collation.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/databases`
    /// **Reference**: <https://cloud.google.com/sql/docs/databases/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`Database`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_database(
        &self,
        project: &str,
        instance: &str,
        body: &Database,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/databases",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a database from a Cloud SQL instance.
    ///
    /// **GCP API**: `DELETE v1/projects/{project}/instances/{instance}/databases/{database}`
    /// **Reference**: <https://cloud.google.com/sql/docs/databases/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    /// - `database` — Name of the database to be deleted in the instance. *(required)*
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_database(
        &self,
        project: &str,
        instance: &str,
        database: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/databases/{}",
            self.base_url(),
            encode(project),
            encode(instance),
            encode(database),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Partially updates a resource containing information about a database inside a Cloud SQL
    /// instance. This method supports patch semantics.
    ///
    /// **GCP API**: `PATCH v1/projects/{project}/instances/{instance}/databases/{database}`
    /// **Reference**: <https://cloud.google.com/sql/docs/databases/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    /// - `database` — Name of the database to be updated in the instance. *(required)*
    ///
    /// # Request Body
    /// [`Database`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_database(
        &self,
        project: &str,
        instance: &str,
        database: &str,
        body: &Database,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/databases/{}",
            self.base_url(),
            encode(project),
            encode(instance),
            encode(database),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists users in the specified Cloud SQL instance.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances/{instance}/users`
    /// **Reference**: <https://cloud.google.com/sql/docs/users/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Response
    /// [`UsersListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_users(
        &self,
        project: &str,
        instance: &str,
    ) -> Result<UsersListResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/users",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_users response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a resource containing information about a user.
    ///
    /// **GCP API**: `GET v1/projects/{project}/instances/{instance}/users/{name}`
    /// **Reference**: <https://cloud.google.com/sql/docs/users/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    /// - `name` — User of the instance. *(required)*
    ///
    /// # Query Parameters
    /// - `host` — Host of a user of the instance.
    ///
    /// # Response
    /// [`User`]
    #[allow(dead_code)]
    pub(crate) async fn get_user(&self, project: &str, instance: &str, name: &str) -> Result<User> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/users/{}",
            self.base_url(),
            encode(project),
            encode(instance),
            encode(name),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_user response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new user in a Cloud SQL instance.
    ///
    /// **GCP API**: `POST v1/projects/{project}/instances/{instance}/users`
    /// **Reference**: <https://cloud.google.com/sql/docs/users/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Request Body
    /// [`User`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_user(
        &self,
        project: &str,
        instance: &str,
        body: &User,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/users",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_user response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a user from a Cloud SQL instance.
    ///
    /// **GCP API**: `DELETE v1/projects/{project}/instances/{instance}/users`
    /// **Reference**: <https://cloud.google.com/sql/docs/users/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Query Parameters
    /// - `host` — Host of the user in the instance.
    /// - `name` — Name of the user in the instance.
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_user(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/users",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let url = crate::append_query_params(url, &[("name", name), ("host", host)]);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_user response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates an existing user in a Cloud SQL instance.
    ///
    /// **GCP API**: `PUT v1/projects/{project}/instances/{instance}/users`
    /// **Reference**: <https://cloud.google.com/sql/docs/users/update>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    /// - `instance` — Database instance ID. This does not include the project ID. *(required)*
    ///
    /// # Query Parameters
    /// - `databaseRoles` — Optional. List of database roles to grant to the user. body.database_roles will be ignored for update request.
    /// - `host` — Optional. Host of the user in the instance.
    /// - `name` — Name of the user in the instance.
    /// - `revokeExistingRoles` — Optional. Specifies whether to revoke existing roles that are not present in the `database_roles` field. If `false` or u
    ///
    /// # Request Body
    /// [`User`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_user(
        &self,
        project: &str,
        instance: &str,
        name: &str,
        host: &str,
        body: &User,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/instances/{}/users",
            self.base_url(),
            encode(project),
            encode(instance),
        );
        let url = crate::append_query_params(url, &[("name", name), ("host", host)]);
        let response = self.client.put(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_user response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all instance operations that have been performed on the given Cloud SQL instance
    /// in the reverse chronological order of the start time.
    ///
    /// **GCP API**: `GET v1/projects/{project}/operations`
    /// **Reference**: <https://cloud.google.com/sql/docs/operations/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID of the project that contains the instance. *(required)*
    ///
    /// # Query Parameters
    /// - `instance` — Cloud SQL instance ID. This does not include the project ID.
    /// - `maxResults` — Maximum number of operations per response.
    /// - `pageToken` — A previously-returned page token representing part of the larger set of results to view.
    ///
    /// # Response
    /// [`OperationsListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_operations(&self, project: &str) -> Result<OperationsListResponse> {
        let url = format!(
            "{}/v1/projects/{}/operations",
            self.base_url(),
            encode(project),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_operations response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves an instance operation that has been performed on an instance.
    ///
    /// **GCP API**: `GET v1/projects/{project}/operations/{operation}`
    /// **Reference**: <https://cloud.google.com/sql/docs/operations/get>
    ///
    /// # Path Parameters
    /// - `project` — Required. Project ID of the project that contains the instance. *(required)*
    /// - `operation` — Required. Instance operation ID. *(required)*
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_operation(
        &self,
        project: &str,
        operation: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/v1/projects/{}/operations/{}",
            self.base_url(),
            encode(project),
            encode(operation),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_operation response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/instances")
            .returning_json(serde_json::to_value(InstancesListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.list_instances("test-project").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/instances/test-instance")
            .returning_json(serde_json::to_value(DatabaseInstance::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.get_instance("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = DatabaseInstance::fixture();
        let result = ops.create_instance("test-project", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/projects/test-project/instances/test-instance")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.delete_instance("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/v1/projects/test-project/instances/test-instance")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = DatabaseInstance::fixture();
        let result = ops
            .update_instance("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_restart_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/restart")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.restart_instance("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clone_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/clone")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = InstancesCloneRequest::fixture();
        let result = ops
            .clone_instance("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_failover_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/failover")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = InstancesFailoverRequest::fixture();
        let result = ops
            .failover_instance("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_promote_replica() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/promoteReplica")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.promote_replica("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reset_ssl_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/resetSslConfig")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.reset_ssl_config("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_server_ca() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/addServerCa")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.add_server_ca("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rotate_server_ca() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/rotateServerCa")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = InstancesRotateServerCaRequest::fixture();
        let result = ops
            .rotate_server_ca("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_import_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/import")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = InstancesImportRequest::fixture();
        let result = ops
            .import_instance("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/export")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = InstancesExportRequest::fixture();
        let result = ops
            .export_instance("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_backup_run() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/backupRuns")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = BackupRun::fixture();
        let result = ops
            .create_backup_run("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_databases() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/instances/test-instance/databases")
            .returning_json(serde_json::to_value(DatabasesListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.list_databases("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/v1/projects/test-project/instances/test-instance/databases/test-database",
        )
        .returning_json(serde_json::to_value(Database::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops
            .get_database("test-project", "test-instance", "test-database")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/databases")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = Database::fixture();
        let result = ops
            .create_database("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/v1/projects/test-project/instances/test-instance/databases/test-database",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops
            .delete_database("test-project", "test-instance", "test-database")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/v1/projects/test-project/instances/test-instance/databases/test-database",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = Database::fixture();
        let result = ops
            .update_database("test-project", "test-instance", "test-database", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_users() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/instances/test-instance/users")
            .returning_json(serde_json::to_value(UsersListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.list_users("test-project", "test-instance").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/instances/test-instance/users/test-name")
            .returning_json(serde_json::to_value(User::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops
            .get_user("test-project", "test-instance", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/instances/test-instance/users")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = User::fixture();
        let result = ops
            .create_user("test-project", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/v1/projects/test-project/instances/test-instance/users?name=test-name&host=test-host",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops
            .delete_user("test-project", "test-instance", "test-name", "test-host")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user() {
        let mut mock = crate::MockClient::new();

        mock.expect_put(
            "/v1/projects/test-project/instances/test-instance/users?name=test-name&host=test-host",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let body = User::fixture();
        let result = ops
            .update_user(
                "test-project",
                "test-instance",
                "test-name",
                "test-host",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_operations() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/operations")
            .returning_json(serde_json::to_value(OperationsListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.list_operations("test-project").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_operation() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/operations/test-operation")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SqladminOps::new(&client);

        let result = ops.get_operation("test-project", "test-operation").await;
        assert!(result.is_ok());
    }
}
