use crate::http::{authenticated_request, default_error_handler};
use crate::{
    Client, Cluster, ClusterId, NetworkId, OrgId, ProjectId, ProjectionLevel, Token, Topology,
};

use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterParams {
    pub network_id: NetworkId,
    pub description: String,
    pub topology: Topology,
    pub instance_type: String,
    pub disk_size_gb: usize,
    pub disk_type: String,
    pub server_version: String,
    pub projection_level: ProjectionLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throughput: Option<i32>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandDisk {
    pub disk_size_gb: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throughput: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
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
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let resp: CreateClusterResponse = resp.json().await?;

        Ok(resp.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
                self.client.base_url, org_id, project_id, id
            ),
        )
        .header("Content-Type", "application/json");

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<Cluster> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
                self.client.base_url, org_id, project_id, id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetClusterResponse = resp.json().await?;

        Ok(result.cluster)
    }

    pub async fn delete(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: ClusterId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}",
                self.client.base_url, org_id, project_id, id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Cluster>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters",
                self.client.base_url, org_id, project_id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListClustersResponse = resp.json().await?;

        Ok(result.clusters)
    }

    pub async fn expand(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        cluster_id: ClusterId,
        params: ExpandDisk,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/clusters/{}/disk/expand",
                self.client.base_url, org_id, project_id, cluster_id
            ),
        )
        .header("Content-Type", "application/json")
        .json(&params);

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }
}
