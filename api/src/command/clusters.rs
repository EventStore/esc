use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, Cluster, ClusterId, NetworkId, OrgId, ProjectId, Token, Topology};
use hyper::Uri;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterParams {
    #[serde(rename = "organizationId")]
    pub network_id: NetworkId,
    pub description: String,
    pub topology: Topology,
    pub instance_type: String,
    pub disk_size_gb: usize,
    pub disk_type: String,
    pub server_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateClusterResponse {
    id: ClusterId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetClusterResponse {
    cluster: Cluster,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListClustersResponse {
    clusters: Vec<Cluster>,
}

pub struct Clusters<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Clusters<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Clusters { client, token }
    }

    pub async fn create(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreateClusterParams,
    ) -> crate::Result<ClusterId> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/clusters",
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

        let resp: CreateClusterResponse = resp_json_payload(&mut resp).await?;

        Ok(resp.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
            self.client.base_url, org_id, project_id, id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<Cluster> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
            self.client.base_url, org_id, project_id, id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: GetClusterResponse = resp_json_payload(&mut resp).await?;

        Ok(result.cluster)
    }

    pub async fn delete(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
            self.client.base_url, org_id, project_id, id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Cluster>> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/clusters",
            self.client.base_url, org_id, project_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: ListClustersResponse = resp_json_payload(&mut resp).await?;

        Ok(result.clusters)
    }
}
