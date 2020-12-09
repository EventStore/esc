use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, OrgId, Policy, PolicyId, Token};
use reqwest::Method;

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
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/policies",
                self.client.base_url, org_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let result: CreatePolicyResponse = resp.json().await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        policy_id: PolicyId,
        params: UpdatePolicyParams,
    ) -> crate::Result<()> {
        let payload = json!({ "policy": params });
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/policies/{}",
                self.client.base_url, org_id, policy_id
            ),
        )
        .json(&payload);

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn delete(&self, org_id: OrgId, policy_id: PolicyId) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/policies/{}",
                self.client.base_url, org_id, policy_id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(&self, org_id: OrgId, policy_id: PolicyId) -> crate::Result<Policy> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/policies/{}",
                self.client.base_url, org_id, policy_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetPolicyResponse = resp.json().await?;

        Ok(result.policy)
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<Policy>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/policies",
                self.client.base_url, org_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListPoliciesResponse = resp.json().await?;

        Ok(result.policies)
    }
}
