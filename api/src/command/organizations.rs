use crate::http::{
    authenticated_request, default_error_handler, req_json_payload, resp_json_payload,
};
use crate::{Client, OrgId, Token};
use hyper::Uri;

pub struct Organizations<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListOrgsResp {
    organizations: Vec<crate::Organization>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetOrgResp {
    organization: crate::Organization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrgRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateOrgRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrgResponse {
    id: OrgId,
}

impl<'a> Organizations<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Organizations { client, token }
    }

    pub async fn list(&self) -> crate::Result<Vec<crate::Organization>> {
        let uri: Uri = format!("{}/resources/v1/organizations", self.client.base_url).parse()?;
        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Accept", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: ListOrgsResp = resp_json_payload(&mut resp).await?;

        Ok(result.organizations)
    }

    pub async fn get(&self, id: OrgId) -> crate::Result<crate::Organization> {
        let uri: Uri =
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id).parse()?;
        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Accept", "application/json")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: GetOrgResp = resp_json_payload(&mut resp).await?;

        Ok(result.organization)
    }

    pub async fn create(&self, name: String) -> crate::Result<OrgId> {
        let uri: Uri = format!("{}/resources/v1/organizations", self.client.base_url).parse()?;
        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&CreateOrgRequest { name })?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: CreateOrgResponse = resp_json_payload(&mut resp).await?;

        Ok(result.id)
    }

    pub async fn update(&self, id: OrgId, name: String) -> crate::Result<()> {
        let uri: Uri =
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id).parse()?;
        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&UpdateOrgRequest { name })?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn delete(&self, id: OrgId) -> crate::Result<()> {
        let uri: Uri =
            format!("{}/resources/v1/organizations/{}", self.client.base_url, id).parse()?;
        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .body(hyper::Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }
}
