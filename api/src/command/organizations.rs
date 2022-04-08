use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, OrgId, Token};
use reqwest::Method;

pub struct Organizations<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListOrgsResp {
    organizations: Vec<crate::Organization>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetOrgResp {
    organization: crate::Organization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrgRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateOrgRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrgResponse {
    id: OrgId,
}

impl<'a> Organizations<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Organizations { client, token }
    }

    pub async fn list(&self) -> crate::Result<Vec<crate::Organization>> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!("{}/resources/v1/organizations", self.client.base_url),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListOrgsResp = resp.json().await?;

        Ok(result.organizations)
    }

    pub async fn get(&self, id: OrgId) -> crate::Result<crate::Organization> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetOrgResp = resp.json().await?;

        Ok(result.organization)
    }

    pub async fn create(&self, name: String) -> crate::Result<OrgId> {
        let req = authenticated_request(
            self.client,
            Method::POST,
            self.token,
            format!("{}/resources/v1/organizations", self.client.base_url),
        )
        .json(&CreateOrgRequest { name });

        let resp = default_error_handler(req.send().await?).await?;

        let result: CreateOrgResponse = resp.json().await?;

        Ok(result.id)
    }

    pub async fn update(&self, id: OrgId, name: String) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::PUT,
            self.token,
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id),
        )
        .json(&UpdateOrgRequest { name });

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn delete(&self, id: OrgId) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::DELETE,
            self.token,
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }
}
