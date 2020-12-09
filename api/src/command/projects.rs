use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, OrgId, ProjectId, Token};
use reqwest::Method;

pub struct Projects<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateProjectParams {
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProjectParams {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateProjectResponse {
    id: ProjectId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetProjectResponse {
    project: crate::Project,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListProjectsResponse {
    projects: Vec<crate::Project>,
}

impl<'a> Projects<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Projects { client, token }
    }

    pub async fn create(&self, org_id: OrgId, name: String) -> crate::Result<ProjectId> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/resources/v1/organizations/{}/projects",
                self.client.base_url, org_id
            ),
        )
        .json(&CreateProjectParams { name });

        let resp = default_error_handler(req.send().await?).await?;

        let result: CreateProjectResponse = resp.json().await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        proj_id: ProjectId,
        name: String,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/resources/v1/organizations/{}/projects/{}",
                self.client.base_url, org_id, proj_id
            ),
        )
        .json(&UpdateProjectParams { name });

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        proj_id: ProjectId,
    ) -> crate::Result<Option<crate::Project>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/resources/v1/organizations/{}/projects/{}",
                self.client.base_url, org_id, proj_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        if resp.status().as_u16() == 404 {
            return Ok(None);
        }

        let result: GetProjectResponse = resp.json().await?;

        Ok(Some(result.project))
    }

    pub async fn delete(&self, org_id: OrgId, proj_id: ProjectId) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/resources/v1/organizations/{}/projects/{}",
                self.client.base_url, org_id, proj_id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<crate::Project>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/resources/v1/organizations/{}/projects",
                self.client.base_url, org_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListProjectsResponse = resp.json().await?;

        Ok(result.projects)
    }
}
