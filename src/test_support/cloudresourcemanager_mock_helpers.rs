//! MockClient helpers for Cloud Resource Manager API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Resource Manager helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudresourcemanagerMockHelpers {
    /// Helper to expect `get_project`: Retrieves the project identified by the specified `name`
    /// (for example, `projects/415104041262`). The caller must have `resourcemanager.projects.get`
    /// permission for this project.
    fn expect_get_project(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_projects`: Lists projects that are direct children of the specified
    /// folder or organization resource. `list()` provides a strongly consistent view of the
    /// projects underneath the specified parent resource. `list()` returns projects sorted based
    /// upon the (ascending) lexical ordering of their `display_name`. The caller must have
    /// `resourcemanager.projects.list` permission on the identified parent.
    fn expect_list_projects(&mut self, parent: &str, page_token: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `search_projects`: Search for projects that the caller has the
    /// `resourcemanager.projects.get` permission on, and also satisfy the specified query. This
    /// method returns projects in an unspecified order. This method is eventually consistent with
    /// project mutations; this means that a newly created project may not appear in the results or
    /// recent updates to an existing project may not be reflected in the results. To retrieve the
    /// latest state of a project, use the GetProject method.
    fn expect_search_projects(&mut self, query: &str, page_token: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_project`: Request that a new project be created. The result is an
    /// `Operation` which can be used to track the creation process. This process usually takes a
    /// few seconds, but can sometimes take much longer. The tracking `Operation` is automatically
    /// deleted after a few hours, so there is no need to call `DeleteOperation`.
    fn expect_create_project(&mut self) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_project`: Marks the project identified by the specified `name` (for
    /// example, `projects/415104041262`) for deletion. This method will only affect the project if
    /// it has a lifecycle state of ACTIVE. This method changes the Project's lifecycle state from
    /// ACTIVE to DELETE_REQUESTED. The deletion starts at an unspecified time, at which point the
    /// Project is no longer accessible. Until the deletion completes, you can check the lifecycle
    /// state checked by retrieving the project with GetProject, and the project remains visible to
    /// ListProjects. However, you cannot update the project. After the deletion completes, the
    /// project is not retrievable by the GetProject, ListProjects, and SearchProjects methods. The
    /// caller must have `resourcemanager.projects.delete` permissions for this project.
    fn expect_delete_project(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `undelete_project`: Restores the project identified by the specified `name`
    /// (for example, `projects/415104041262`). You can only use this method for a project that has
    /// a lifecycle state of DELETE_REQUESTED. After deletion starts, the project cannot be
    /// restored. The caller must have `resourcemanager.projects.undelete` permission for this
    /// project.
    fn expect_undelete_project(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `move_project`: Move a project to another place in your resource hierarchy,
    /// under a new resource parent. Returns an operation which can be used to track the process of
    /// the project move workflow. Upon success, the `Operation.response` field will be populated
    /// with the moved project. The caller must have `resourcemanager.projects.move` permission on
    /// the project, on the project's current and proposed new parent. If project has no current
    /// parent, or it currently does not have an associated organization resource, you will also
    /// need the `resourcemanager.projects.setIamPolicy` permission in the project.
    fn expect_move_project(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_project`: Updates the `display_name` and labels of the project
    /// identified by the specified `name` (for example, `projects/415104041262`). Deleting all
    /// labels requires an update mask for labels field. The caller must have
    /// `resourcemanager.projects.update` permission for this project.
    fn expect_update_project(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_iam_policy`: Returns the IAM access control policy for the specified
    /// project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123. Permission is
    /// denied if the policy or the resource do not exist.
    fn expect_get_iam_policy(&mut self, resource: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_iam_policy`: Sets the IAM access control policy for the specified
    /// project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123. CAUTION: This
    /// method will replace the existing policy, and cannot be used to append additional IAM
    /// settings. Note: Removing service accounts from policies or changing their roles can render
    /// services completely inoperable. It is important to understand how the service account is
    /// being used before removing or updating its roles. The following constraints apply when using
    /// `setIamPolicy()`: + Project does not support `allUsers` and `allAuthenticatedUsers` as
    /// `members` in a `Binding` of a `Policy`. + The owner role can be granted to a `user`,
    /// `serviceAccount`, or a group that is part of an organization. For example,
    /// group@myownpersonaldomain.com could be added as an owner to a project in the
    /// myownpersonaldomain.com organization, but not the examplepetstore.com organization. +
    /// Service accounts can be made owners of a project directly without any restrictions. However,
    /// to be added as an owner, a user must be invited using the Cloud Platform console and must
    /// accept the invitation. + A user cannot be granted the owner role using `setIamPolicy()`. The
    /// user must be granted the owner role using the Cloud Platform Console and must explicitly
    /// accept the invitation. + Invitations to grant the owner role cannot be sent using
    /// `setIamPolicy()`; they must be sent only using the Cloud Platform Console. + If the project
    /// is not part of an organization, there must be at least one owner who has accepted the Terms
    /// of Service (ToS) agreement in the policy. Calling `setIamPolicy()` to remove the last ToS-
    /// accepted owner from the policy will fail. This restriction also applies to legacy projects
    /// that no longer have owners who have accepted the ToS. Edits to IAM policies will be rejected
    /// until the lack of a ToS-accepting owner is rectified. If the project is part of an
    /// organization, you can remove all owners, potentially making the organization inaccessible.
    fn expect_set_iam_policy(&mut self, resource: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `test_iam_permissions`: Returns permissions that a caller has on the
    /// specified project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123..
    fn expect_test_iam_permissions(&mut self, resource: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudresourcemanagerMockHelpers for MockClient {
    /// Helper to expect `get_project`: Retrieves the project identified by the specified `name`
    /// (for example, `projects/415104041262`). The caller must have `resourcemanager.projects.get`
    /// permission for this project.
    fn expect_get_project(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_projects`: Lists projects that are direct children of the specified
    /// folder or organization resource. `list()` provides a strongly consistent view of the
    /// projects underneath the specified parent resource. `list()` returns projects sorted based
    /// upon the (ascending) lexical ordering of their `display_name`. The caller must have
    /// `resourcemanager.projects.list` permission on the identified parent.
    fn expect_list_projects(
        &mut self,
        parent: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/v3/projects".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !parent.is_empty() {
            __qp.push(format!("parent={}", parent));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `search_projects`: Search for projects that the caller has the
    /// `resourcemanager.projects.get` permission on, and also satisfy the specified query. This
    /// method returns projects in an unspecified order. This method is eventually consistent with
    /// project mutations; this means that a newly created project may not appear in the results or
    /// recent updates to an existing project may not be reflected in the results. To retrieve the
    /// latest state of a project, use the GetProject method.
    fn expect_search_projects(
        &mut self,
        query: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = "/v3/projects:search".to_string();
        let mut __qp: Vec<String> = Vec::new();
        if !query.is_empty() {
            __qp.push(format!("query={}", query));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_project`: Request that a new project be created. The result is an
    /// `Operation` which can be used to track the creation process. This process usually takes a
    /// few seconds, but can sometimes take much longer. The tracking `Operation` is automatically
    /// deleted after a few hours, so there is no need to call `DeleteOperation`.
    fn expect_create_project(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/v3/projects".to_string();
        self.expect_post(&path)
    }

    /// Helper to expect `delete_project`: Marks the project identified by the specified `name` (for
    /// example, `projects/415104041262`) for deletion. This method will only affect the project if
    /// it has a lifecycle state of ACTIVE. This method changes the Project's lifecycle state from
    /// ACTIVE to DELETE_REQUESTED. The deletion starts at an unspecified time, at which point the
    /// Project is no longer accessible. Until the deletion completes, you can check the lifecycle
    /// state checked by retrieving the project with GetProject, and the project remains visible to
    /// ListProjects. However, you cannot update the project. After the deletion completes, the
    /// project is not retrievable by the GetProject, ListProjects, and SearchProjects methods. The
    /// caller must have `resourcemanager.projects.delete` permissions for this project.
    fn expect_delete_project(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_delete(&path)
    }

    /// Helper to expect `undelete_project`: Restores the project identified by the specified `name`
    /// (for example, `projects/415104041262`). You can only use this method for a project that has
    /// a lifecycle state of DELETE_REQUESTED. After deletion starts, the project cannot be
    /// restored. The caller must have `resourcemanager.projects.undelete` permission for this
    /// project.
    fn expect_undelete_project(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}:undelete");
        self.expect_post(&path)
    }

    /// Helper to expect `move_project`: Move a project to another place in your resource hierarchy,
    /// under a new resource parent. Returns an operation which can be used to track the process of
    /// the project move workflow. Upon success, the `Operation.response` field will be populated
    /// with the moved project. The caller must have `resourcemanager.projects.move` permission on
    /// the project, on the project's current and proposed new parent. If project has no current
    /// parent, or it currently does not have an associated organization resource, you will also
    /// need the `resourcemanager.projects.setIamPolicy` permission in the project.
    fn expect_move_project(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}:move");
        self.expect_post(&path)
    }

    /// Helper to expect `update_project`: Updates the `display_name` and labels of the project
    /// identified by the specified `name` (for example, `projects/415104041262`). Deleting all
    /// labels requires an update mask for labels field. The caller must have
    /// `resourcemanager.projects.update` permission for this project.
    fn expect_update_project(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{name}");
        self.expect_patch(&path)
    }

    /// Helper to expect `get_iam_policy`: Returns the IAM access control policy for the specified
    /// project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123. Permission is
    /// denied if the policy or the resource do not exist.
    fn expect_get_iam_policy(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{resource}:getIamPolicy");
        self.expect_post(&path)
    }

    /// Helper to expect `set_iam_policy`: Sets the IAM access control policy for the specified
    /// project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123. CAUTION: This
    /// method will replace the existing policy, and cannot be used to append additional IAM
    /// settings. Note: Removing service accounts from policies or changing their roles can render
    /// services completely inoperable. It is important to understand how the service account is
    /// being used before removing or updating its roles. The following constraints apply when using
    /// `setIamPolicy()`: + Project does not support `allUsers` and `allAuthenticatedUsers` as
    /// `members` in a `Binding` of a `Policy`. + The owner role can be granted to a `user`,
    /// `serviceAccount`, or a group that is part of an organization. For example,
    /// group@myownpersonaldomain.com could be added as an owner to a project in the
    /// myownpersonaldomain.com organization, but not the examplepetstore.com organization. +
    /// Service accounts can be made owners of a project directly without any restrictions. However,
    /// to be added as an owner, a user must be invited using the Cloud Platform console and must
    /// accept the invitation. + A user cannot be granted the owner role using `setIamPolicy()`. The
    /// user must be granted the owner role using the Cloud Platform Console and must explicitly
    /// accept the invitation. + Invitations to grant the owner role cannot be sent using
    /// `setIamPolicy()`; they must be sent only using the Cloud Platform Console. + If the project
    /// is not part of an organization, there must be at least one owner who has accepted the Terms
    /// of Service (ToS) agreement in the policy. Calling `setIamPolicy()` to remove the last ToS-
    /// accepted owner from the policy will fail. This restriction also applies to legacy projects
    /// that no longer have owners who have accepted the ToS. Edits to IAM policies will be rejected
    /// until the lack of a ToS-accepting owner is rectified. If the project is part of an
    /// organization, you can remove all owners, potentially making the organization inaccessible.
    fn expect_set_iam_policy(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{resource}:setIamPolicy");
        self.expect_post(&path)
    }

    /// Helper to expect `test_iam_permissions`: Returns permissions that a caller has on the
    /// specified project, in the format `projects/{ProjectIdOrNumber}` e.g. projects/123..
    fn expect_test_iam_permissions(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v3/{resource}:testIamPermissions");
        self.expect_post(&path)
    }
}
