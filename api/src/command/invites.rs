use crate::http::{
    authenticated_request, default_error_handler, req_json_payload, resp_json_payload,
};
use crate::{Client, Email, Invite, InviteId, OrgId, Token};
use hyper::{Body, Uri};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInviteParams {
    pub user_email: Email,
}

#[derive(Debug, Deserialize)]
struct CreateInviteResponse {
    id: InviteId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInviteParams {
    pub user_email: Email,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetInviteResponse {
    invite: Invite,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListInvitesResponse {
    invites: Vec<Invite>,
}

pub struct Invites<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Invites<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Invites { client, token }
    }

    pub async fn create(&self, org_id: OrgId, user_email: Email) -> crate::Result<InviteId> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/invites",
            self.client.base_url, org_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&CreateInviteParams { user_email })?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: CreateInviteResponse = resp_json_payload(&mut resp).await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        id: InviteId,
        user_email: Email,
    ) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/invites/{}",
            self.client.base_url, org_id, id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(req_json_payload(&UpdateInviteParams { user_email })?)?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn get(self, org_id: OrgId, id: InviteId) -> crate::Result<Option<Invite>> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/invites/{}",
            self.client.base_url, org_id, id
        )
        .parse()?;
        let req = authenticated_request(self.token, uri)
            .method("GET")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        if resp.status().as_u16() == 404 {
            return Ok(None);
        }

        default_error_handler(&mut resp).await?;
        let result: GetInviteResponse = resp_json_payload(&mut resp).await?;

        Ok(Some(result.invite))
    }

    pub async fn delete(self, org_id: OrgId, id: InviteId) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/invites/{}",
            self.client.base_url, org_id, id
        )
        .parse()?;
        let req = authenticated_request(self.token, uri)
            .method("DELETE")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<Invite>> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/invites",
            self.client.base_url, org_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Accept", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: ListInvitesResponse = resp_json_payload(&mut resp).await?;

        Ok(result.invites)
    }
}
