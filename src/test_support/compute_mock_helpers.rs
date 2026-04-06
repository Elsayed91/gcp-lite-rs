//! MockClient helpers for Compute Engine API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Compute Engine helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ComputeMockHelpers {
    /// Helper to expect `create_disk`: Creates a persistent disk in the specified project using the
    /// data in the request. You can create a disk from a source (sourceImage, sourceSnapshot,
    /// orsourceDisk) or create an empty 500 GB data disk by omitting all properties. You can also
    /// create a disk that is larger than the default size by specifying the sizeGb property.
    fn expect_create_disk(&mut self, project: &str, zone: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_disk`: Deletes the specified persistent disk. Deleting a disk
    /// removes its data permanently and is irreversible. However, deleting a disk does not delete
    /// any snapshots previously made from the disk. You must separatelydelete snapshots.
    fn expect_delete_disk(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_disk`: Returns the specified persistent disk.
    fn expect_get_disk(&mut self, project: &str, zone: &str, disk: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_disks`: Retrieves a list of persistent disks contained within the
    /// specified zone.
    fn expect_list_disks(
        &mut self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_snapshot`: Creates a snapshot of a specified persistent disk. For
    /// regular snapshot creation, consider using snapshots.insert instead, as that method supports
    /// more features, such as creating snapshots in a project different from the source disk
    /// project.
    fn expect_create_snapshot(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_instance`: Creates an instance resource in the specified project
    /// using the data included in the request.
    fn expect_create_instance(&mut self, project: &str, zone: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_instance`: Deletes the specified Instance resource. For more
    /// information, seeDeleting an instance.
    fn expect_delete_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_instance`: Returns the specified Instance resource.
    fn expect_get_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_instances`: Retrieves the list of instances contained within the
    /// specified zone.
    fn expect_list_instances(
        &mut self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_instance`: Starts an instance that was stopped using
    /// theinstances().stop method. For more information, seeRestart an instance.
    fn expect_start_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_instance`: Stops a running instance, shutting it down cleanly, and
    /// allows you to restart the instance at a later time. Stopped instances do not incur VM usage
    /// charges while they are stopped. However, resources that the VM is using, such as persistent
    /// disks and static IP addresses, will continue to be charged until they are deleted. For more
    /// information, seeStopping an instance.
    fn expect_stop_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `reset_instance`: Performs a reset on the instance. This is a hard reset.
    /// The VM does not do a graceful shutdown. For more information, seeResetting an instance.
    fn expect_reset_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_snapshot`: Deletes the specified Snapshot resource. Keep in mind
    /// that deleting a single snapshot might not necessarily delete all the data on that snapshot.
    /// If any data on the snapshot that is marked for deletion is needed for subsequent snapshots,
    /// the data will be moved to the next corresponding snapshot. For more information, seeDeleting
    /// snapshots.
    fn expect_delete_snapshot(&mut self, project: &str, snapshot: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_snapshot`: Returns the specified Snapshot resource.
    fn expect_get_snapshot(&mut self, project: &str, snapshot: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_snapshots`: Retrieves the list of Snapshot resources contained within
    /// the specified project.
    fn expect_list_snapshots(&mut self, project: &str, page_token: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `release_address`: Deletes the specified address resource.
    fn expect_release_address(
        &mut self,
        project: &str,
        region: &str,
        address: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_address`: Returns the specified address resource.
    fn expect_get_address(
        &mut self,
        project: &str,
        region: &str,
        address: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_addresses`: Retrieves a list of addresses contained within the
    /// specified region.
    fn expect_list_addresses(
        &mut self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_router`: Returns the specified Router resource.
    fn expect_get_router(
        &mut self,
        project: &str,
        region: &str,
        router: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_router`: Patches the specified Router resource with the data
    /// included in the request. This method supportsPATCH semantics and usesJSON merge patch format
    /// and processing rules.
    fn expect_patch_router(
        &mut self,
        project: &str,
        region: &str,
        router: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `remove_access_config`: Deletes an access config from an instance's network
    /// interface.
    fn expect_remove_access_config(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
        access_config: &str,
        network_interface: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_disk_auto_delete`: Sets the auto-delete flag for a disk attached to an
    /// instance.
    fn expect_set_disk_auto_delete(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
        auto_delete: &str,
        device_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_machine_type`: Changes the machine type for a stopped instance to the
    /// machine type specified in the request.
    fn expect_set_machine_type(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `resize_disk`: Resizes the specified persistent disk. You can only increase
    /// the size of the disk.
    fn expect_resize_disk(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_global_backend_service`: Returns the specified BackendService
    /// resource.
    fn expect_get_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_global_backend_services`: Retrieves the list of BackendService
    /// resources available to the specified project.
    fn expect_list_global_backend_services(
        &mut self,
        project: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_global_backend_service`: Creates a BackendService resource in the
    /// specified project using the data included in the request. For more information, see Backend
    /// services overview.
    fn expect_create_global_backend_service(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_global_backend_service`: Deletes the specified BackendService
    /// resource.
    fn expect_delete_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_global_backend_service`: Patches the specified BackendService
    /// resource with the data included in the request. For more information, see Backend services
    /// overview. This method supports PATCH semantics and uses the JSON merge patch format and
    /// processing rules.
    fn expect_patch_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_regional_backend_service`: Returns the specified regional
    /// BackendService resource.
    fn expect_get_regional_backend_service(
        &mut self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_regional_backend_services`: Retrieves the list of regional
    /// BackendService resources available to the specified project in the given region.
    fn expect_list_regional_backend_services(
        &mut self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_regional_backend_service`: Deletes the specified regional
    /// BackendService resource.
    fn expect_delete_regional_backend_service(
        &mut self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_scheduling`: Sets an instance's scheduling options. You can only call
    /// this method on astopped instance, that is, a VM instance that is in a `TERMINATED` state.
    /// SeeInstance Life Cycle for more information on the possible instance states. For more
    /// information about setting scheduling options for a VM, seeSet VM host maintenance policy.
    fn expect_set_scheduling(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_firewall`: Returns the specified firewall.
    fn expect_get_firewall(&mut self, project: &str, firewall: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_firewalls`: Retrieves the list of firewall rules available to the
    /// specified project.
    fn expect_list_firewalls(&mut self, project: &str, page_token: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_firewall`: Deletes the specified firewall.
    fn expect_delete_firewall(&mut self, project: &str, firewall: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_firewall`: Updates the specified firewall rule with the data
    /// included in the request. This method supportsPATCH semantics and uses theJSON merge patch
    /// format and processing rules.
    fn expect_patch_firewall(&mut self, project: &str, firewall: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_instance_metadata`: Sets metadata for the specified instance to the
    /// data included in the request.
    fn expect_set_instance_metadata(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_project_common_instance_metadata`: Sets metadata common to all
    /// instances within the specified project using the data included in the request.
    fn expect_set_project_common_instance_metadata(
        &mut self,
        project: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_instance_service_account`: Sets the service account on the instance.
    /// For more information, readChanging the service account and access scopes for an instance.
    fn expect_set_instance_service_account(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_shielded_instance_config`: Updates the Shielded Instance config for
    /// an instance. You can only use this method on a stopped instance. This method supportsPATCH
    /// semantics and uses theJSON merge patch format and processing rules.
    fn expect_update_shielded_instance_config(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_network`: Deletes the specified network.
    fn expect_delete_network(&mut self, project: &str, network: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_subnetwork`: Patches the specified subnetwork with the data included
    /// in the request. Only certain fields can be updated with a patch request as indicated in the
    /// field descriptions. You must specify the current fingerprint of the subnetwork resource
    /// being patched.
    fn expect_patch_subnetwork(
        &mut self,
        project: &str,
        region: &str,
        subnetwork: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_ssl_policies`: Lists all the SSL policies that have been configured
    /// for the specified project.
    fn expect_list_ssl_policies(
        &mut self,
        project: &str,
        page_token: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_ssl_policy`: Patches the specified SSL policy with the data included
    /// in the request.
    fn expect_patch_ssl_policy(
        &mut self,
        project: &str,
        ssl_policy: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ComputeMockHelpers for MockClient {
    /// Helper to expect `create_disk`: Creates a persistent disk in the specified project using the
    /// data in the request. You can create a disk from a source (sourceImage, sourceSnapshot,
    /// orsourceDisk) or create an empty 500 GB data disk by omitting all properties. You can also
    /// create a disk that is larger than the default size by specifying the sizeGb property.
    fn expect_create_disk(
        &mut self,
        project: &str,
        zone: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/disks");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_disk`: Deletes the specified persistent disk. Deleting a disk
    /// removes its data permanently and is irreversible. However, deleting a disk does not delete
    /// any snapshots previously made from the disk. You must separatelydelete snapshots.
    fn expect_delete_disk(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/disks/{disk}");
        self.expect_delete(&path)
    }

    /// Helper to expect `get_disk`: Returns the specified persistent disk.
    fn expect_get_disk(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/disks/{disk}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_disks`: Retrieves a list of persistent disks contained within the
    /// specified zone.
    fn expect_list_disks(
        &mut self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/zones/{zone}/disks");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_snapshot`: Creates a snapshot of a specified persistent disk. For
    /// regular snapshot creation, consider using snapshots.insert instead, as that method supports
    /// more features, such as creating snapshots in a project different from the source disk
    /// project.
    fn expect_create_snapshot(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/zones/{zone}/disks/{disk}/createSnapshot");
        self.expect_post(&path)
    }

    /// Helper to expect `create_instance`: Creates an instance resource in the specified project
    /// using the data included in the request.
    fn expect_create_instance(
        &mut self,
        project: &str,
        zone: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/instances");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_instance`: Deletes the specified Instance resource. For more
    /// information, seeDeleting an instance.
    fn expect_delete_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}");
        self.expect_delete(&path)
    }

    /// Helper to expect `get_instance`: Returns the specified Instance resource.
    fn expect_get_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_instances`: Retrieves the list of instances contained within the
    /// specified zone.
    fn expect_list_instances(
        &mut self,
        project: &str,
        zone: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/zones/{zone}/instances");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `start_instance`: Starts an instance that was stopped using
    /// theinstances().stop method. For more information, seeRestart an instance.
    fn expect_start_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/start");
        self.expect_post(&path)
    }

    /// Helper to expect `stop_instance`: Stops a running instance, shutting it down cleanly, and
    /// allows you to restart the instance at a later time. Stopped instances do not incur VM usage
    /// charges while they are stopped. However, resources that the VM is using, such as persistent
    /// disks and static IP addresses, will continue to be charged until they are deleted. For more
    /// information, seeStopping an instance.
    fn expect_stop_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/stop");
        self.expect_post(&path)
    }

    /// Helper to expect `reset_instance`: Performs a reset on the instance. This is a hard reset.
    /// The VM does not do a graceful shutdown. For more information, seeResetting an instance.
    fn expect_reset_instance(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/reset");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_snapshot`: Deletes the specified Snapshot resource. Keep in mind
    /// that deleting a single snapshot might not necessarily delete all the data on that snapshot.
    /// If any data on the snapshot that is marked for deletion is needed for subsequent snapshots,
    /// the data will be moved to the next corresponding snapshot. For more information, seeDeleting
    /// snapshots.
    fn expect_delete_snapshot(
        &mut self,
        project: &str,
        snapshot: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/snapshots/{snapshot}");
        self.expect_delete(&path)
    }

    /// Helper to expect `get_snapshot`: Returns the specified Snapshot resource.
    fn expect_get_snapshot(
        &mut self,
        project: &str,
        snapshot: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/snapshots/{snapshot}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_snapshots`: Retrieves the list of Snapshot resources contained within
    /// the specified project.
    fn expect_list_snapshots(
        &mut self,
        project: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/global/snapshots");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `release_address`: Deletes the specified address resource.
    fn expect_release_address(
        &mut self,
        project: &str,
        region: &str,
        address: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/regions/{region}/addresses/{address}");
        self.expect_delete(&path)
    }

    /// Helper to expect `get_address`: Returns the specified address resource.
    fn expect_get_address(
        &mut self,
        project: &str,
        region: &str,
        address: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/regions/{region}/addresses/{address}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_addresses`: Retrieves a list of addresses contained within the
    /// specified region.
    fn expect_list_addresses(
        &mut self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/regions/{region}/addresses");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_router`: Returns the specified Router resource.
    fn expect_get_router(
        &mut self,
        project: &str,
        region: &str,
        router: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/regions/{region}/routers/{router}");
        self.expect_get(&path)
    }

    /// Helper to expect `patch_router`: Patches the specified Router resource with the data
    /// included in the request. This method supportsPATCH semantics and usesJSON merge patch format
    /// and processing rules.
    fn expect_patch_router(
        &mut self,
        project: &str,
        region: &str,
        router: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/regions/{region}/routers/{router}");
        self.expect_patch(&path)
    }

    /// Helper to expect `remove_access_config`: Deletes an access config from an instance's network
    /// interface.
    fn expect_remove_access_config(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
        access_config: &str,
        network_interface: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/deleteAccessConfig"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !access_config.is_empty() {
            __qp.push(format!("accessConfig={}", access_config));
        }
        if !network_interface.is_empty() {
            __qp.push(format!("networkInterface={}", network_interface));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `set_disk_auto_delete`: Sets the auto-delete flag for a disk attached to an
    /// instance.
    fn expect_set_disk_auto_delete(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
        auto_delete: &str,
        device_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/setDiskAutoDelete"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !auto_delete.is_empty() {
            __qp.push(format!("autoDelete={}", auto_delete));
        }
        if !device_name.is_empty() {
            __qp.push(format!("deviceName={}", device_name));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `set_machine_type`: Changes the machine type for a stopped instance to the
    /// machine type specified in the request.
    fn expect_set_machine_type(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/setMachineType"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `resize_disk`: Resizes the specified persistent disk. You can only increase
    /// the size of the disk.
    fn expect_resize_disk(
        &mut self,
        project: &str,
        zone: &str,
        disk: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/zones/{zone}/disks/{disk}/resize");
        self.expect_post(&path)
    }

    /// Helper to expect `get_global_backend_service`: Returns the specified BackendService
    /// resource.
    fn expect_get_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/global/backendServices/{backend_service}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_global_backend_services`: Retrieves the list of BackendService
    /// resources available to the specified project.
    fn expect_list_global_backend_services(
        &mut self,
        project: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/global/backendServices");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_global_backend_service`: Creates a BackendService resource in the
    /// specified project using the data included in the request. For more information, see Backend
    /// services overview.
    fn expect_create_global_backend_service(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/backendServices");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_global_backend_service`: Deletes the specified BackendService
    /// resource.
    fn expect_delete_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/global/backendServices/{backend_service}");
        self.expect_delete(&path)
    }

    /// Helper to expect `patch_global_backend_service`: Patches the specified BackendService
    /// resource with the data included in the request. For more information, see Backend services
    /// overview. This method supports PATCH semantics and uses the JSON merge patch format and
    /// processing rules.
    fn expect_patch_global_backend_service(
        &mut self,
        project: &str,
        backend_service: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/global/backendServices/{backend_service}");
        self.expect_patch(&path)
    }

    /// Helper to expect `get_regional_backend_service`: Returns the specified regional
    /// BackendService resource.
    fn expect_get_regional_backend_service(
        &mut self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/regions/{region}/backendServices/{backend_service}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_regional_backend_services`: Retrieves the list of regional
    /// BackendService resources available to the specified project in the given region.
    fn expect_list_regional_backend_services(
        &mut self,
        project: &str,
        region: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/regions/{region}/backendServices");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `delete_regional_backend_service`: Deletes the specified regional
    /// BackendService resource.
    fn expect_delete_regional_backend_service(
        &mut self,
        project: &str,
        region: &str,
        backend_service: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/regions/{region}/backendServices/{backend_service}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `set_scheduling`: Sets an instance's scheduling options. You can only call
    /// this method on astopped instance, that is, a VM instance that is in a `TERMINATED` state.
    /// SeeInstance Life Cycle for more information on the possible instance states. For more
    /// information about setting scheduling options for a VM, seeSet VM host maintenance policy.
    fn expect_set_scheduling(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/setScheduling"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `get_firewall`: Returns the specified firewall.
    fn expect_get_firewall(
        &mut self,
        project: &str,
        firewall: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/firewalls/{firewall}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_firewalls`: Retrieves the list of firewall rules available to the
    /// specified project.
    fn expect_list_firewalls(
        &mut self,
        project: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/global/firewalls");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `delete_firewall`: Deletes the specified firewall.
    fn expect_delete_firewall(
        &mut self,
        project: &str,
        firewall: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/firewalls/{firewall}");
        self.expect_delete(&path)
    }

    /// Helper to expect `patch_firewall`: Updates the specified firewall rule with the data
    /// included in the request. This method supportsPATCH semantics and uses theJSON merge patch
    /// format and processing rules.
    fn expect_patch_firewall(
        &mut self,
        project: &str,
        firewall: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/firewalls/{firewall}");
        self.expect_patch(&path)
    }

    /// Helper to expect `set_instance_metadata`: Sets metadata for the specified instance to the
    /// data included in the request.
    fn expect_set_instance_metadata(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/setMetadata");
        self.expect_post(&path)
    }

    /// Helper to expect `set_project_common_instance_metadata`: Sets metadata common to all
    /// instances within the specified project using the data included in the request.
    fn expect_set_project_common_instance_metadata(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/setCommonInstanceMetadata");
        self.expect_post(&path)
    }

    /// Helper to expect `set_instance_service_account`: Sets the service account on the instance.
    /// For more information, readChanging the service account and access scopes for an instance.
    fn expect_set_instance_service_account(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/setServiceAccount"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `update_shielded_instance_config`: Updates the Shielded Instance config for
    /// an instance. You can only use this method on a stopped instance. This method supportsPATCH
    /// semantics and uses theJSON merge patch format and processing rules.
    fn expect_update_shielded_instance_config(
        &mut self,
        project: &str,
        zone: &str,
        instance: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/updateShieldedInstanceConfig"
        );
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_network`: Deletes the specified network.
    fn expect_delete_network(
        &mut self,
        project: &str,
        network: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/networks/{network}");
        self.expect_delete(&path)
    }

    /// Helper to expect `patch_subnetwork`: Patches the specified subnetwork with the data included
    /// in the request. Only certain fields can be updated with a patch request as indicated in the
    /// field descriptions. You must specify the current fingerprint of the subnetwork resource
    /// being patched.
    fn expect_patch_subnetwork(
        &mut self,
        project: &str,
        region: &str,
        subnetwork: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetwork}");
        self.expect_patch(&path)
    }

    /// Helper to expect `list_ssl_policies`: Lists all the SSL policies that have been configured
    /// for the specified project.
    fn expect_list_ssl_policies(
        &mut self,
        project: &str,
        page_token: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/compute/v1/projects/{project}/global/sslPolicies");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `patch_ssl_policy`: Patches the specified SSL policy with the data included
    /// in the request.
    fn expect_patch_ssl_policy(
        &mut self,
        project: &str,
        ssl_policy: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/compute/v1/projects/{project}/global/sslPolicies/{ssl_policy}");
        self.expect_patch(&path)
    }
}
