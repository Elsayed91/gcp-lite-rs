//! Operation contracts for the Cloud KMS API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudkms.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudkms::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud KMS API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudkms::CloudkmsClient`] instead.
pub struct CloudkmsOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> CloudkmsOps<'a> {
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
        "https://cloudkms.googleapis.com"
    }

    /// Lists information about the supported locations for this service. This method can be
    /// called in two ways: * **List all public locations:** Use the path `GET /v1/locations`. *
    /// **List project-visible locations:** Use the path `GET
    /// /v1/projects/{project_id}/locations`. This may include public locations as well as
    /// private or other locations specifically visible to the project.
    ///
    /// **GCP API**: `GET v1/{+name}/locations`
    ///
    /// # Path Parameters
    /// - `name` — The resource that owns the locations collection, if applicable. *(required)*
    ///
    /// # Query Parameters
    /// - `extraLocationTypes` — Optional. Do not use this field. It is unsupported and is ignored unless explicitly documented otherwise. This is primar
    /// - `filter` — A filter to narrow down results to a preferred subset. The filtering language accepts strings like `"displayName=tokyo"`
    /// - `pageSize` — The maximum number of results to return. If not set, the service selects a default.
    /// - `pageToken` — A page token received from the `next_page_token` field in the response. Send that page token to receive the subsequent p
    ///
    /// # Response
    /// [`ListLocationsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_locations(
        &self,
        name: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> Result<ListLocationsResponse> {
        let url = format!("{}/v1/{}/locations", self.base_url(), name,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_locations response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists KeyRings.
    ///
    /// **GCP API**: `GET v1/{+parent}/keyRings`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the location associated with the KeyRings, in the format `projects/*/locations/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering
    /// - `orderBy` — Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. F
    /// - `pageSize` — Optional. Optional limit on the number of KeyRings to include in the response. Further KeyRings can subsequently be obta
    /// - `pageToken` — Optional. Optional pagination token, returned earlier via ListKeyRingsResponse.next_page_token.
    ///
    /// # Response
    /// [`ListKeyRingsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_key_rings(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> Result<ListKeyRingsResponse> {
        let url = format!("{}/v1/{}/keyRings", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_key_rings response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns metadata for a given KeyRing.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the KeyRing to get. *(required)*
    ///
    /// # Response
    /// [`KeyRing`]
    #[allow(dead_code)]
    pub(crate) async fn get_key_ring(&self, name: &str) -> Result<KeyRing> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_key_ring response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists CryptoKeys.
    ///
    /// **GCP API**: `GET v1/{+parent}/cryptoKeys`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the KeyRing to list, in the format `projects/*/locations/*/keyRings/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering
    /// - `orderBy` — Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. F
    /// - `pageSize` — Optional. Optional limit on the number of CryptoKeys to include in the response. Further CryptoKeys can subsequently be
    /// - `pageToken` — Optional. Optional pagination token, returned earlier via ListCryptoKeysResponse.next_page_token.
    /// - `versionView` — The fields of the primary version to include in the response.
    ///
    /// # Response
    /// [`ListCryptoKeysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_crypto_keys(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> Result<ListCryptoKeysResponse> {
        let url = format!("{}/v1/{}/cryptoKeys", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_crypto_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Returns metadata for a given CryptoKey, as well as its primary CryptoKeyVersion.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the CryptoKey to get. *(required)*
    ///
    /// # Response
    /// [`CryptoKey`]
    #[allow(dead_code)]
    pub(crate) async fn get_crypto_key(&self, name: &str) -> Result<CryptoKey> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_crypto_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the access control policy for a resource. Returns an empty policy if the resource
    /// exists and does not have a policy set.
    ///
    /// **GCP API**: `GET v1/{+resource}:getIamPolicy`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/desig *(required)*
    ///
    /// # Query Parameters
    /// - `options.requestedPolicyVersion` — Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests spec
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn get_key_ring_iam_policy(&self, resource: &str) -> Result<Policy> {
        let url = format!("{}/v1/{}:getIamPolicy", self.base_url(), resource,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_key_ring_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the access control policy for a resource. Returns an empty policy if the resource
    /// exists and does not have a policy set.
    ///
    /// **GCP API**: `GET v1/{+resource}:getIamPolicy`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/desig *(required)*
    ///
    /// # Query Parameters
    /// - `options.requestedPolicyVersion` — Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests spec
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn get_crypto_key_iam_policy(&self, resource: &str) -> Result<Policy> {
        let url = format!("{}/v1/{}:getIamPolicy", self.base_url(), resource,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_crypto_key_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    ///
    /// **GCP API**: `POST v1/{+resource}:setIamPolicy`
    ///
    /// # Path Parameters
    /// - `resource` — REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/desig *(required)*
    ///
    /// # Request Body
    /// [`SetIamPolicyRequest`]
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn set_crypto_key_iam_policy(
        &self,
        resource: &str,
        body: &SetIamPolicyRequest,
    ) -> Result<Policy> {
        let url = format!("{}/v1/{}:setIamPolicy", self.base_url(), resource,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse set_crypto_key_iam_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Update a CryptoKey.
    ///
    /// **GCP API**: `PATCH v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `updateMask` — Required. List of fields to be updated in this request.
    ///
    /// # Request Body
    /// [`CryptoKey`]
    ///
    /// # Response
    /// [`CryptoKey`]
    #[allow(dead_code)]
    pub(crate) async fn update_crypto_key(
        &self,
        name: &str,
        update_mask: &str,
        body: &CryptoKey,
    ) -> Result<CryptoKey> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let url = crate::append_query_params(url, &[("updateMask", update_mask)]);
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_crypto_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_locations() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name/locations?pageSize=test-pageSize&pageToken=test-pageToken&filter=test-filter")
            .returning_json(serde_json::to_value(ListLocationsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops
            .list_locations(
                "test-name",
                "test-pageSize",
                "test-pageToken",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_key_rings() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/keyRings?pageSize=test-pageSize&pageToken=test-pageToken&filter=test-filter")
            .returning_json(serde_json::to_value(ListKeyRingsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops
            .list_key_rings(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_key_ring() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(KeyRing::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops.get_key_ring("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_crypto_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/cryptoKeys?pageSize=test-pageSize&pageToken=test-pageToken&filter=test-filter")
            .returning_json(serde_json::to_value(ListCryptoKeysResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops
            .list_crypto_keys(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_crypto_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(CryptoKey::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops.get_crypto_key("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_key_ring_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-resource:getIamPolicy")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops.get_key_ring_iam_policy("test-resource").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_crypto_key_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-resource:getIamPolicy")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let result = ops.get_crypto_key_iam_policy("test-resource").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_crypto_key_iam_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-resource:setIamPolicy")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let body = SetIamPolicyRequest::fixture();
        let result = ops.set_crypto_key_iam_policy("test-resource", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_crypto_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/v1/test-name?updateMask=test-updateMask")
            .returning_json(serde_json::to_value(CryptoKey::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudkmsOps::new(&client);

        let body = CryptoKey::fixture();
        let result = ops
            .update_crypto_key("test-name", "test-updateMask", &body)
            .await;
        assert!(result.is_ok());
    }
}
