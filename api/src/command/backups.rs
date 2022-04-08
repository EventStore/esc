use crate::http::{authenticated_request, default_error_handler};
use crate::{Backup, BackupId, Client, ClusterId, OrgId, ProjectId, Token};
use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupParams {
    pub source_cluster_id: ClusterId,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBackupResponse {
    id: BackupId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetBackupResponse {
    backup: Backup,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListBackupsResponse {
    backups: Vec<Backup>,
}

pub struct Backups<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Backups<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Backups { client, token }
    }

    pub async fn create(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreateBackupParams,
    ) -> crate::Result<BackupId> {
        let req = authenticated_request(
            self.client,
            Method::POST,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/backups",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let resp: CreateBackupResponse = resp.json().await?;

        Ok(resp.id)
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: BackupId,
    ) -> crate::Result<Backup> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/backups/{}",
                self.client.base_url, org_id, project_id, id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: GetBackupResponse = resp.json().await?;

        Ok(result.backup)
    }

    pub async fn delete(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: BackupId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/backups/{}",
                self.client.base_url, org_id, project_id, id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Backup>> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/mesdb/v1/organizations/{}/projects/{}/backups",
                self.client.base_url, org_id, project_id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListBackupsResponse = resp.json().await?;

        Ok(result.backups)
    }
}
