use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, OrgId, ProjectId, Token};
use hyper::{Body, Uri};

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
        let uri: Uri = format!(
            "{}/resources/v1/organizations/{}/projects",
            self.client.base_url, org_id
        )
        .parse()?;

        let payload = serde_json::to_vec(&CreateProjectParams { name })?;
        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: CreateProjectResponse = resp_json_payload(&mut resp).await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        proj_id: ProjectId,
        name: String,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/resources/v1/organizations/{}/projects/{}",
            self.client.base_url, org_id, proj_id
        )
        .parse()?;

        let payload = serde_json::to_vec(&UpdateProjectParams { name })?;
        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("content-type", "application/json")
            .body(Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        proj_id: ProjectId,
    ) -> crate::Result<Option<crate::Project>> {
        let uri: Uri = format!(
            "{}/resources/v1/organizations/{}/projects/{}",
            self.client.base_url, org_id, proj_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("content-type", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        if resp.status().as_u16() == 404 {
            return Ok(None);
        }

        default_error_handler(&mut resp).await?;
        let result: GetProjectResponse = resp_json_payload(&mut resp).await?;

        Ok(Some(result.project))
    }

    pub async fn delete(&self, org_id: OrgId, proj_id: ProjectId) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/resources/v1/organizations/{}/projects/{}",
            self.client.base_url, org_id, proj_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<crate::Project>> {
        let uri: Uri = format!(
            "{}/resources/v1/organizations/{}/projects",
            self.client.base_url, org_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Accept", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: ListProjectsResponse = resp_json_payload(&mut resp).await?;

        Ok(result.projects)
    }
}
