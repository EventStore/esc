use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, Network, NetworkId, OrgId, ProjectId, Provider, Token};
use hyper::Uri;

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
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/networks",
            self.client.base_url, org_id, project_id
        )
        .parse()?;

        let payload = serde_json::to_vec(&params)?;
        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let resp: CreateNetworkResponse = resp_json_payload(&mut resp).await?;

        Ok(resp.id)
    }

    pub async fn update(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
        params: UpdateNetworkParams,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
            self.client.base_url, org_id, project_id, network_id
        )
        .parse()?;

        let payload = serde_json::to_vec(&params)?;
        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn delete(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
            self.client.base_url, org_id, project_id, network_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn get(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        network_id: NetworkId,
    ) -> crate::Result<Network> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/networks/{}",
            self.client.base_url, org_id, project_id, network_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: GetNetworksResponse = resp_json_payload(&mut resp).await?;

        Ok(result.network)
    }

    pub async fn list(self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Network>> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/networks",
            self.client.base_url, org_id, project_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: ListNetworksResponse = resp_json_payload(&mut resp).await?;

        Ok(result.networks)
    }
}
