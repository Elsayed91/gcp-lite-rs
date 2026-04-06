//! Operation contracts for the Cloud Resource Manager API (v3).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudresourcemanager.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudresourcemanager::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud Resource Manager API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudresourcemanager::CloudresourcemanagerClient`] instead.
pub struct CloudresourcemanagerOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> CloudresourcemanagerOps<'a> {
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
        "https://cloudresourcemanager.googleapis.com"
    }

    /// Retrieves the project identified by the specified `name` (for example,
    /// `projects/415104041262`). The caller must have `resourcemanager.projects.get` permission
    /// for this project.
    ///
    /// **GCP API**: `GET v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the project (for example, `projects/415104041262`). *(required)*
    ///
    /// # Response
    /// [`Project`]
    #[allow(dead_code)]
    pub(crate) async fn get_project(&self, name: &str) -> Result<Project> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists projects that are direct children of the specified folder or organization
    /// resource. `list()` provides a strongly consistent view of the projects underneath the
    /// specified parent resource. `list()` returns projects sorted based upon the (ascending)
    /// lexical ordering of their `display_name`. The caller must have
    /// `resourcemanager.projects.list` permission on the identified parent.
    ///
    /// **GCP API**: `GET v3/projects`
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. The maximum number of projects to return in the response. The server can return fewer projects than requested.
    /// - `pageToken` — Optional. A pagination token returned from a previous call to ListProjects that indicates from where listing should cont
    /// - `parent` — Required. The name of the parent resource whose projects are being listed. Only children of this parent resource are lis
    /// - `showDeleted` — Optional. Indicate that projects in the `DELETE_REQUESTED` state should also be returned. Normally only `ACTIVE` project
    ///
    /// # Response
    /// [`ListProjectsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_projects(
        &self,
        parent: &str,
        page_token: &str,
    ) -> Result<ListProjectsResponse> {
        let url = format!("{}/v3/projects", self.base_url(),);
        let url = crate::append_query_params(url, &[("parent", parent), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_projects response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Search for projects that the caller has the `resourcemanager.projects.get` permission
    /// on, and also satisfy the specified query. This method returns projects in an unspecified
    /// order. This method is eventually consistent with project mutations; this means that a
    /// newly created project may not appear in the results or recent updates to an existing
    /// project may not be reflected in the results. To retrieve the latest state of a project,
    /// use the GetProject method.
    ///
    /// **GCP API**: `GET v3/projects:search`
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. The maximum number of projects to return in the response. The server can return fewer projects than requested.
    /// - `pageToken` — Optional. A pagination token returned from a previous call to ListProjects that indicates from where listing should cont
    /// - `query` — Optional. A query string for searching for projects that the caller has `resourcemanager.projects.get` permission to. If
    ///
    /// # Response
    /// [`SearchProjectsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn search_projects(
        &self,
        query: &str,
        page_token: &str,
    ) -> Result<SearchProjectsResponse> {
        let url = format!("{}/v3/projects:search", self.base_url(),);
        let url = crate::append_query_params(url, &[("query", query), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse search_projects response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Request that a new project be created. The result is an `Operation` which can be used to
    /// track the creation process. This process usually takes a few seconds, but can sometimes
    /// take much longer. The tracking `Operation` is automatically deleted after a few hours,
    /// so there is no need to call `DeleteOperation`.
    ///
    /// **GCP API**: `POST v3/projects`
    ///
    /// # Request Body
    /// [`Project`]
    ///
    /// # Response
    /// [`ProjectsLro`]
    #[allow(dead_code)]
    pub(crate) async fn create_project(&self, body: &Project) -> Result<ProjectsLro> {
        let url = format!("{}/v3/projects", self.base_url(),);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Marks the project identified by the specified `name` (for example,
    /// `projects/415104041262`) for deletion. This method will only affect the project if it
    /// has a lifecycle state of ACTIVE. This method changes the Project's lifecycle state from
    /// ACTIVE to DELETE_REQUESTED. The deletion starts at an unspecified time, at which point
    /// the Project is no longer accessible. Until the deletion completes, you can check the
    /// lifecycle state checked by retrieving the project with GetProject, and the project
    /// remains visible to ListProjects. However, you cannot update the project. After the
    /// deletion completes, the project is not retrievable by the GetProject, ListProjects, and
    /// SearchProjects methods. The caller must have `resourcemanager.projects.delete`
    /// permissions for this project.
    ///
    /// **GCP API**: `DELETE v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the Project (for example, `projects/415104041262`). *(required)*
    ///
    /// # Response
    /// [`ProjectsLro`]
    #[allow(dead_code)]
    pub(crate) async fn delete_project(&self, name: &str) -> Result<ProjectsLro> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Restores the project identified by the specified `name` (for example,
    /// `projects/415104041262`). You can only use this method for a project that has a
    /// lifecycle state of DELETE_REQUESTED. After deletion starts, the project cannot be
    /// restored. The caller must have `resourcemanager.projects.undelete` permission for this
    /// project.
    ///
    /// **GCP API**: `POST v3/{+name}:undelete`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the project (for example, `projects/415104041262`). Required. *(required)*
    ///
    /// # Request Body
    /// [`UndeleteProjectRequest`]
    ///
    /// # Response
    /// [`ProjectsLro`]
    #[allow(dead_code)]
    pub(crate) async fn undelete_project(
        &self,
        name: &str,
        body: &UndeleteProjectRequest,
    ) -> Result<ProjectsLro> {
        let url = format!("{}/v3/{}:undelete", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse undelete_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Move a project to another place in your resource hierarchy, under a new resource parent.
    /// Returns an operation which can be used to track the process of the project move
    /// workflow. Upon success, the `Operation.response` field will be populated with the moved
    /// project. The caller must have `resourcemanager.projects.move` permission on the project,
    /// on the project's current and proposed new parent. If project has no current parent, or
    /// it currently does not have an associated organization resource, you will also need the
    /// `resourcemanager.projects.setIamPolicy` permission in the project.
    ///
    /// **GCP API**: `POST v3/{+name}:move`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the project to move. *(required)*
    ///
    /// # Request Body
    /// [`MoveProjectRequest`]
    ///
    /// # Response
    /// [`ProjectsLro`]
    #[allow(dead_code)]
    pub(crate) async fn move_project(
        &self,
        name: &str,
        body: &MoveProjectRequest,
    ) -> Result<ProjectsLro> {
        let url = format!("{}/v3/{}:move", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse move_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates the `display_name` and labels of the project identified by the specified `name`
    /// (for example, `projects/415104041262`). Deleting all labels requires an update mask for
    /// labels field. The caller must have `resourcemanager.projects.update` permission for this
    /// project.
    ///
    /// **GCP API**: `PATCH v3/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Output only. The unique resource name of the project. It is an int64 generated number prefixed by "projects/". Example:  *(required)*
    ///
    /// # Query Parameters
    /// - `updateMask` — Optional. An update mask to selectively update fields.
    ///
    /// # Request Body
    /// [`Project`]
    ///
    /// # Response
    /// [`ProjectsLro`]
    #[allow(dead_code)]
    pub(crate) async fn update_project(&self, name: &str, body: &Project) -> Result<ProjectsLro> {
        let url = format!("{}/v3/{}", self.base_url(), name,);
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns the IAM access control policy for the specified project, in the format
    /// `projects/{ProjectIdOrNumber}` e.g. projects/123. Permission is denied if the policy or
    /// the resource do not exist.
    ///
    /// **GCP API**: `POST v3/{+resource}:getIamPolicy`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/desig *(required)*
    ///
    /// # Request Body
    /// [`GetIamPolicyRequest`]
    ///
    /// # Response
    /// [`IamPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn get_iam_policy(
        &self,
        resource: &str,
        body: &GetIamPolicyRequest,
    ) -> Result<IamPolicy> {
        let url = format!("{}/v3/{}:getIamPolicy", self.base_url(), resource,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets the IAM access control policy for the specified project, in the format
    /// `projects/{ProjectIdOrNumber}` e.g. projects/123. CAUTION: This method will replace the
    /// existing policy, and cannot be used to append additional IAM settings. Note: Removing
    /// service accounts from policies or changing their roles can render services completely
    /// inoperable. It is important to understand how the service account is being used before
    /// removing or updating its roles. The following constraints apply when using
    /// `setIamPolicy()`: + Project does not support `allUsers` and `allAuthenticatedUsers` as
    /// `members` in a `Binding` of a `Policy`. + The owner role can be granted to a `user`,
    /// `serviceAccount`, or a group that is part of an organization. For example,
    /// group@myownpersonaldomain.com could be added as an owner to a project in the
    /// myownpersonaldomain.com organization, but not the examplepetstore.com organization. +
    /// Service accounts can be made owners of a project directly without any restrictions.
    /// However, to be added as an owner, a user must be invited using the Cloud Platform
    /// console and must accept the invitation. + A user cannot be granted the owner role using
    /// `setIamPolicy()`. The user must be granted the owner role using the Cloud Platform
    /// Console and must explicitly accept the invitation. + Invitations to grant the owner role
    /// cannot be sent using `setIamPolicy()`; they must be sent only using the Cloud Platform
    /// Console. + If the project is not part of an organization, there must be at least one
    /// owner who has accepted the Terms of Service (ToS) agreement in the policy. Calling
    /// `setIamPolicy()` to remove the last ToS-accepted owner from the policy will fail. This
    /// restriction also applies to legacy projects that no longer have owners who have accepted
    /// the ToS. Edits to IAM policies will be rejected until the lack of a ToS-accepting owner
    /// is rectified. If the project is part of an organization, you can remove all owners,
    /// potentially making the organization inaccessible.
    ///
    /// **GCP API**: `POST v3/{+resource}:setIamPolicy`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/desig *(required)*
    ///
    /// # Request Body
    /// [`SetIamPolicyRequest`]
    ///
    /// # Response
    /// [`IamPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn set_iam_policy(
        &self,
        resource: &str,
        body: &SetIamPolicyRequest,
    ) -> Result<IamPolicy> {
        let url = format!("{}/v3/{}:setIamPolicy", self.base_url(), resource,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns permissions that a caller has on the specified project, in the format
    /// `projects/{ProjectIdOrNumber}` e.g. projects/123..
    ///
    /// **GCP API**: `POST v3/{+resource}:testIamPermissions`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/api *(required)*
    ///
    /// # Request Body
    /// [`TestIamPermissionsRequest`]
    ///
    /// # Response
    /// [`TestIamPermissionsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn test_iam_permissions(
        &self,
        resource: &str,
        body: &TestIamPermissionsRequest,
    ) -> Result<TestIamPermissionsResponse> {
        let url = format!("{}/v3/{}:testIamPermissions", self.base_url(), resource,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse test_iam_permissions response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/test-name")
            .returning_json(serde_json::to_value(Project::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let result = ops.get_project("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_projects() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/projects?parent=test-parent&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListProjectsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let result = ops.list_projects("test-parent", "test-pageToken").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_projects() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/projects:search?query=test-query&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(SearchProjectsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let result = ops.search_projects("test-query", "test-pageToken").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/projects")
            .returning_json(serde_json::to_value(ProjectsLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = Project::fixture();
        let result = ops.create_project(&body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v3/test-name")
            .returning_json(serde_json::to_value(ProjectsLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let result = ops.delete_project("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_undelete_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-name:undelete")
            .returning_json(serde_json::to_value(ProjectsLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = UndeleteProjectRequest::fixture();
        let result = ops.undelete_project("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_move_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-name:move")
            .returning_json(serde_json::to_value(ProjectsLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = MoveProjectRequest::fixture();
        let result = ops.move_project("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/v3/test-name")
            .returning_json(serde_json::to_value(ProjectsLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = Project::fixture();
        let result = ops.update_project("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-resource:getIamPolicy")
            .returning_json(serde_json::to_value(IamPolicy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = GetIamPolicyRequest::fixture();
        let result = ops.get_iam_policy("test-resource", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-resource:setIamPolicy")
            .returning_json(serde_json::to_value(IamPolicy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = SetIamPolicyRequest::fixture();
        let result = ops.set_iam_policy("test-resource", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_iam_permissions() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/test-resource:testIamPermissions")
            .returning_json(serde_json::to_value(TestIamPermissionsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudresourcemanagerOps::new(&client);

        let body = TestIamPermissionsRequest::fixture();
        let result = ops.test_iam_permissions("test-resource", &body).await;
        assert!(result.is_ok());
    }
}
