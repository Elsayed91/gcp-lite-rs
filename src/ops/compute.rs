//! Operation contracts for the Compute Engine API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/compute.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::compute::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Compute Engine API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::compute::ComputeClient`] instead.
pub struct ComputeOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> ComputeOps<'a> {
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
        "https://compute.googleapis.com/compute/v1"
    }

    /// Creates a persistent disk in the specified project using the data in the request. You
    /// can create a disk from a source (sourceImage, sourceSnapshot, orsourceDisk) or create an
    /// empty 500 GB data disk by omitting all properties. You can also create a disk that is
    /// larger than the default size by specifying the sizeGb property.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/disks`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    /// - `sourceImage` — Source image to restore onto a disk. This field is optional.
    ///
    /// # Request Body
    /// [`Disk`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_disk(
        &self,
        project: &str,
        zone: &str,
        body: &Disk,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks",
            self.base_url(),
            encode(project),
            encode(zone),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified persistent disk. Deleting a disk removes its data permanently and
    /// is irreversible. However, deleting a disk does not delete any snapshots previously made
    /// from the disk. You must separatelydelete snapshots.
    ///
    /// **GCP API**: `DELETE projects/{project}/zones/{zone}/disks/{disk}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `disk` — Name of the persistent disk to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_disk(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(disk),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified persistent disk.
    ///
    /// **GCP API**: `GET projects/{project}/zones/{zone}/disks/{disk}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `disk` — Name of the persistent disk to return. *(required)*
    ///
    /// # Response
    /// [`Disk`]
    #[allow(dead_code)]
    pub(crate) async fn get_disk(&self, project: &str, zone: &str, disk: &str) -> Result<Disk> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(disk),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a list of persistent disks contained within the specified zone.
    ///
    /// **GCP API**: `GET projects/{project}/zones/{zone}/disks`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`DiskList`]
    #[allow(dead_code)]
    pub(crate) async fn list_disks(
        &self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> Result<DiskList> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks",
            self.base_url(),
            encode(project),
            encode(zone),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_disks response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a snapshot of a specified persistent disk. For regular snapshot creation,
    /// consider using snapshots.insert instead, as that method supports more features, such as
    /// creating snapshots in a project different from the source disk project.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/disks/{disk}/createSnapshot`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/createSnapshot>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `disk` — Name of the persistent disk to snapshot. *(required)*
    ///
    /// # Query Parameters
    /// - `guestFlush` — [Input Only] Whether to attempt an application consistent snapshot by informing the OS to prepare for the snapshot proce
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Snapshot`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_snapshot(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        body: &Snapshot,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}/createSnapshot",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(disk),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_snapshot response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates an instance resource in the specified project using the data included in the
    /// request.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    /// - `sourceInstanceTemplate` — Specifies instance template to create the instance. This field is optional. It can be a full or partial URL. For example
    /// - `sourceMachineImage` — Specifies the machine image to use to create the instance. This field is optional. It can be a full or partial URL. For
    ///
    /// # Request Body
    /// [`Instance`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_instance(
        &self,
        project: &str,
        zone: &str,
        body: &Instance,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            self.base_url(),
            encode(project),
            encode(zone),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified Instance resource. For more information, seeDeleting an instance.
    ///
    /// **GCP API**: `DELETE projects/{project}/zones/{zone}/instances/{instance}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance resource to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified Instance resource.
    ///
    /// **GCP API**: `GET projects/{project}/zones/{zone}/instances/{instance}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance resource to return. *(required)*
    ///
    /// # Response
    /// [`Instance`]
    #[allow(dead_code)]
    pub(crate) async fn get_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Instance> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves the list of instances contained within the specified zone.
    ///
    /// **GCP API**: `GET projects/{project}/zones/{zone}/instances`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`InstanceList`]
    #[allow(dead_code)]
    pub(crate) async fn list_instances(
        &self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> Result<InstanceList> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            self.base_url(),
            encode(project),
            encode(zone),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_instances response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Starts an instance that was stopped using theinstances().stop method. For more
    /// information, seeRestart an instance.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/start`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/start>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance resource to start. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn start_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/start",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse start_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Stops a running instance, shutting it down cleanly, and allows you to restart the
    /// instance at a later time. Stopped instances do not incur VM usage charges while they are
    /// stopped. However, resources that the VM is using, such as persistent disks and static IP
    /// addresses, will continue to be charged until they are deleted. For more information,
    /// seeStopping an instance.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/stop`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/stop>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance resource to stop. *(required)*
    ///
    /// # Query Parameters
    /// - `discardLocalSsd` — This property is required if the instance has any attached Local SSD disks. If false, Local SSD data will be preserved w
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn stop_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/stop",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse stop_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Performs a reset on the instance. This is a hard reset. The VM does not do a graceful
    /// shutdown. For more information, seeResetting an instance.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/reset`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/reset>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance scoping this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn reset_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/reset",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse reset_instance response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified Snapshot resource. Keep in mind that deleting a single snapshot
    /// might not necessarily delete all the data on that snapshot. If any data on the snapshot
    /// that is marked for deletion is needed for subsequent snapshots, the data will be moved
    /// to the next corresponding snapshot. For more information, seeDeleting snapshots.
    ///
    /// **GCP API**: `DELETE projects/{project}/global/snapshots/{snapshot}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/snapshots/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `snapshot` — Name of the Snapshot resource to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_snapshot(
        &self,
        project: &str,
        snapshot: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/snapshots/{}",
            self.base_url(),
            encode(project),
            encode(snapshot),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_snapshot response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified Snapshot resource.
    ///
    /// **GCP API**: `GET projects/{project}/global/snapshots/{snapshot}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/snapshots/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `snapshot` — Name of the Snapshot resource to return. *(required)*
    ///
    /// # Response
    /// [`Snapshot`]
    #[allow(dead_code)]
    pub(crate) async fn get_snapshot(&self, project: &str, snapshot: &str) -> Result<Snapshot> {
        let url = format!(
            "{}/projects/{}/global/snapshots/{}",
            self.base_url(),
            encode(project),
            encode(snapshot),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_snapshot response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves the list of Snapshot resources contained within the specified project.
    ///
    /// **GCP API**: `GET projects/{project}/global/snapshots`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/snapshots/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`SnapshotList`]
    #[allow(dead_code)]
    pub(crate) async fn list_snapshots(
        &self,
        project: &str,
        page_token: &str,
    ) -> Result<SnapshotList> {
        let url = format!(
            "{}/projects/{}/global/snapshots",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_snapshots response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified address resource.
    ///
    /// **GCP API**: `DELETE projects/{project}/regions/{region}/addresses/{address}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/addresses/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region for this request. *(required)*
    /// - `address` — Name of the address resource to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn release_address(
        &self,
        project: &str,
        region: &str,
        address: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/regions/{}/addresses/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(address),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse release_address response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified address resource.
    ///
    /// **GCP API**: `GET projects/{project}/regions/{region}/addresses/{address}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/addresses/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region for this request. *(required)*
    /// - `address` — Name of the address resource to return. *(required)*
    ///
    /// # Response
    /// [`Address`]
    #[allow(dead_code)]
    pub(crate) async fn get_address(
        &self,
        project: &str,
        region: &str,
        address: &str,
    ) -> Result<Address> {
        let url = format!(
            "{}/projects/{}/regions/{}/addresses/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(address),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_address response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves a list of addresses contained within the specified region.
    ///
    /// **GCP API**: `GET projects/{project}/regions/{region}/addresses`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/addresses/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`AddressList`]
    #[allow(dead_code)]
    pub(crate) async fn list_addresses(
        &self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> Result<AddressList> {
        let url = format!(
            "{}/projects/{}/regions/{}/addresses",
            self.base_url(),
            encode(project),
            encode(region),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_addresses response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified Router resource.
    ///
    /// **GCP API**: `GET projects/{project}/regions/{region}/routers/{router}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/routers/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region for this request. *(required)*
    /// - `router` — Name of the Router resource to return. *(required)*
    ///
    /// # Response
    /// [`Router`]
    #[allow(dead_code)]
    pub(crate) async fn get_router(
        &self,
        project: &str,
        region: &str,
        router: &str,
    ) -> Result<Router> {
        let url = format!(
            "{}/projects/{}/regions/{}/routers/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(router),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_router response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Patches the specified Router resource with the data included in the request. This method
    /// supportsPATCH semantics and usesJSON merge patch format and processing rules.
    ///
    /// **GCP API**: `PATCH projects/{project}/regions/{region}/routers/{router}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/routers/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region for this request. *(required)*
    /// - `router` — Name of the Router resource to patch. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Router`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_router(
        &self,
        project: &str,
        region: &str,
        router: &str,
        body: &Router,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/regions/{}/routers/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(router),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_router response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes an access config from an instance's network interface.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/deleteAccessConfig`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/deleteAccessConfig>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — The instance name for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `accessConfig` — The name of the access config to delete.
    /// - `networkInterface` — The name of the network interface.
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn remove_access_config(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        access_config: &str,
        network_interface: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/deleteAccessConfig",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let url = crate::append_query_params(
            url,
            &[
                ("accessConfig", access_config),
                ("networkInterface", network_interface),
            ],
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse remove_access_config response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets the auto-delete flag for a disk attached to an instance.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/setDiskAutoDelete`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setDiskAutoDelete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — The instance name for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `autoDelete` — Whether to auto-delete the disk when the instance is deleted.
    /// - `deviceName` — The device name of the disk to modify. Make a get() request on the instance to view currently attached disks and device
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_disk_auto_delete(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        auto_delete: &str,
        device_name: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/setDiskAutoDelete",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let url = crate::append_query_params(
            url,
            &[("autoDelete", auto_delete), ("deviceName", device_name)],
        );
        let response = self
            .client
            .post(&url, &serde_json::Value::Object(Default::default()))
            .await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_disk_auto_delete response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Changes the machine type for a stopped instance to the machine type specified in the
    /// request.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/setMachineType`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setMachineType>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance scoping this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`InstancesSetMachineTypeRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_machine_type(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        body: &InstancesSetMachineTypeRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/setMachineType",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_machine_type response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Resizes the specified persistent disk. You can only increase the size of the disk.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/disks/{disk}/resize`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/disks/resize>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `disk` — The name of the persistent disk. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`DisksResizeRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn resize_disk(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        body: &DisksResizeRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/disks/{}/resize",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(disk),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse resize_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified BackendService resource.
    ///
    /// **GCP API**: `GET projects/{project}/global/backendServices/{backendService}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/backendServices/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `backendService` — Name of the BackendService resource to return. *(required)*
    ///
    /// # Response
    /// [`BackendService`]
    #[allow(dead_code)]
    pub(crate) async fn get_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
    ) -> Result<BackendService> {
        let url = format!(
            "{}/projects/{}/global/backendServices/{}",
            self.base_url(),
            encode(project),
            encode(backend_service),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_global_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves the list of BackendService resources available to the specified project.
    ///
    /// **GCP API**: `GET projects/{project}/global/backendServices`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/backendServices/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`BackendServiceList`]
    #[allow(dead_code)]
    pub(crate) async fn list_global_backend_services(
        &self,
        project: &str,
        page_token: &str,
    ) -> Result<BackendServiceList> {
        let url = format!(
            "{}/projects/{}/global/backendServices",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_global_backend_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a BackendService resource in the specified project using the data included in
    /// the request. For more information, see Backend services overview.
    ///
    /// **GCP API**: `POST projects/{project}/global/backendServices`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/backendServices/insert>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`BackendService`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn create_global_backend_service(
        &self,
        project: &str,
        body: &BackendService,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/backendServices",
            self.base_url(),
            encode(project),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_global_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified BackendService resource.
    ///
    /// **GCP API**: `DELETE projects/{project}/global/backendServices/{backendService}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/backendServices/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `backendService` — Name of the BackendService resource to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/backendServices/{}",
            self.base_url(),
            encode(project),
            encode(backend_service),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_global_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Patches the specified BackendService resource with the data included in the request. For
    /// more information, see Backend services overview. This method supports PATCH semantics
    /// and uses the JSON merge patch format and processing rules.
    ///
    /// **GCP API**: `PATCH projects/{project}/global/backendServices/{backendService}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/backendServices/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `backendService` — Name of the BackendService resource to patch. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`BackendService`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
        body: &BackendService,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/backendServices/{}",
            self.base_url(),
            encode(project),
            encode(backend_service),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_global_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified regional BackendService resource.
    ///
    /// **GCP API**: `GET projects/{project}/regions/{region}/backendServices/{backendService}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region scoping this request. *(required)*
    /// - `backendService` — Name of the BackendService resource to return. *(required)*
    ///
    /// # Response
    /// [`BackendService`]
    #[allow(dead_code)]
    pub(crate) async fn get_regional_backend_service(
        &self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> Result<BackendService> {
        let url = format!(
            "{}/projects/{}/regions/{}/backendServices/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(backend_service),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_regional_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves the list of regional BackendService resources available to the specified
    /// project in the given region.
    ///
    /// **GCP API**: `GET projects/{project}/regions/{region}/backendServices`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region scoping this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`BackendServiceList`]
    #[allow(dead_code)]
    pub(crate) async fn list_regional_backend_services(
        &self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> Result<BackendServiceList> {
        let url = format!(
            "{}/projects/{}/regions/{}/backendServices",
            self.base_url(),
            encode(project),
            encode(region),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_regional_backend_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified regional BackendService resource.
    ///
    /// **GCP API**: `DELETE projects/{project}/regions/{region}/backendServices/{backendService}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region scoping this request. *(required)*
    /// - `backendService` — Name of the BackendService resource to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_regional_backend_service(
        &self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/regions/{}/backendServices/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(backend_service),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_regional_backend_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets an instance's scheduling options. You can only call this method on astopped
    /// instance, that is, a VM instance that is in a `TERMINATED` state. SeeInstance Life Cycle
    /// for more information on the possible instance states. For more information about setting
    /// scheduling options for a VM, seeSet VM host maintenance policy.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/setScheduling`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setScheduling>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Instance name for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Scheduling`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_scheduling(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        body: &Scheduling,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/setScheduling",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_scheduling response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the specified firewall.
    ///
    /// **GCP API**: `GET projects/{project}/global/firewalls/{firewall}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/firewalls/get>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `firewall` — Name of the firewall rule to return. *(required)*
    ///
    /// # Response
    /// [`Firewall`]
    #[allow(dead_code)]
    pub(crate) async fn get_firewall(&self, project: &str, firewall: &str) -> Result<Firewall> {
        let url = format!(
            "{}/projects/{}/global/firewalls/{}",
            self.base_url(),
            encode(project),
            encode(firewall),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_firewall response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Retrieves the list of firewall rules available to the specified project.
    ///
    /// **GCP API**: `GET projects/{project}/global/firewalls`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/firewalls/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`FirewallList`]
    #[allow(dead_code)]
    pub(crate) async fn list_firewalls(
        &self,
        project: &str,
        page_token: &str,
    ) -> Result<FirewallList> {
        let url = format!(
            "{}/projects/{}/global/firewalls",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_firewalls response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified firewall.
    ///
    /// **GCP API**: `DELETE projects/{project}/global/firewalls/{firewall}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/firewalls/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `firewall` — Name of the firewall rule to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_firewall(
        &self,
        project: &str,
        firewall: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/firewalls/{}",
            self.base_url(),
            encode(project),
            encode(firewall),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_firewall response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates the specified firewall rule with the data included in the request. This method
    /// supportsPATCH semantics and uses theJSON merge patch format and processing rules.
    ///
    /// **GCP API**: `PATCH projects/{project}/global/firewalls/{firewall}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/firewalls/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `firewall` — Name of the firewall rule to patch. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Firewall`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_firewall(
        &self,
        project: &str,
        firewall: &str,
        body: &Firewall,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/firewalls/{}",
            self.base_url(),
            encode(project),
            encode(firewall),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_firewall response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets metadata for the specified instance to the data included in the request.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/setMetadata`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setMetadata>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance scoping this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Metadata`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_instance_metadata(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        body: &Metadata,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/setMetadata",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_instance_metadata response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets metadata common to all instances within the specified project using the data
    /// included in the request.
    ///
    /// **GCP API**: `POST projects/{project}/setCommonInstanceMetadata`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/projects/setCommonInstanceMetadata>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Metadata`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_project_common_instance_metadata(
        &self,
        project: &str,
        body: &Metadata,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/setCommonInstanceMetadata",
            self.base_url(),
            encode(project),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_project_common_instance_metadata response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets the service account on the instance. For more information, readChanging the service
    /// account and access scopes for an instance.
    ///
    /// **GCP API**: `POST projects/{project}/zones/{zone}/instances/{instance}/setServiceAccount`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setServiceAccount>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name of the instance resource to start. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`InstancesSetServiceAccountRequest`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn set_instance_service_account(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        body: &InstancesSetServiceAccountRequest,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/setServiceAccount",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_instance_service_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates the Shielded Instance config for an instance. You can only use this method on a
    /// stopped instance. This method supportsPATCH semantics and uses theJSON merge patch
    /// format and processing rules.
    ///
    /// **GCP API**: `PATCH projects/{project}/zones/{zone}/instances/{instance}/updateShieldedInstanceConfig`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/instances/updateShieldedInstanceConfig>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `zone` — The name of the zone for this request. *(required)*
    /// - `instance` — Name or id of the instance scoping this request. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`ShieldedInstanceConfig`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn update_shielded_instance_config(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        body: &ShieldedInstanceConfig,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/updateShieldedInstanceConfig",
            self.base_url(),
            encode(project),
            encode(zone),
            encode(instance),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_shielded_instance_config response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the specified network.
    ///
    /// **GCP API**: `DELETE projects/{project}/global/networks/{network}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/networks/delete>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `network` — Name of the network to delete. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn delete_network(
        &self,
        project: &str,
        network: &str,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/networks/{}",
            self.base_url(),
            encode(project),
            encode(network),
        );
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_network response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Patches the specified subnetwork with the data included in the request. Only certain
    /// fields can be updated with a patch request as indicated in the field descriptions. You
    /// must specify the current fingerprint of the subnetwork resource being patched.
    ///
    /// **GCP API**: `PATCH projects/{project}/regions/{region}/subnetworks/{subnetwork}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/subnetworks/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `region` — Name of the region scoping this request. *(required)*
    /// - `subnetwork` — Name of the Subnetwork resource to patch. *(required)*
    ///
    /// # Query Parameters
    /// - `drainTimeoutSeconds` — The drain timeout specifies the upper bound in seconds on the amount of time allowed to drain connections from the curre
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`Subnetwork`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_subnetwork(
        &self,
        project: &str,
        region: &str,
        subnetwork: &str,
        body: &Subnetwork,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/regions/{}/subnetworks/{}",
            self.base_url(),
            encode(project),
            encode(region),
            encode(subnetwork),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_subnetwork response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all the SSL policies that have been configured for the specified project.
    ///
    /// **GCP API**: `GET projects/{project}/global/sslPolicies`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/sslPolicies/list>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — A filter expression that filters resources listed in the response. Most Compute resources support two types of filter ex
    /// - `maxResults` — The maximum number of results per page that should be returned. If the number of available results is larger than `maxRe
    /// - `orderBy` — Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource na
    /// - `pageToken` — Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the nex
    /// - `returnPartialSuccess` — Opt-in for partial success behavior which provides partial results in case of failure. The default value is false. For e
    ///
    /// # Response
    /// [`SslPoliciesList`]
    #[allow(dead_code)]
    pub(crate) async fn list_ssl_policies(
        &self,
        project: &str,
        page_token: &str,
        filter: &str,
    ) -> Result<SslPoliciesList> {
        let url = format!(
            "{}/projects/{}/global/sslPolicies",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(url, &[("pageToken", page_token), ("filter", filter)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_ssl_policies response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Patches the specified SSL policy with the data included in the request.
    ///
    /// **GCP API**: `PATCH projects/{project}/global/sslPolicies/{sslPolicy}`
    /// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/sslPolicies/patch>
    ///
    /// # Path Parameters
    /// - `project` — Project ID for this request. *(required)*
    /// - `sslPolicy` — Name of the SSL policy to update. The name must be 1-63 characters long, and comply with RFC1035. *(required)*
    ///
    /// # Query Parameters
    /// - `requestId` — An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the ser
    ///
    /// # Request Body
    /// [`SslPolicy`]
    ///
    /// # Response
    /// [`OperationResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_ssl_policy(
        &self,
        project: &str,
        ssl_policy: &str,
        body: &SslPolicy,
    ) -> Result<OperationResponse> {
        let url = format!(
            "{}/projects/{}/global/sslPolicies/{}",
            self.base_url(),
            encode(project),
            encode(ssl_policy),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_ssl_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/disks")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Disk::fixture();
        let result = ops.create_disk("test-project", "test-zone", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/test-project/zones/test-zone/disks/test-disk")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_disk("test-project", "test-zone", "test-disk")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/zones/test-zone/disks/test-disk")
            .returning_json(serde_json::to_value(Disk::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.get_disk("test-project", "test-zone", "test-disk").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_disks() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/zones/test-zone/disks?pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(DiskList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_disks("test-project", "test-zone", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/disks/test-disk/createSnapshot",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Snapshot::fixture();
        let result = ops
            .create_snapshot("test-project", "test-zone", "test-disk", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Instance::fixture();
        let result = ops
            .create_instance("test-project", "test-zone", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_instance("test-project", "test-zone", "test-instance")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance",
        )
        .returning_json(serde_json::to_value(Instance::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_instance("test-project", "test-zone", "test-instance")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/zones/test-zone/instances?pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(InstanceList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_instances("test-project", "test-zone", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/start",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .start_instance("test-project", "test-zone", "test-instance")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/stop",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .stop_instance("test-project", "test-zone", "test-instance")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reset_instance() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/reset",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .reset_instance("test-project", "test-zone", "test-instance")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/test-project/global/snapshots/test-snapshot")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.delete_snapshot("test-project", "test-snapshot").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/global/snapshots/test-snapshot")
            .returning_json(serde_json::to_value(Snapshot::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.get_snapshot("test-project", "test-snapshot").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_snapshots() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/global/snapshots?pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(SnapshotList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.list_snapshots("test-project", "test-pageToken").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_release_address() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/compute/v1/projects/test-project/regions/test-region/addresses/test-address",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .release_address("test-project", "test-region", "test-address")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_address() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/regions/test-region/addresses/test-address",
        )
        .returning_json(serde_json::to_value(Address::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_address("test-project", "test-region", "test-address")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_addresses() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/regions/test-region/addresses?pageToken=test-pageToken")
            .returning_json(serde_json::to_value(AddressList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_addresses("test-project", "test-region", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_router() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/regions/test-region/routers/test-router",
        )
        .returning_json(serde_json::to_value(Router::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_router("test-project", "test-region", "test-router")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_router() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/compute/v1/projects/test-project/regions/test-region/routers/test-router",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Router::fixture();
        let result = ops
            .patch_router("test-project", "test-region", "test-router", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_access_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/deleteAccessConfig?accessConfig=test-accessConfig&networkInterface=test-networkInterface")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .remove_access_config(
                "test-project",
                "test-zone",
                "test-instance",
                "test-accessConfig",
                "test-networkInterface",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_disk_auto_delete() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/setDiskAutoDelete?autoDelete=test-autoDelete&deviceName=test-deviceName")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .set_disk_auto_delete(
                "test-project",
                "test-zone",
                "test-instance",
                "test-autoDelete",
                "test-deviceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_machine_type() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/setMachineType")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = InstancesSetMachineTypeRequest::fixture();
        let result = ops
            .set_machine_type("test-project", "test-zone", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resize_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/disks/test-disk/resize",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = DisksResizeRequest::fixture();
        let result = ops
            .resize_disk("test-project", "test-zone", "test-disk", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_global_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/global/backendServices/test-backendService",
        )
        .returning_json(serde_json::to_value(BackendService::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_global_backend_service("test-project", "test-backendService")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_global_backend_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/global/backendServices?pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(BackendServiceList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_global_backend_services("test-project", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_global_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/global/backendServices")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = BackendService::fixture();
        let result = ops
            .create_global_backend_service("test-project", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_global_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/compute/v1/projects/test-project/global/backendServices/test-backendService",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_global_backend_service("test-project", "test-backendService")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_global_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/compute/v1/projects/test-project/global/backendServices/test-backendService",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = BackendService::fixture();
        let result = ops
            .patch_global_backend_service("test-project", "test-backendService", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_regional_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/regions/test-region/backendServices/test-backendService")
            .returning_json(serde_json::to_value(BackendService::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_regional_backend_service("test-project", "test-region", "test-backendService")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_regional_backend_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/regions/test-region/backendServices?pageToken=test-pageToken")
            .returning_json(serde_json::to_value(BackendServiceList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_regional_backend_services("test-project", "test-region", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_regional_backend_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/test-project/regions/test-region/backendServices/test-backendService")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_regional_backend_service("test-project", "test-region", "test-backendService")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_scheduling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/setScheduling")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Scheduling::fixture();
        let result = ops
            .set_scheduling("test-project", "test-zone", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/global/firewalls/test-firewall")
            .returning_json(serde_json::to_value(Firewall::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.get_firewall("test-project", "test-firewall").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_firewalls() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/compute/v1/projects/test-project/global/firewalls?pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(FirewallList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.list_firewalls("test-project", "test-pageToken").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/test-project/global/firewalls/test-firewall")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.delete_firewall("test-project", "test-firewall").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/test-project/global/firewalls/test-firewall")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Firewall::fixture();
        let result = ops
            .patch_firewall("test-project", "test-firewall", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_instance_metadata() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/setMetadata",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Metadata::fixture();
        let result = ops
            .set_instance_metadata("test-project", "test-zone", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_project_common_instance_metadata() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/setCommonInstanceMetadata")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Metadata::fixture();
        let result = ops
            .set_project_common_instance_metadata("test-project", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_instance_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/setServiceAccount")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = InstancesSetServiceAccountRequest::fixture();
        let result = ops
            .set_instance_service_account("test-project", "test-zone", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_shielded_instance_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/test-project/zones/test-zone/instances/test-instance/updateShieldedInstanceConfig")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = ShieldedInstanceConfig::fixture();
        let result = ops
            .update_shielded_instance_config("test-project", "test-zone", "test-instance", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_network() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/test-project/global/networks/test-network")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.delete_network("test-project", "test-network").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_subnetwork() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/compute/v1/projects/test-project/regions/test-region/subnetworks/test-subnetwork",
        )
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = Subnetwork::fixture();
        let result = ops
            .patch_subnetwork("test-project", "test-region", "test-subnetwork", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_ssl_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/test-project/global/sslPolicies?pageToken=test-pageToken&filter=test-filter")
            .returning_json(serde_json::to_value(SslPoliciesList::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_ssl_policies("test-project", "test-pageToken", "test-filter")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_ssl_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/test-project/global/sslPolicies/test-sslPolicy")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = SslPolicy::fixture();
        let result = ops
            .patch_ssl_policy("test-project", "test-sslPolicy", &body)
            .await;
        assert!(result.is_ok());
    }
}
