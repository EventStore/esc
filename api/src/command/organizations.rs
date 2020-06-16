use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, Token};
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
}
