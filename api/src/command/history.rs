use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, HistoryItem, JobId, OrgId, ProjectId, Token};
use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListHistoryResponse {
    items: Vec<HistoryItem>,
}

pub struct History<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> History<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        History { client, token }
    }

    pub async fn list(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        job_id: Option<JobId>,
    ) -> crate::Result<Vec<HistoryItem>> {
        let mut req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/orchestrate/v1/organizations/{}/projects/{}/history",
                self.client.base_url, org_id, project_id
            ),
        )
        .header("Content-Type", "application/json");

        if let Some(job_id) = job_id {
            req = req.query(&[("jobId", job_id.0)]);
        }

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListHistoryResponse = resp.json().await?;

        Ok(result.items)
    }
}
