use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Backup, BackupId, Client, ClusterId, OrgId, ProjectId, Token};
use hyper::Uri;

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
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/backups",
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

        let resp: CreateBackupResponse = resp_json_payload(&mut resp).await?;

        Ok(resp.id)
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: BackupId,
    ) -> crate::Result<Backup> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/backups/{}",
            self.client.base_url, org_id, project_id, id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: GetBackupResponse = resp_json_payload(&mut resp).await?;

        Ok(result.backup)
    }

    pub async fn delete(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: BackupId,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/backups/{}",
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

    pub async fn list(&self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Backup>> {
        let uri: Uri = format!(
            "{}/mesdb/v1/organizations/{}/projects/{}/backups",
            self.client.base_url, org_id, project_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let result: ListBackupsResponse = resp_json_payload(&mut resp).await?;

        Ok(result.backups)
    }
}
