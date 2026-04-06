//! Cloud Resource Manager (Projects) API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::cloudresourcemanager::CloudresourcemanagerOps`. This layer adds:
//! - Ergonomic method signatures
//! - Blocking variants that poll operations to completion
//! - IAM policy read-modify-write convenience methods

use crate::{
    GcpError, GcpHttpClient, Result,
    operation::{PollConfig, ResourceManagerOperation},
    ops::cloudresourcemanager::CloudresourcemanagerOps,
    types::cloudresourcemanager::{
        GetIamPolicyRequest, GetPolicyOptions, IamBinding, IamPolicy, ListProjectsResponse,
        MoveProjectRequest, Project, ProjectsLro, SearchProjectsResponse, SetIamPolicyRequest,
        TestIamPermissionsRequest, TestIamPermissionsResponse, UndeleteProjectRequest,
    },
};

/// Client for the Cloud Resource Manager (Projects) API
pub struct ProjectsClient<'a> {
    ops: CloudresourcemanagerOps<'a>,
}

impl<'a> ProjectsClient<'a> {
    /// Create a new Projects API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: CloudresourcemanagerOps::new(client),
        }
    }

    // ── Query ────────────────────────────────────────────────────────

    /// Get a project by project ID (e.g. `"my-project-123"`).
    pub async fn get_project(&self, project: &str) -> Result<Project> {
        let name = format!("projects/{}", project);
        self.ops.get_project(&name).await
    }

    /// List projects under a parent (e.g. `"folders/123"` or `"organizations/456"`).
    pub async fn list_projects(&self, parent: &str) -> Result<ListProjectsResponse> {
        self.ops.list_projects(parent, "").await
    }

    /// Stream all projects under a parent, automatically handling pagination.
    pub fn list_projects_stream(
        &self,
        parent: &str,
    ) -> impl futures::Stream<Item = Result<Project>> + '_ {
        let parent = parent.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_projects(&parent, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.projects { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Search for projects matching a query (e.g. `"id:my-project"` or `"name:My Project"`).
    pub async fn search_projects(&self, query: &str) -> Result<SearchProjectsResponse> {
        self.ops.search_projects(query, "").await
    }

    /// Stream all projects matching a search query, automatically handling pagination.
    pub fn search_projects_stream(
        &self,
        query: &str,
    ) -> impl futures::Stream<Item = Result<Project>> + '_ {
        let query = query.to_string();
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .search_projects(&query, page_token.as_deref().unwrap_or(""))
                    .await?;
                for item in response.projects { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    // ── IAM Policy ──────────────────────────────────────────────────

    /// Get the IAM policy for a project.
    pub async fn get_iam_policy(&self, project: &str) -> Result<IamPolicy> {
        let resource = format!("projects/{}", project);
        let body = GetIamPolicyRequest {
            options: Some(GetPolicyOptions {
                requested_policy_version: Some(3),
            }),
        };
        self.ops.get_iam_policy(&resource, &body).await
    }

    /// Set the IAM policy for a project (replaces the entire policy).
    pub async fn set_iam_policy(&self, project: &str, policy: &IamPolicy) -> Result<IamPolicy> {
        let resource = format!("projects/{}", project);
        let body = SetIamPolicyRequest {
            policy: Some(policy.clone()),
            update_mask: None,
        };
        self.ops.set_iam_policy(&resource, &body).await
    }

    /// Add an IAM binding to a project's policy.
    ///
    /// Uses read-modify-write with etag-based conflict retry (up to 3 attempts).
    /// If the member already has the role, this is a no-op.
    pub async fn add_iam_policy_binding(
        &self,
        project: &str,
        role: &str,
        member: &str,
    ) -> Result<IamPolicy> {
        for attempt in 0..3 {
            let mut policy = self.get_iam_policy(project).await?;

            // Find existing binding for this role
            let existing = policy
                .bindings
                .iter_mut()
                .find(|b| b.role.as_deref() == Some(role));

            match existing {
                Some(binding) => {
                    // Already has this member? No-op.
                    if binding.members.iter().any(|m| m == member) {
                        return Ok(policy);
                    }
                    binding.members.push(member.to_string());
                }
                None => {
                    // Create new binding
                    policy.bindings.push(IamBinding {
                        role: Some(role.to_string()),
                        members: vec![member.to_string()],
                        condition: None,
                    });
                }
            }

            match self.set_iam_policy(project, &policy).await {
                Ok(updated) => return Ok(updated),
                Err(e) if is_etag_conflict(&e) && attempt < 2 => continue,
                Err(e) => return Err(e),
            }
        }

        unreachable!()
    }

    /// Remove an IAM binding from a project's policy.
    ///
    /// Uses read-modify-write with etag-based conflict retry (up to 3 attempts).
    /// If the member doesn't have the role, this is a no-op.
    pub async fn remove_iam_policy_binding(
        &self,
        project: &str,
        role: &str,
        member: &str,
    ) -> Result<IamPolicy> {
        for attempt in 0..3 {
            let mut policy = self.get_iam_policy(project).await?;

            let binding_idx = policy
                .bindings
                .iter()
                .position(|b| b.role.as_deref() == Some(role));

            match binding_idx {
                Some(idx) => {
                    let binding = &mut policy.bindings[idx];
                    let member_idx = binding.members.iter().position(|m| m == member);
                    match member_idx {
                        Some(mi) => {
                            binding.members.remove(mi);
                            // Remove the whole binding if no members left
                            if binding.members.is_empty() {
                                policy.bindings.remove(idx);
                            }
                        }
                        None => {
                            // Member not in this role — no-op
                            return Ok(policy);
                        }
                    }
                }
                None => {
                    // Role not in policy — no-op
                    return Ok(policy);
                }
            }

            match self.set_iam_policy(project, &policy).await {
                Ok(updated) => return Ok(updated),
                Err(e) if is_etag_conflict(&e) && attempt < 2 => continue,
                Err(e) => return Err(e),
            }
        }

        unreachable!()
    }

    /// Test IAM permissions on a project.
    pub async fn test_iam_permissions(
        &self,
        project: &str,
        permissions: Vec<String>,
    ) -> Result<TestIamPermissionsResponse> {
        let resource = format!("projects/{}", project);
        let body = TestIamPermissionsRequest { permissions };
        self.ops.test_iam_permissions(&resource, &body).await
    }

    // ── Create / Delete / Undelete (LRO) ────────────────────────────

    /// Create a new project (blocks until complete).
    pub async fn create_project(&self, project: &Project) -> Result<()> {
        let op = self.create_project_start(project).await?;
        op.wait().await
    }

    /// Create a new project (returns operation for manual polling).
    pub async fn create_project_start(
        &self,
        project: &Project,
    ) -> Result<ResourceManagerOperation<'a>> {
        let lro = self.ops.create_project(project).await?;
        self.lro_operation(lro)
    }

    /// Delete a project (blocks until complete).
    pub async fn delete_project(&self, project: &str) -> Result<()> {
        let op = self.delete_project_start(project).await?;
        op.wait().await
    }

    /// Delete a project (returns operation for manual polling).
    pub async fn delete_project_start(
        &self,
        project: &str,
    ) -> Result<ResourceManagerOperation<'a>> {
        let name = format!("projects/{}", project);
        let lro = self.ops.delete_project(&name).await?;
        self.lro_operation(lro)
    }

    /// Undelete a project (blocks until complete).
    pub async fn undelete_project(&self, project: &str) -> Result<()> {
        let op = self.undelete_project_start(project).await?;
        op.wait().await
    }

    /// Undelete a project (returns operation for manual polling).
    pub async fn undelete_project_start(
        &self,
        project: &str,
    ) -> Result<ResourceManagerOperation<'a>> {
        let name = format!("projects/{}", project);
        let lro = self
            .ops
            .undelete_project(&name, &UndeleteProjectRequest {})
            .await?;
        self.lro_operation(lro)
    }

    /// Move a project to a new parent (blocks until complete).
    pub async fn move_project(&self, project: &str, destination_parent: &str) -> Result<()> {
        let op = self.move_project_start(project, destination_parent).await?;
        op.wait().await
    }

    /// Move a project to a new parent (returns operation for manual polling).
    pub async fn move_project_start(
        &self,
        project: &str,
        destination_parent: &str,
    ) -> Result<ResourceManagerOperation<'a>> {
        let name = format!("projects/{}", project);
        let body = MoveProjectRequest {
            destination_parent: Some(destination_parent.to_string()),
        };
        let lro = self.ops.move_project(&name, &body).await?;
        self.lro_operation(lro)
    }

    /// Update a project (blocks until complete).
    pub async fn update_project(&self, project: &Project) -> Result<()> {
        let op = self.update_project_start(project).await?;
        op.wait().await
    }

    /// Update a project (returns operation for manual polling).
    pub async fn update_project_start(
        &self,
        project: &Project,
    ) -> Result<ResourceManagerOperation<'a>> {
        let lro = self.ops.update_project(&project.name, project).await?;
        self.lro_operation(lro)
    }

    // ── Helpers ──────────────────────────────────────────────────────

    fn lro_operation(&self, lro: ProjectsLro) -> Result<ResourceManagerOperation<'a>> {
        // If the LRO is already done, check for errors and short-circuit —
        // the operation name may be a placeholder that GCP rejects on poll.
        if lro.done
            && let Some(error) = &lro.error
        {
            let message = error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            let code = error.get("code").and_then(|v| {
                v.as_str()
                    .map(String::from)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });
            return Err(crate::GcpError::OperationFailed {
                operation: lro.name,
                message,
                code,
            });
        }
        let config = PollConfig::project_operation();
        Ok(ResourceManagerOperation::new(
            self.ops.client,
            lro.name,
            config.initial_interval(),
            config.timeout(),
            lro.done,
        ))
    }
}

