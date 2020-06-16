use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, OrgId, Token};
use hyper::{Body, Uri};

pub struct Projects<'a> {
    client: &'a Client,
    token: &'a Token,
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
