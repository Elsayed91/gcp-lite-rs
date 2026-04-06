//! Compute Engine API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::compute::ComputeOps`. This layer adds:
//! - Ergonomic method signatures (project/zone/name instead of raw resource names)
//! - Blocking variants that poll operations to completion
//! - Pagination helpers (streams)

use crate::{
    GcpHttpClient, Result,
    operation::{Operation, PollConfig},
    ops::compute::ComputeOps,
    types::compute::{
        Address, AddressList, BackendService, BackendServiceList, Disk, DiskList,
        DisksResizeRequest, Firewall, FirewallList, Instance, InstanceList,
        InstancesSetMachineTypeRequest, InstancesSetServiceAccountRequest, Metadata,
        OperationResponse, Router, Scheduling, ShieldedInstanceConfig, Snapshot, SnapshotList,
        SslPolicy, Subnetwork,
    },
};

/// Client for the Compute Engine API
pub struct ComputeClient<'a> {
    ops: ComputeOps<'a>,
}

impl<'a> ComputeClient<'a> {
    /// Create a new Compute API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: ComputeOps::new(client),
        }
    }

    // ── Disks ──────────────────────────────────────────────────────────

    /// Get a disk by name.
    pub async fn get_disk(&self, project: &str, zone: &str, disk: &str) -> Result<Disk> {
        self.ops.get_disk(project, zone, disk).await
    }

    /// List disks in a zone.
    pub async fn list_disks(&self, project: &str, zone: &str) -> Result<DiskList> {
        self.ops.list_disks(project, zone, "").await
    }

    /// Stream all disks in a zone, automatically handling pagination.
    pub fn list_disks_stream(
        &self,
        project: &str,
        zone: &str,
    ) -> impl futures::Stream<Item = Result<Disk>> + '_ {
        let project = project.to_string();
        let zone = zone.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_disks(&project, &zone, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create a disk (blocks until complete).
    pub async fn create_disk(&self, project: &str, zone: &str, disk: &Disk) -> Result<()> {
        let op = self.create_disk_start(project, zone, disk).await?;
        op.wait().await
    }

    /// Create a disk (returns operation for manual polling).
    pub async fn create_disk_start(
        &self,
        project: &str,
        zone: &str,
        disk: &Disk,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.create_disk(project, zone, disk).await?;
        self.zonal_operation(op_response)
    }

    /// Delete a disk (blocks until complete).
    pub async fn delete_disk(&self, project: &str, zone: &str, disk: &str) -> Result<()> {
        let op = self.delete_disk_start(project, zone, disk).await?;
        op.wait().await
    }

    /// Delete a disk (returns operation for manual polling).
    pub async fn delete_disk_start(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.delete_disk(project, zone, disk).await?;
        self.zonal_operation(op_response)
    }

    // ── Snapshots ─────────────────────────────────────────────────────

    /// Get a snapshot by name.
    pub async fn get_snapshot(&self, project: &str, snapshot: &str) -> Result<Snapshot> {
        self.ops.get_snapshot(project, snapshot).await
    }

    /// List snapshots in a project.
    pub async fn list_snapshots(&self, project: &str) -> Result<SnapshotList> {
        self.ops.list_snapshots(project, "").await
    }

    /// Stream all snapshots in a project, automatically handling pagination.
    pub fn list_snapshots_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<Snapshot>> + '_ {
        let project = project.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_snapshots(&project, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create a snapshot from a disk (blocks until complete).
    pub async fn create_snapshot(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        snapshot_name: &str,
    ) -> Result<()> {
        let op = self
            .create_snapshot_start(project, zone, disk, snapshot_name)
            .await?;
        op.wait().await
    }

    /// Create a snapshot from a disk (returns operation for manual polling).
    pub async fn create_snapshot_start(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        snapshot_name: &str,
    ) -> Result<Operation<'a>> {
        let body = Snapshot {
            name: snapshot_name.to_string(),
            ..Default::default()
        };
        let op_response = self.ops.create_snapshot(project, zone, disk, &body).await?;
        self.zonal_operation(op_response)
    }

    /// Delete a snapshot (blocks until complete).
    pub async fn delete_snapshot(&self, project: &str, snapshot: &str) -> Result<()> {
        let op = self.delete_snapshot_start(project, snapshot).await?;
        op.wait().await
    }

    /// Delete a snapshot (returns operation for manual polling).
    pub async fn delete_snapshot_start(
        &self,
        project: &str,
        snapshot: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.delete_snapshot(project, snapshot).await?;
        self.global_operation(op_response)
    }

    // ── Instances ─────────────────────────────────────────────────────

    /// Get an instance by name.
    pub async fn get_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Instance> {
        self.ops.get_instance(project, zone, instance).await
    }

    /// List instances in a zone.
    pub async fn list_instances(&self, project: &str, zone: &str) -> Result<InstanceList> {
        self.ops.list_instances(project, zone, "").await
    }

    /// Stream all instances in a zone, automatically handling pagination.
    pub fn list_instances_stream(
        &self,
        project: &str,
        zone: &str,
    ) -> impl futures::Stream<Item = Result<Instance>> + '_ {
        let project = project.to_string();
        let zone = zone.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_instances(&project, &zone, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create an instance (blocks until complete).
    pub async fn create_instance(
        &self,
        project: &str,
        zone: &str,
        instance: &Instance,
    ) -> Result<()> {
        let op = self.create_instance_start(project, zone, instance).await?;
        op.wait().await
    }

    /// Create an instance (returns operation for manual polling).
    pub async fn create_instance_start(
        &self,
        project: &str,
        zone: &str,
        instance: &Instance,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.create_instance(project, zone, instance).await?;
        self.zonal_operation(op_response)
    }

    /// Delete an instance (blocks until complete).
    pub async fn delete_instance(&self, project: &str, zone: &str, instance: &str) -> Result<()> {
        let op = self.delete_instance_start(project, zone, instance).await?;
        op.wait().await
    }

    /// Delete an instance (returns operation for manual polling).
    pub async fn delete_instance_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.delete_instance(project, zone, instance).await?;
        self.zonal_operation(op_response)
    }

    /// Start a stopped instance (blocks until complete).
    pub async fn start_instance(&self, project: &str, zone: &str, instance: &str) -> Result<()> {
        let op = self.start_instance_start(project, zone, instance).await?;
        op.wait().await
    }

    /// Start a stopped instance (returns operation for manual polling).
    pub async fn start_instance_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.start_instance(project, zone, instance).await?;
        self.zonal_operation(op_response)
    }

    /// Stop a running instance (blocks until complete).
    pub async fn stop_instance(&self, project: &str, zone: &str, instance: &str) -> Result<()> {
        let op = self.stop_instance_start(project, zone, instance).await?;
        op.wait().await
    }

    /// Stop a running instance (returns operation for manual polling).
    pub async fn stop_instance_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.stop_instance(project, zone, instance).await?;
        self.zonal_operation(op_response)
    }

    /// Reset an instance (hard reset, blocks until complete).
    pub async fn reset_instance(&self, project: &str, zone: &str, instance: &str) -> Result<()> {
        let op = self.reset_instance_start(project, zone, instance).await?;
        op.wait().await
    }

    /// Reset an instance (returns operation for manual polling).
    pub async fn reset_instance_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.reset_instance(project, zone, instance).await?;
        self.zonal_operation(op_response)
    }

    // ── Addresses ──────────────────────────────────────────────────────

    /// Get an address by name.
    pub async fn get_address(&self, project: &str, region: &str, address: &str) -> Result<Address> {
        self.ops.get_address(project, region, address).await
    }

    /// List addresses in a region.
    pub async fn list_addresses(&self, project: &str, region: &str) -> Result<AddressList> {
        self.ops.list_addresses(project, region, "").await
    }

    /// Stream all addresses in a region, automatically handling pagination.
    pub fn list_addresses_stream(
        &self,
        project: &str,
        region: &str,
    ) -> impl futures::Stream<Item = Result<Address>> + '_ {
        let project = project.to_string();
        let region = region.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_addresses(&project, &region, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Release (delete) a static IP address (blocks until complete).
    pub async fn release_address(&self, project: &str, region: &str, address: &str) -> Result<()> {
        let op = self.release_address_start(project, region, address).await?;
        op.wait().await
    }

    /// Release (delete) a static IP address (returns operation for manual polling).
    pub async fn release_address_start(
        &self,
        project: &str,
        region: &str,
        address: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.release_address(project, region, address).await?;
        self.regional_operation(op_response)
    }

    // ── Routers / NAT ────────────────────────────────────────────────

    /// Get a Cloud Router.
    pub async fn get_router(&self, project: &str, region: &str, router: &str) -> Result<Router> {
        self.ops.get_router(project, region, router).await
    }

    /// Patch a Cloud Router (blocks until complete).
    pub async fn patch_router(
        &self,
        project: &str,
        region: &str,
        router: &str,
        body: &Router,
    ) -> Result<()> {
        let op = self
            .patch_router_start(project, region, router, body)
            .await?;
        op.wait().await
    }

    /// Patch a Cloud Router (returns operation for manual polling).
    ///
    /// If the body's `name` is empty, it is set to `router` to avoid GCP rejecting
    /// the empty string (`Router.name` is a required `String`, not `Option`).
    pub async fn patch_router_start(
        &self,
        project: &str,
        region: &str,
        router: &str,
        body: &Router,
    ) -> Result<Operation<'a>> {
        let mut patched = body.clone();
        if patched.name.is_empty() {
            patched.name = router.to_string();
        }
        let op_response = self
            .ops
            .patch_router(project, region, router, &patched)
            .await?;
        self.regional_operation(op_response)
    }

    /// Delete a NAT gateway from a Cloud Router (blocks until complete).
    ///
    /// This is a convenience method that:
    /// 1. Gets the current router configuration
    /// 2. Removes the named NAT from the nats list
    /// 3. Patches the router with the updated configuration
    pub async fn delete_nat_gateway(
        &self,
        project: &str,
        region: &str,
        router: &str,
        nat_name: &str,
    ) -> Result<()> {
        let op = self
            .delete_nat_gateway_start(project, region, router, nat_name)
            .await?;
        op.wait().await
    }

    /// Delete a NAT gateway from a Cloud Router (returns operation for manual polling).
    pub async fn delete_nat_gateway_start(
        &self,
        project: &str,
        region: &str,
        router: &str,
        nat_name: &str,
    ) -> Result<Operation<'a>> {
        // Use raw JSON to preserve all router fields during the round-trip
        let base_url = "https://compute.googleapis.com/compute/v1";
        let url = format!(
            "{}/projects/{}/regions/{}/routers/{}",
            base_url,
            urlencoding::encode(project),
            urlencoding::encode(region),
            urlencoding::encode(router),
        );

        // Step 1: Get current router config as raw JSON
        let response = self.ops.client.get(&url).await?;
        let mut router_json: serde_json::Value =
            serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
                message: format!("Failed to parse router response: {}", e),
                body: Some(String::from_utf8_lossy(&response).to_string()),
            })?;

        // Step 2: Remove the NAT from the nats array
        let mut found = false;
        if let Some(nats) = router_json.get_mut("nats").and_then(|v| v.as_array_mut()) {
            let initial_len = nats.len();
            nats.retain(|nat| {
                nat.get("name")
                    .and_then(|n| n.as_str())
                    .map(|n| n != nat_name)
                    .unwrap_or(true)
            });
            found = nats.len() < initial_len;
        }

        if !found {
            return Err(crate::GcpError::NotFound {
                resource: format!("NAT gateway '{}' in router '{}'", nat_name, router),
                method: "delete_nat_gateway".to_string(),
            });
        }

        // Step 3: Patch the router with updated config
        let patch_response = self.ops.client.patch(&url, &router_json).await?;
        let op_response: OperationResponse =
            serde_json::from_slice(&patch_response).map_err(|e| {
                crate::GcpError::InvalidResponse {
                    message: format!("Failed to parse operation response: {}", e),
                    body: Some(String::from_utf8_lossy(&patch_response).to_string()),
                }
            })?;
        self.regional_operation(op_response)
    }

    // ── Instance Extensions ──────────────────────────────────────────

    /// Remove an access config (external IP) from an instance (blocks until complete).
    pub async fn remove_access_config(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        access_config: &str,
        network_interface: &str,
    ) -> Result<()> {
        let op = self
            .remove_access_config_start(project, zone, instance, access_config, network_interface)
            .await?;
        op.wait().await
    }

    /// Remove an access config (returns operation for manual polling).
    pub async fn remove_access_config_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        access_config: &str,
        network_interface: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .remove_access_config(project, zone, instance, access_config, network_interface)
            .await?;
        self.instance_operation(op_response)
    }

    /// Set the auto-delete flag for a disk attached to an instance (blocks until complete).
    ///
    /// When `auto_delete` is `true`, the disk is deleted when the instance is deleted.
    /// When `false`, the disk persists independently after instance deletion.
    /// The `device_name` is the disk's device name within the instance (e.g. `persistent-disk-0`),
    /// visible in [`AttachedDisk::device_name`] on the instance resource.
    pub async fn set_disk_auto_delete(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        device_name: &str,
        auto_delete: bool,
    ) -> Result<()> {
        let op = self
            .set_disk_auto_delete_start(project, zone, instance, device_name, auto_delete)
            .await?;
        op.wait().await
    }

    /// Set the auto-delete flag for a disk attached to an instance (returns operation for manual polling).
    pub async fn set_disk_auto_delete_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        device_name: &str,
        auto_delete: bool,
    ) -> Result<Operation<'a>> {
        let auto_delete_str = if auto_delete { "true" } else { "false" };
        let op_response = self
            .ops
            .set_disk_auto_delete(project, zone, instance, auto_delete_str, device_name)
            .await?;
        self.instance_operation(op_response)
    }

    /// Set the machine type for an instance (blocks until complete).
    ///
    /// The instance must be stopped before changing its machine type.
    pub async fn set_machine_type(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        machine_type: &str,
    ) -> Result<()> {
        let op = self
            .set_machine_type_start(project, zone, instance, machine_type)
            .await?;
        op.wait().await
    }

    /// Set the machine type for an instance (returns operation for manual polling).
    pub async fn set_machine_type_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        machine_type: &str,
    ) -> Result<Operation<'a>> {
        let body = InstancesSetMachineTypeRequest {
            machine_type: format!("zones/{}/machineTypes/{}", zone, machine_type),
        };
        let op_response = self
            .ops
            .set_machine_type(project, zone, instance, &body)
            .await?;
        self.instance_operation(op_response)
    }

    // ── Disk Extensions ──────────────────────────────────────────────

    /// Resize a disk (blocks until complete).
    ///
    /// The new size must be larger than the current size.
    pub async fn resize_disk(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        new_size_gb: u64,
    ) -> Result<()> {
        let op = self
            .resize_disk_start(project, zone, disk, new_size_gb)
            .await?;
        op.wait().await
    }

    /// Resize a disk (returns operation for manual polling).
    pub async fn resize_disk_start(
        &self,
        project: &str,
        zone: &str,
        disk: &str,
        new_size_gb: u64,
    ) -> Result<Operation<'a>> {
        let body = DisksResizeRequest {
            size_gb: Some(new_size_gb.to_string()),
        };
        let op_response = self.ops.resize_disk(project, zone, disk, &body).await?;
        self.zonal_operation(op_response)
    }

    /// Create a disk from a snapshot (blocks until complete).
    pub async fn create_disk_from_snapshot(
        &self,
        project: &str,
        zone: &str,
        disk_name: &str,
        snapshot: &str,
        disk_type: Option<&str>,
        size_gb: Option<u64>,
    ) -> Result<()> {
        let op = self
            .create_disk_from_snapshot_start(project, zone, disk_name, snapshot, disk_type, size_gb)
            .await?;
        op.wait().await
    }

    /// Create a disk from a snapshot (returns operation for manual polling).
    pub async fn create_disk_from_snapshot_start(
        &self,
        project: &str,
        zone: &str,
        disk_name: &str,
        snapshot: &str,
        disk_type: Option<&str>,
        size_gb: Option<u64>,
    ) -> Result<Operation<'a>> {
        let disk = Disk {
            name: disk_name.to_string(),
            source_snapshot: Some(format!(
                "projects/{}/global/snapshots/{}",
                project, snapshot
            )),
            disk_type: disk_type
                .map(|dt| format!("projects/{}/zones/{}/diskTypes/{}", project, zone, dt)),
            size_gb: size_gb.map(|s| s.to_string()),
            ..Default::default()
        };
        let op_response = self.ops.create_disk(project, zone, &disk).await?;
        self.zonal_operation(op_response)
    }

    // ── Backend Services (Global) ───────────────────────────────────

    /// Get a global backend service.
    pub async fn get_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
    ) -> Result<BackendService> {
        self.ops
            .get_global_backend_service(project, backend_service)
            .await
    }

    /// List global backend services.
    pub async fn list_global_backend_services(&self, project: &str) -> Result<BackendServiceList> {
        self.ops.list_global_backend_services(project, "").await
    }

    /// Stream all global backend services, automatically handling pagination.
    pub fn list_global_backend_services_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<BackendService>> + '_ {
        let project = project.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_global_backend_services(&project, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Create a global backend service (blocks until complete).
    pub async fn create_global_backend_service(
        &self,
        project: &str,
        body: &BackendService,
    ) -> Result<()> {
        let op = self
            .create_global_backend_service_start(project, body)
            .await?;
        op.wait().await
    }

    /// Create a global backend service (returns operation for manual polling).
    pub async fn create_global_backend_service_start(
        &self,
        project: &str,
        body: &BackendService,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .create_global_backend_service(project, body)
            .await?;
        self.global_operation(op_response)
    }

    /// Delete a global backend service (blocks until complete).
    pub async fn delete_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
    ) -> Result<()> {
        let op = self
            .delete_global_backend_service_start(project, backend_service)
            .await?;
        op.wait().await
    }

    /// Delete a global backend service (returns operation for manual polling).
    pub async fn delete_global_backend_service_start(
        &self,
        project: &str,
        backend_service: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .delete_global_backend_service(project, backend_service)
            .await?;
        self.global_operation(op_response)
    }

    /// Patch a global backend service (blocks until complete).
    pub async fn patch_global_backend_service(
        &self,
        project: &str,
        backend_service: &str,
        body: &BackendService,
    ) -> Result<()> {
        let op = self
            .patch_global_backend_service_start(project, backend_service, body)
            .await?;
        op.wait().await
    }

    /// Patch a global backend service (returns operation for manual polling).
    ///
    /// If the body's `name` is empty, it is set to `backend_service` to avoid
    /// GCP rejecting the empty string.
    pub async fn patch_global_backend_service_start(
        &self,
        project: &str,
        backend_service: &str,
        body: &BackendService,
    ) -> Result<Operation<'a>> {
        let mut patched = body.clone();
        if patched.name.is_empty() {
            patched.name = backend_service.to_string();
        }
        let op_response = self
            .ops
            .patch_global_backend_service(project, backend_service, &patched)
            .await?;
        self.global_operation(op_response)
    }

    // ── Backend Services (Regional) ─────────────────────────────────

    /// Get a regional backend service.
    pub async fn get_regional_backend_service(
        &self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> Result<BackendService> {
        self.ops
            .get_regional_backend_service(project, region, backend_service)
            .await
    }

    /// List regional backend services.
    pub async fn list_regional_backend_services(
        &self,
        project: &str,
        region: &str,
    ) -> Result<BackendServiceList> {
        self.ops
            .list_regional_backend_services(project, region, "")
            .await
    }

    /// Stream all regional backend services, automatically handling pagination.
    pub fn list_regional_backend_services_stream(
        &self,
        project: &str,
        region: &str,
    ) -> impl futures::Stream<Item = Result<BackendService>> + '_ {
        let project = project.to_string();
        let region = region.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_regional_backend_services(&project, &region, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Delete a regional backend service (blocks until complete).
    pub async fn delete_regional_backend_service(
        &self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> Result<()> {
        let op = self
            .delete_regional_backend_service_start(project, region, backend_service)
            .await?;
        op.wait().await
    }

    /// Delete a regional backend service (returns operation for manual polling).
    pub async fn delete_regional_backend_service_start(
        &self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .delete_regional_backend_service(project, region, backend_service)
            .await?;
        self.regional_operation(op_response)
    }

    /// Delete a backend service (convenience: auto-detects global vs regional).
    pub async fn delete_backend_service(
        &self,
        project: &str,
        backend_service: &str,
        region: Option<&str>,
    ) -> Result<()> {
        match region {
            Some(r) => {
                self.delete_regional_backend_service(project, r, backend_service)
                    .await
            }
            None => {
                self.delete_global_backend_service(project, backend_service)
                    .await
            }
        }
    }

    // ── Instance Scheduling ──────────────────────────────────────────

    /// Set scheduling options for an instance (blocks until complete).
    ///
    /// The instance must be stopped before changing scheduling options.
    pub async fn set_scheduling(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        scheduling: &Scheduling,
    ) -> Result<()> {
        let op = self
            .set_scheduling_start(project, zone, instance, scheduling)
            .await?;
        op.wait().await
    }

    /// Set scheduling options for an instance (returns operation for manual polling).
    pub async fn set_scheduling_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        scheduling: &Scheduling,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .set_scheduling(project, zone, instance, scheduling)
            .await?;
        self.instance_operation(op_response)
    }

    // ── Firewalls ───────────────────────────────────────────────────

    /// Get a firewall rule by name.
    pub async fn get_firewall(&self, project: &str, firewall: &str) -> Result<Firewall> {
        self.ops.get_firewall(project, firewall).await
    }

    /// List firewall rules in a project.
    pub async fn list_firewalls(&self, project: &str) -> Result<FirewallList> {
        self.ops.list_firewalls(project, "").await
    }

    /// Stream all firewall rules in a project, automatically handling pagination.
    pub fn list_firewalls_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<Firewall>> + '_ {
        let project = project.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_firewalls(&project, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.items { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Delete a firewall rule (blocks until complete).
    pub async fn delete_firewall(&self, project: &str, firewall: &str) -> Result<()> {
        let op = self.delete_firewall_start(project, firewall).await?;
        op.wait().await
    }

    /// Delete a firewall rule (returns operation for manual polling).
    pub async fn delete_firewall_start(
        &self,
        project: &str,
        firewall: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.delete_firewall(project, firewall).await?;
        self.global_operation(op_response)
    }

    /// Patch (update) a firewall rule (blocks until complete).
    pub async fn patch_firewall(
        &self,
        project: &str,
        firewall: &str,
        body: &Firewall,
    ) -> Result<()> {
        let op = self.patch_firewall_start(project, firewall, body).await?;
        op.wait().await
    }

    /// Patch (update) a firewall rule (returns operation for manual polling).
    ///
    /// If the body's `name` is empty, it is set to `firewall` to avoid GCP rejecting
    /// the empty string.
    pub async fn patch_firewall_start(
        &self,
        project: &str,
        firewall: &str,
        body: &Firewall,
    ) -> Result<Operation<'a>> {
        let mut patched = body.clone();
        if patched.name.is_empty() {
            patched.name = firewall.to_string();
        }
        let op_response = self.ops.patch_firewall(project, firewall, &patched).await?;
        self.global_operation(op_response)
    }

    // ── Instance Metadata ─────────────────────────────────────────────

    /// Set metadata on an instance (blocks until complete).
    ///
    /// Used to manage SSH keys (CIS 4.3), enable OS Login (CIS 4.4), and
    /// disable serial port access (CIS 4.5).
    /// The `fingerprint` must match the current metadata fingerprint —
    /// obtain it from `get_instance().metadata.fingerprint`.
    pub async fn set_instance_metadata(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        metadata: &Metadata,
    ) -> Result<()> {
        let op = self
            .set_instance_metadata_start(project, zone, instance, metadata)
            .await?;
        op.wait().await
    }

    /// Set metadata on an instance (returns operation for manual polling).
    pub async fn set_instance_metadata_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        metadata: &Metadata,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .set_instance_metadata(project, zone, instance, metadata)
            .await?;
        self.instance_operation(op_response)
    }

    /// Set project-level common instance metadata (blocks until complete).
    ///
    /// Used to enable OS Login project-wide (CIS 4.4).
    /// The `fingerprint` must match the current project metadata fingerprint.
    pub async fn set_project_common_instance_metadata(
        &self,
        project: &str,
        metadata: &Metadata,
    ) -> Result<()> {
        let op = self
            .set_project_common_instance_metadata_start(project, metadata)
            .await?;
        op.wait().await
    }

    /// Set project-level common instance metadata (returns operation for manual polling).
    pub async fn set_project_common_instance_metadata_start(
        &self,
        project: &str,
        metadata: &Metadata,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .set_project_common_instance_metadata(project, metadata)
            .await?;
        self.global_operation(op_response)
    }

    /// Set the service account on an instance (blocks until complete).
    ///
    /// Used to remediate CIS 4.1 (default SA) and 4.2 (full API scope).
    /// The instance must be stopped before changing its service account.
    pub async fn set_instance_service_account(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        email: &str,
        scopes: Vec<String>,
    ) -> Result<()> {
        let op = self
            .set_instance_service_account_start(project, zone, instance, email, scopes)
            .await?;
        op.wait().await
    }

    /// Set the service account on an instance (returns operation for manual polling).
    pub async fn set_instance_service_account_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        email: &str,
        scopes: Vec<String>,
    ) -> Result<Operation<'a>> {
        let body = InstancesSetServiceAccountRequest {
            email: email.to_string(),
            scopes,
        };
        let op_response = self
            .ops
            .set_instance_service_account(project, zone, instance, &body)
            .await?;
        self.instance_operation(op_response)
    }

    /// Update the Shielded VM config for an instance (blocks until complete).
    ///
    /// Used to enable vTPM, Secure Boot, and integrity monitoring (CIS 4.8).
    /// The instance must be stopped before changing Shielded VM config.
    pub async fn update_shielded_instance_config(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        config: &ShieldedInstanceConfig,
    ) -> Result<()> {
        let op = self
            .update_shielded_instance_config_start(project, zone, instance, config)
            .await?;
        op.wait().await
    }

    /// Update the Shielded VM config for an instance (returns operation for manual polling).
    pub async fn update_shielded_instance_config_start(
        &self,
        project: &str,
        zone: &str,
        instance: &str,
        config: &ShieldedInstanceConfig,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .update_shielded_instance_config(project, zone, instance, config)
            .await?;
        self.instance_operation(op_response)
    }

    // ── Networks ─────────────────────────────────────────────────────

    /// Delete a VPC network (blocks until complete).
    ///
    /// Used to remove the default VPC network (CIS 3.1) or legacy VPC (CIS 3.2).
    /// All subnetworks and routes must be deleted before deleting the network.
    pub async fn delete_network(&self, project: &str, network: &str) -> Result<()> {
        let op = self.delete_network_start(project, network).await?;
        op.wait().await
    }

    /// Delete a VPC network (returns operation for manual polling).
    pub async fn delete_network_start(
        &self,
        project: &str,
        network: &str,
    ) -> Result<Operation<'a>> {
        let op_response = self.ops.delete_network(project, network).await?;
        self.global_operation(op_response)
    }

    // ── Subnetworks ──────────────────────────────────────────────────

    /// Patch a subnetwork (blocks until complete).
    ///
    /// Used to enable VPC flow logs (CIS 3.8).
    /// The `fingerprint` in the body must match the current subnetwork fingerprint.
    pub async fn patch_subnetwork(
        &self,
        project: &str,
        region: &str,
        subnetwork: &str,
        body: &Subnetwork,
    ) -> Result<()> {
        let op = self
            .patch_subnetwork_start(project, region, subnetwork, body)
            .await?;
        op.wait().await
    }

    /// Patch a subnetwork (returns operation for manual polling).
    pub async fn patch_subnetwork_start(
        &self,
        project: &str,
        region: &str,
        subnetwork: &str,
        body: &Subnetwork,
    ) -> Result<Operation<'a>> {
        let op_response = self
            .ops
            .patch_subnetwork(project, region, subnetwork, body)
            .await?;
        self.regional_operation(op_response)
    }

    // ── SSL Policies ─────────────────────────────────────────────────

    /// List all SSL policies in a project (auto-paginated).
    ///
    /// Used to check for weak TLS ciphers/protocols (CIS 3.9).
    pub async fn list_ssl_policies(&self, project: &str) -> Result<Vec<SslPolicy>> {
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_ssl_policies(project, &page_token, "").await?;
            all.extend(resp.items);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Patch an SSL policy (blocks until complete).
    ///
    /// Used to remediate weak TLS cipher suites (CIS 3.9).
    /// If the body's `name` is empty, it is set to `ssl_policy`.
    pub async fn patch_ssl_policy(
        &self,
        project: &str,
        ssl_policy: &str,
        body: &SslPolicy,
    ) -> Result<()> {
        let op = self
            .patch_ssl_policy_start(project, ssl_policy, body)
            .await?;
        op.wait().await
    }

    /// Patch an SSL policy (returns operation for manual polling).
    pub async fn patch_ssl_policy_start(
        &self,
        project: &str,
        ssl_policy: &str,
        body: &SslPolicy,
    ) -> Result<Operation<'a>> {
        let mut patched = body.clone();
        if patched.name.is_empty() {
            patched.name = ssl_policy.to_string();
        }
        let op_response = self
            .ops
            .patch_ssl_policy(project, ssl_policy, &patched)
            .await?;
        self.global_operation(op_response)
    }

    // ── Helpers ───────────────────────────────────────────────────────

    fn zonal_operation(&self, op: OperationResponse) -> Result<Operation<'a>> {
        let initially_done = op.status.as_deref() == Some("DONE");
        let url = op
            .self_link
            .ok_or_else(|| crate::GcpError::InvalidResponse {
                message: "Operation response missing selfLink".to_string(),
                body: None,
            })?;
        let config = PollConfig::disk_operation();
        Ok(Operation::new(
            self.ops.client,
            url,
            config.initial_interval(),
            config.timeout(),
            initially_done,
        ))
    }

    fn global_operation(&self, op: OperationResponse) -> Result<Operation<'a>> {
        let initially_done = op.status.as_deref() == Some("DONE");
        let url = op
            .self_link
            .ok_or_else(|| crate::GcpError::InvalidResponse {
                message: "Operation response missing selfLink".to_string(),
                body: None,
            })?;
        let config = PollConfig::disk_operation();
        Ok(Operation::new(
            self.ops.client,
            url,
            config.initial_interval(),
            config.timeout(),
            initially_done,
        ))
    }

    fn regional_operation(&self, op: OperationResponse) -> Result<Operation<'a>> {
        let initially_done = op.status.as_deref() == Some("DONE");
        let url = op
            .self_link
            .ok_or_else(|| crate::GcpError::InvalidResponse {
                message: "Operation response missing selfLink".to_string(),
                body: None,
            })?;
        let config = PollConfig::network_operation();
        Ok(Operation::new(
            self.ops.client,
            url,
            config.initial_interval(),
            config.timeout(),
            initially_done,
        ))
    }

    fn instance_operation(&self, op: OperationResponse) -> Result<Operation<'a>> {
        let initially_done = op.status.as_deref() == Some("DONE");
        let url = op
            .self_link
            .ok_or_else(|| crate::GcpError::InvalidResponse {
                message: "Operation response missing selfLink".to_string(),
                body: None,
            })?;
        let config = PollConfig::instance_operation();
        Ok(Operation::new(
            self.ops.client,
            url,
            config.initial_interval(),
            config.timeout(),
            initially_done,
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_disks_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        // Page 2 (more specific, registered first)
        mock.expect_get("/compute/v1/projects/p/zones/z/disks?pageToken=tok2")
            .returning_json(json!({
                "items": [{"name": "disk-3", "status": "READY"}]
            }));

        // Page 1
        mock.expect_get("/compute/v1/projects/p/zones/z/disks")
            .returning_json(json!({
                "items": [
                    {"name": "disk-1", "status": "READY"},
                    {"name": "disk-2", "status": "READY"}
                ],
                "nextPageToken": "tok2"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let compute = client.compute();
        let stream = compute.list_disks_stream("p", "z");
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(disk)) = stream.next().await {
            names.push(disk.name);
        }
        assert_eq!(names, vec!["disk-1", "disk-2", "disk-3"]);
    }

    #[tokio::test]
    async fn test_list_instances_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/zones/z/instances?pageToken=next")
            .returning_json(json!({
                "items": [{"name": "vm-2", "status": "RUNNING"}]
            }));

        mock.expect_get("/compute/v1/projects/p/zones/z/instances")
            .returning_json(json!({
                "items": [{"name": "vm-1", "status": "RUNNING"}],
                "nextPageToken": "next"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let compute = client.compute();
        let stream = compute.list_instances_stream("p", "z");
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(inst)) = stream.next().await {
            names.push(inst.name);
        }
        assert_eq!(names, vec!["vm-1", "vm-2"]);
    }

    #[tokio::test]
    async fn test_list_snapshots_stream_single_page() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/global/snapshots")
            .returning_json(json!({
                "items": [
                    {"name": "snap-1", "status": "READY"},
                    {"name": "snap-2", "status": "READY"}
                ]
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let compute = client.compute();
        let stream = compute.list_snapshots_stream("p");
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(snap)) = stream.next().await {
            names.push(snap.name);
        }
        assert_eq!(names, vec!["snap-1", "snap-2"]);
    }

    // ── set_disk_auto_delete Tests ───────────────────────────────────

    #[tokio::test]
    async fn test_set_disk_auto_delete_false() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/p/zones/z/instances/i/setDiskAutoDelete?autoDelete=false&deviceName=persistent-disk-0",
        )
        .returning_json(serde_json::json!({
            "name": "op-1",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-1"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .compute()
            .set_disk_auto_delete("p", "z", "i", "persistent-disk-0", false)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_disk_auto_delete_true() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/p/zones/z/instances/i/setDiskAutoDelete?autoDelete=true&deviceName=persistent-disk-0",
        )
        .returning_json(serde_json::json!({
            "name": "op-2",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-2"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .compute()
            .set_disk_auto_delete("p", "z", "i", "persistent-disk-0", true)
            .await;
        assert!(result.is_ok());
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_create_disk_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        // GCP returns status:"DONE" for a no-op create (e.g., disk already exists)
        mock.expect_post("/compute/v1/projects/p/zones/z/disks")
            .returning_json(json!({
                "name": "op-done",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-done",
                "targetLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/disks/my-disk"
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let disk = crate::types::compute::Disk {
            name: "my-disk".to_string(),
            ..Default::default()
        };

        let result = client.compute().create_disk("p", "z", &disk).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_disk_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/p/zones/z/disks/gone-disk")
            .returning_json(json!({
                "name": "op-done",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-done"
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client.compute().delete_disk("p", "z", "gone-disk").await;
        assert!(result.is_ok());
    }

    // ── set_scheduling Tests ────────────────────────────────────────

    #[tokio::test]
    async fn test_set_scheduling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/p/zones/z/instances/vm-1/setScheduling")
            .returning_json(json!({
                "name": "op-sched",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-sched"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let scheduling = crate::types::compute::Scheduling {
            preemptible: Some(true),
            on_host_maintenance: Some("TERMINATE".to_string()),
            automatic_restart: Some(false),
            ..Default::default()
        };
        let result = client
            .compute()
            .set_scheduling("p", "z", "vm-1", &scheduling)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_scheduling_with_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/compute/v1/projects/p/zones/z/instances/vm-1/setScheduling")
            .returning_json(json!({
                "name": "op-sched",
                "status": "RUNNING",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-sched"
            }))
            .times(1);

        mock.expect_get("/compute/v1/projects/p/zones/z/operations/op-sched")
            .returning_json_sequence(vec![
                json!({ "name": "op-sched", "status": "RUNNING" }),
                json!({ "name": "op-sched", "status": "DONE" }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);
        let scheduling = crate::types::compute::Scheduling {
            preemptible: Some(true),
            ..Default::default()
        };
        let result = client
            .compute()
            .set_scheduling("p", "z", "vm-1", &scheduling)
            .await;
        assert!(result.is_ok());
    }

    // ── Firewall Tests ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/global/firewalls/allow-ssh")
            .returning_json(json!({
                "name": "allow-ssh",
                "network": "global/networks/default",
                "direction": "INGRESS",
                "priority": 1000,
                "allowed": [{"IPProtocol": "tcp", "ports": ["22"]}],
                "sourceRanges": ["0.0.0.0/0"]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let fw = client
            .compute()
            .get_firewall("p", "allow-ssh")
            .await
            .unwrap();
        assert_eq!(fw.name, "allow-ssh");
        assert_eq!(
            fw.direction.unwrap(),
            crate::types::compute::FirewallDirection::Ingress
        );
        assert_eq!(fw.priority, Some(1000));
        assert_eq!(fw.allowed.len(), 1);
        assert_eq!(fw.allowed[0].ip_protocol.as_deref(), Some("tcp"));
        assert_eq!(fw.allowed[0].ports, Some(vec!["22".to_string()]));
        assert_eq!(fw.source_ranges, vec!["0.0.0.0/0"]);
    }

    #[tokio::test]
    async fn test_list_firewalls() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/global/firewalls")
            .returning_json(json!({
                "items": [
                    {"name": "allow-ssh", "priority": 1000},
                    {"name": "allow-http", "priority": 1000}
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let list = client.compute().list_firewalls("p").await.unwrap();
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].name, "allow-ssh");
        assert_eq!(list.items[1].name, "allow-http");
    }

    #[tokio::test]
    async fn test_list_firewalls_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/global/firewalls?pageToken=tok2")
            .returning_json(json!({
                "items": [{"name": "fw-3"}]
            }));

        mock.expect_get("/compute/v1/projects/p/global/firewalls")
            .returning_json(json!({
                "items": [{"name": "fw-1"}, {"name": "fw-2"}],
                "nextPageToken": "tok2"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let compute = client.compute();
        let stream = compute.list_firewalls_stream("p");
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(fw)) = stream.next().await {
            names.push(fw.name);
        }
        assert_eq!(names, vec!["fw-1", "fw-2", "fw-3"]);
    }

    #[tokio::test]
    async fn test_delete_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/p/global/firewalls/allow-ssh")
            .returning_json(json!({
                "name": "op-del-fw",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-del-fw"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.compute().delete_firewall("p", "allow-ssh").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_firewall_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/p/global/firewalls/gone-fw")
            .returning_json(json!({
                "name": "op-done",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-done"
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.compute().delete_firewall("p", "gone-fw").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_firewall() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/p/global/firewalls/allow-ssh")
            .returning_json(json!({
                "name": "op-patch-fw",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-patch-fw"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::compute::Firewall {
            name: "allow-ssh".to_string(),
            disabled: Some(true),
            ..Default::default()
        };
        let result = client
            .compute()
            .patch_firewall("p", "allow-ssh", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_firewall_sets_name_if_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/p/global/firewalls/allow-ssh")
            .returning_json(json!({
                "name": "op-patch-fw",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-patch-fw"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        // Empty name in body — should be auto-filled
        let body = crate::types::compute::Firewall {
            name: String::new(),
            priority: Some(500),
            ..Default::default()
        };
        let result = client
            .compute()
            .patch_firewall("p", "allow-ssh", &body)
            .await;
        assert!(result.is_ok());
    }

    // ── set_instance_metadata Tests ─────────────────────────────────

    #[tokio::test]
    async fn test_set_instance_metadata() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/p/zones/z/instances/vm-1/setMetadata",
        )
        .returning_json(json!({
            "name": "op-set-meta",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-set-meta"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let meta = crate::types::compute::Metadata {
            fingerprint: Some("abc123".to_string()),
            items: vec![crate::types::compute::MetadataItem {
                key: Some("serial-port-enable".to_string()),
                value: Some("FALSE".to_string()),
            }],
            ..Default::default()
        };
        let result = client
            .compute()
            .set_instance_metadata("p", "z", "vm-1", &meta)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_project_common_instance_metadata() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/p/setCommonInstanceMetadata",
        )
        .returning_json(json!({
            "name": "op-set-proj-meta",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-set-proj-meta"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let meta = crate::types::compute::Metadata {
            fingerprint: Some("fp-proj".to_string()),
            items: vec![crate::types::compute::MetadataItem {
                key: Some("enable-oslogin".to_string()),
                value: Some("TRUE".to_string()),
            }],
            ..Default::default()
        };
        let result = client
            .compute()
            .set_project_common_instance_metadata("p", &meta)
            .await;
        assert!(result.is_ok());
    }

    // ── set_instance_service_account Tests ──────────────────────────

    #[tokio::test]
    async fn test_set_instance_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/compute/v1/projects/p/zones/z/instances/vm-1/setServiceAccount",
        )
        .returning_json(json!({
            "name": "op-set-sa",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-set-sa"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client
            .compute()
            .set_instance_service_account(
                "p",
                "z",
                "vm-1",
                "sa@project.iam.gserviceaccount.com",
                vec!["https://www.googleapis.com/auth/cloud-platform".to_string()],
            )
            .await;
        assert!(result.is_ok());
    }

    // ── update_shielded_instance_config Tests ───────────────────────

    #[tokio::test]
    async fn test_update_shielded_instance_config() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/compute/v1/projects/p/zones/z/instances/vm-1/updateShieldedInstanceConfig",
        )
        .returning_json(json!({
            "name": "op-shielded",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-shielded"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let config = crate::types::compute::ShieldedInstanceConfig {
            enable_vtpm: Some(true),
            enable_secure_boot: Some(true),
            enable_integrity_monitoring: Some(true),
        };
        let result = client
            .compute()
            .update_shielded_instance_config("p", "z", "vm-1", &config)
            .await;
        assert!(result.is_ok());
    }

    // ── delete_network Tests ─────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_network() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/compute/v1/projects/p/global/networks/legacy-vpc")
            .returning_json(json!({
                "name": "op-del-net",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-del-net"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.compute().delete_network("p", "legacy-vpc").await;
        assert!(result.is_ok());
    }

    // ── patch_subnetwork Tests ───────────────────────────────────────

    #[tokio::test]
    async fn test_patch_subnetwork_flow_logs() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch(
            "/compute/v1/projects/p/regions/us-central1/subnetworks/default",
        )
        .returning_json(json!({
            "name": "op-patch-subnet",
            "status": "DONE",
            "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/regions/us-central1/operations/op-patch-subnet"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::compute::Subnetwork {
            fingerprint: Some("fp-subnet".to_string()),
            log_config: Some(crate::types::compute::SubnetworkLogConfig {
                enable: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        let result = client
            .compute()
            .patch_subnetwork("p", "us-central1", "default", &body)
            .await;
        assert!(result.is_ok());
    }

    // ── list_ssl_policies Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_list_ssl_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/compute/v1/projects/p/global/sslPolicies")
            .returning_json(json!({
                "items": [
                    {
                        "name": "my-ssl-policy",
                        "profile": "MODERN",
                        "minTlsVersion": "TLS_1_2",
                        "id": "12345",
                        "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/sslPolicies/my-ssl-policy"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let result = client.compute().list_ssl_policies("p").await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "my-ssl-policy");
        assert_eq!(result[0].profile.as_deref(), Some("MODERN"));
        assert_eq!(result[0].min_tls_version.as_deref(), Some("TLS_1_2"));
    }

    // ── patch_ssl_policy Tests ───────────────────────────────────────

    #[tokio::test]
    async fn test_patch_ssl_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/p/global/sslPolicies/my-policy")
            .returning_json(json!({
                "name": "op-patch-ssl",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-patch-ssl"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::compute::SslPolicy {
            name: "my-policy".to_string(),
            profile: Some("MODERN".to_string()),
            min_tls_version: Some("TLS_1_2".to_string()),
            ..Default::default()
        };
        let result = client
            .compute()
            .patch_ssl_policy("p", "my-policy", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_ssl_policy_sets_name_if_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/p/global/sslPolicies/target-policy")
            .returning_json(json!({
                "name": "op-patch-ssl",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-patch-ssl"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        // Empty name in body — should be auto-filled
        let body = crate::types::compute::SslPolicy {
            name: String::new(),
            profile: Some("RESTRICTED".to_string()),
            ..Default::default()
        };
        let result = client
            .compute()
            .patch_ssl_policy("p", "target-policy", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_firewall_with_allowed_rules() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/compute/v1/projects/p/global/firewalls/web-fw")
            .returning_json(json!({
                "name": "op-patch-fw",
                "status": "DONE",
                "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/global/operations/op-patch-fw"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let body = crate::types::compute::Firewall {
            name: "web-fw".to_string(),
            allowed: vec![crate::types::compute::FirewallAllowed {
                ip_protocol: Some("tcp".to_string()),
                ports: Some(vec!["80".to_string(), "443".to_string()]),
            }],
            source_ranges: vec!["0.0.0.0/0".to_string()],
            ..Default::default()
        };
        let result = client.compute().patch_firewall("p", "web-fw", &body).await;
        assert!(result.is_ok());
    }
}
