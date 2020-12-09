use crate::http::{authenticated_request, default_error_handler};
use crate::Provider;
use crate::{Client, NetworkId, OrgId, Peering, PeeringId, ProjectId, Token};
use reqwest::Method;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringParams {
    pub network_id: NetworkId,
    pub description: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
    pub peer_network_region: String,
    pub routes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePeeringResponse {
    id: PeeringId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePeeringParams {
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeeringResponse {
    pub peering: Peering,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPeeringsResponse {
    pub peerings: Vec<Peering>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivePeeringCommandsParams {
    pub provider: Provider,
    pub peer_account_id: String,
    pub peer_network_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeeringCommand {
    pub title: String,
    pub language: String,
    pub value: String,
    pub file_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivePeeringCommandsResponse {
    pub commands: Vec<PeeringCommand>,
}

pub struct Peerings<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug)]
pub enum PeeringFailure {
    ConfigurationRequired,
}

impl std::fmt::Display for PeeringFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PeeringFailure::ConfigurationRequired => {
                writeln!(f, "Configuration required on upstream provider")
            }
        }
    }
}

impl std::error::Error for PeeringFailure {}

impl<'a> Peerings<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Peerings { client, token }
    }

    pub async fn create(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreatePeeringParams,
    ) -> crate::Result<Result<PeeringId, PeeringFailure>> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = req.send().await?;

        if resp.status().as_u16() == 412 {
            return Ok(Err(PeeringFailure::ConfigurationRequired));
        }

        let resp = default_error_handler(resp).await?;

        let resp: CreatePeeringResponse = resp.json().await?;

        Ok(Ok(resp.id))
    }

    pub async fn update(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        peering_id: PeeringId,
        params: UpdatePeeringParams,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
                self.client.base_url, org_id, project_id, peering_id
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
        peering_id: PeeringId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
                self.client.base_url, org_id, project_id, peering_id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        peering_id: PeeringId,
    ) -> crate::Result<Peering> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
                self.client.base_url, org_id, project_id, peering_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetPeeringResponse = resp.json().await?;

        Ok(result.peering)
    }

    pub async fn list(self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Peering>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings",
                self.client.base_url, org_id, project_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListPeeringsResponse = resp.json().await?;

        Ok(result.peerings)
    }

    pub async fn derive_peering_commands(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        params: DerivePeeringCommandsParams,
    ) -> crate::Result<Vec<PeeringCommand>> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/infra/v1/organizations/{}/projects/{}/peerings/commands",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let resp: DerivePeeringCommandsResponse = resp.json().await?;

        Ok(resp.commands)
    }
}
