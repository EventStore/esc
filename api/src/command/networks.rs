use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, Network, NetworkId, OrgId, ProjectId, Provider, Token};
use reqwest::Method;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkParams {
    pub provider: Provider,
    pub cidr_block: String,
    pub description: String,
    pub region: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateNetworkResponse {
    id: NetworkId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNetworkParams {
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworksResponse {
    pub network: Network,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNetworksResponse {
    pub networks: Vec<Network>,
}

pub struct Networks<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Networks<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Networks { client, token }
    }

    pub async fn create(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreateNetworkParams,
    ) -> crate::Result<NetworkId> {
        let req = authenticated_request(
            self.client,
            Method::POST,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/networks",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let resp: CreateNetworkResponse = resp.json().await?;

        Ok(resp.id)
    }

    pub async fn update(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
        params: UpdateNetworkParams,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
                self.client.base_url, org_id, project_id, network_id
            ),
        )
        .json(&params);

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn delete(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
                self.client.base_url, org_id, project_id, network_id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
    ) -> crate::Result<Network> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
                self.client.base_url, org_id, project_id, network_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetNetworksResponse = resp.json().await?;

        Ok(result.network)
    }

    pub async fn list(self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Network>> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/networks",
                self.client.base_url, org_id, project_id
            ),
        );

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListNetworksResponse = resp.json().await?;

        Ok(result.networks)
    }
}
