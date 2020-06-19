use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, NetworkId, OrgId, Peering, PeeringId, ProjectId, Token};
use hyper::Uri;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringParams {
    pub network_id: NetworkId,
    pub description: String,
    pub peer_account: String,
    pub peer_network: String,
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

pub struct Peerings<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Peerings<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Peerings { client, token }
    }

    pub async fn create(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreatePeeringParams,
    ) -> crate::Result<PeeringId> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/peerings",
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

        let resp: CreatePeeringResponse = resp_json_payload(&mut resp).await?;

        Ok(resp.id)
    }

    pub async fn update(
        self,
        org_id: OrgId,
        project_id: ProjectId,
        peering_id: PeeringId,
        params: UpdatePeeringParams,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
            self.client.base_url, org_id, project_id, peering_id
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
        peering_id: PeeringId,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
            self.client.base_url, org_id, project_id, peering_id
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
        peering_id: PeeringId,
    ) -> crate::Result<Peering> {
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/peerings/{}",
            self.client.base_url, org_id, project_id, peering_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: GetPeeringResponse = resp_json_payload(&mut resp).await?;

        Ok(result.peering)
    }

    pub async fn list(self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Peering>> {
        // TODO - Fix bespin backend to fix that URL path.
        let uri: Uri = format!(
            "{}/infra/v1/organizations/{}/projects/{}/peerings",
            self.client.base_url, org_id, project_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: ListPeeringsResponse = resp_json_payload(&mut resp).await?;

        Ok(result.peerings)
    }
}
