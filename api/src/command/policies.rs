use crate::http::{
    authenticated_request, default_error_handler, req_json_payload, resp_json_payload,
};
use crate::{Client, OrgId, Policy, PolicyId, Token};
use hyper::{Body, Uri};

pub struct Policies<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Policies<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Policies { client, token }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreatePolicyResponse {
    id: PolicyId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyParams {
    pub name: String,
    pub subjects: Vec<String>,
    pub resources: Vec<String>,
    pub actions: Vec<String>,
    pub effect: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponse {
    pub policy: Policy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesResponse {
    pub policies: Vec<Policy>,
}

impl<'a> Policies<'a> {
    pub async fn create(
        &self,
        org_id: OrgId,
        params: CreatePolicyParams,
    ) -> crate::Result<PolicyId> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/policies",
            self.client.base_url, org_id
        )
        .parse()?;

        let payload = json!({ "policy": params });
        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&payload)?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: CreatePolicyResponse = resp_json_payload(&mut resp).await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        policy_id: PolicyId,
        params: UpdatePolicyParams,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/policies/{}",
            self.client.base_url, org_id, policy_id
        )
        .parse()?;

        let payload = json!({ "policy": params });
        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&payload)?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn delete(&self, org_id: OrgId, policy_id: PolicyId) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/policies/{}",
            self.client.base_url, org_id, policy_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .header("Content-Type", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn get(&self, org_id: OrgId, policy_id: PolicyId) -> crate::Result<Policy> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/policies/{}",
            self.client.base_url, org_id, policy_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .header("Content-Type", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: GetPolicyResponse = resp_json_payload(&mut resp).await?;

        Ok(result.policy)
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<Policy>> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/policies",
            self.client.base_url, org_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: ListPoliciesResponse = resp_json_payload(&mut resp).await?;

        Ok(result.policies)
    }
}