/// Check if an error is an etag conflict (409 Aborted).
fn is_etag_conflict(error: &GcpError) -> bool {
    match error {
        GcpError::ServerError { status: 409, .. } => true,
        GcpError::InvalidResponse { message, .. } => {
            message.contains("409") || message.contains("Aborted")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_get_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/projects/my-project")
            .returning_json(json!({
                "name": "projects/123456",
                "projectId": "my-project",
                "displayName": "My Project",
                "state": "ACTIVE",
                "parent": "organizations/789"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects.get_project("my-project").await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.project_id, Some("my-project".to_string()));
        assert_eq!(project.display_name, Some("My Project".to_string()));
    }

    #[tokio::test]
    async fn test_get_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/owner",
                        "members": ["user:admin@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects.get_iam_policy("my-project").await;
        assert!(result.is_ok());
        let policy = result.unwrap();
        assert_eq!(policy.version, Some(3));
        assert_eq!(policy.bindings.len(), 1);
        assert_eq!(policy.bindings[0].role, Some("roles/owner".to_string()));
    }

    #[tokio::test]
    async fn test_set_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/projects/my-project:setIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:viewer@example.com"]
                    }
                ],
                "etag": "BwXyz456="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let policy = crate::types::cloudresourcemanager::IamPolicy {
            version: Some(3),
            bindings: vec![crate::types::cloudresourcemanager::IamBinding {
                role: Some("roles/viewer".to_string()),
                members: vec!["user:viewer@example.com".to_string()],
                condition: None,
            }],
            etag: Some("BwXyz123=".to_string()),
            ..Default::default()
        };

        let result = projects.set_iam_policy("my-project", &policy).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_iam_policy_binding_new_role() {
        let mut mock = crate::MockClient::new();

        // get_iam_policy
        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/owner",
                        "members": ["user:admin@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        // set_iam_policy
        mock.expect_post("/v3/projects/my-project:setIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/owner",
                        "members": ["user:admin@example.com"]
                    },
                    {
                        "role": "roles/viewer",
                        "members": ["serviceAccount:test@test.iam.gserviceaccount.com"]
                    }
                ],
                "etag": "BwXyz456="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .add_iam_policy_binding(
                "my-project",
                "roles/viewer",
                "serviceAccount:test@test.iam.gserviceaccount.com",
            )
            .await;
        assert!(result.is_ok());
        let policy = result.unwrap();
        assert_eq!(policy.bindings.len(), 2);
    }

    #[tokio::test]
    async fn test_add_iam_policy_binding_existing_role() {
        let mut mock = crate::MockClient::new();

        // get_iam_policy — already has roles/viewer with one member
        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:existing@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        // set_iam_policy — adds new member to existing binding
        mock.expect_post("/v3/projects/my-project:setIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:existing@example.com", "user:new@example.com"]
                    }
                ],
                "etag": "BwXyz456="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .add_iam_policy_binding("my-project", "roles/viewer", "user:new@example.com")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_iam_policy_binding_already_exists() {
        let mut mock = crate::MockClient::new();

        // get_iam_policy — member already has the role
        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:existing@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        // No set_iam_policy call expected — it's a no-op

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .add_iam_policy_binding("my-project", "roles/viewer", "user:existing@example.com")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_iam_policy_binding() {
        let mut mock = crate::MockClient::new();

        // get_iam_policy
        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:a@example.com", "user:b@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        // set_iam_policy — removes user:a, keeps user:b
        mock.expect_post("/v3/projects/my-project:setIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:b@example.com"]
                    }
                ],
                "etag": "BwXyz456="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .remove_iam_policy_binding("my-project", "roles/viewer", "user:a@example.com")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_iam_policy_binding_empty_removes_binding() {
        let mut mock = crate::MockClient::new();

        // get_iam_policy — sole member in binding
        mock.expect_post("/v3/projects/my-project:getIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [
                    {
                        "role": "roles/viewer",
                        "members": ["user:only@example.com"]
                    }
                ],
                "etag": "BwXyz123="
            }))
            .times(1);

        // set_iam_policy — entire binding removed
        mock.expect_post("/v3/projects/my-project:setIamPolicy")
            .returning_json(json!({
                "version": 3,
                "bindings": [],
                "etag": "BwXyz456="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .remove_iam_policy_binding("my-project", "roles/viewer", "user:only@example.com")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_project_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/projects")
            .returning_json(json!({
                "name": "operations/cp.123456",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v3/operations/cp.123456")
            .returning_json(json!({
                "name": "operations/cp.123456",
                "done": true
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let project = crate::types::cloudresourcemanager::Project {
            name: String::new(),
            project_id: Some("new-test-project".to_string()),
            display_name: Some("New Test Project".to_string()),
            parent: Some("organizations/123".to_string()),
            ..Default::default()
        };

        let op = projects.create_project_start(&project).await;
        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v3/projects/my-project")
            .returning_json(json!({
                "name": "operations/dp.789",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v3/operations/dp.789")
            .returning_json(json!({
                "name": "operations/dp.789",
                "done": true
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects.delete_project("my-project").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_projects_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        // Page 2 (more specific, registered first)
        mock.expect_get("/v3/projects?parent=folders%2F123&pageToken=tok2")
            .returning_json(json!({
                "projects": [
                    {"name": "projects/789", "projectId": "proj-c", "state": "ACTIVE"}
                ]
            }));

        // Page 1
        mock.expect_get("/v3/projects?parent=folders%2F123")
            .returning_json(json!({
                "projects": [
                    {"name": "projects/123", "projectId": "proj-a", "state": "ACTIVE"},
                    {"name": "projects/456", "projectId": "proj-b", "state": "ACTIVE"}
                ],
                "nextPageToken": "tok2"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();
        let stream = projects.list_projects_stream("folders/123");
        futures::pin_mut!(stream);

        let mut ids = Vec::new();
        while let Some(Ok(proj)) = stream.next().await {
            ids.push(proj.project_id.unwrap_or_default());
        }
        assert_eq!(ids, vec!["proj-a", "proj-b", "proj-c"]);
    }

    #[tokio::test]
    async fn test_search_projects_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/v3/projects:search?query=state%3AACTIVE&pageToken=stok")
            .returning_json(json!({
                "projects": [
                    {"name": "projects/999", "projectId": "found-2", "state": "ACTIVE"}
                ]
            }));

        mock.expect_get("/v3/projects:search?query=state%3AACTIVE")
            .returning_json(json!({
                "projects": [
                    {"name": "projects/111", "projectId": "found-1", "state": "ACTIVE"}
                ],
                "nextPageToken": "stok"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();
        let stream = projects.search_projects_stream("state:ACTIVE");
        futures::pin_mut!(stream);

        let mut ids = Vec::new();
        while let Some(Ok(proj)) = stream.next().await {
            ids.push(proj.project_id.unwrap_or_default());
        }
        assert_eq!(ids, vec!["found-1", "found-2"]);
    }

    #[tokio::test]
    async fn test_test_iam_permissions() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v3/projects/my-project:testIamPermissions")
            .returning_json(json!({
                "permissions": ["resourcemanager.projects.get"]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let projects = client.projects();

        let result = projects
            .test_iam_permissions(
                "my-project",
                vec![
                    "resourcemanager.projects.get".to_string(),
                    "resourcemanager.projects.delete".to_string(),
                ],
            )
            .await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.permissions.len(), 1);
        assert_eq!(resp.permissions[0], "resourcemanager.projects.get");
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_delete_already_deleted_project_skips_polling() {
        let mut mock = crate::MockClient::new();

        // GCP returns done:true for deleting an already-deleted project
        mock.expect_delete("/v3/projects/my-project")
            .returning_json(json!({
                "name": "DONE_OPERATION",
                "done": true
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client.projects().delete_project("my-project").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_initially_done_lro_with_error_returns_error() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v3/projects/my-project")
            .returning_json(json!({
                "name": "DONE_OPERATION",
                "done": true,
                "error": {
                    "code": 403,
                    "message": "Permission denied on resource"
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client.projects().delete_project("my-project").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::GcpError::OperationFailed { .. }),
            "Expected OperationFailed, got: {:?}",
            err
        );
    }
}
