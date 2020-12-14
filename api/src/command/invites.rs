use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, Email, GroupId, Invite, InviteId, OrgId, Token};
use reqwest::Method;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInviteParams {
    pub user_email: Email,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupId>>,
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

    pub async fn create(
        &self,
        org_id: OrgId,
        user_email: Email,
        groups: Option<Vec<GroupId>>,
    ) -> crate::Result<InviteId> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/invites",
                self.client.base_url, org_id
            ),
        )
        .json(&CreateInviteParams { user_email, groups });

        let resp = default_error_handler(req.send().await?).await?;

        let result: CreateInviteResponse = resp.json().await?;

        Ok(result.id)
    }

    pub async fn update(
        &self,
        org_id: OrgId,
        id: InviteId,
        user_email: Email,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/invites/{}",
                self.client.base_url, org_id, id
            ),
        )
        .json(&UpdateInviteParams { user_email });

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(self, org_id: OrgId, id: InviteId) -> crate::Result<Option<Invite>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/invites/{}",
                self.client.base_url, org_id, id
            ),
        );

        let resp = default_error_handler(req.send().await?).await?;

        if resp.status().as_u16() == 404 {
            return Ok(None);
        }

        let result: GetInviteResponse = resp.json().await?;

        Ok(Some(result.invite))
    }

    pub async fn delete(self, org_id: OrgId, id: InviteId) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/invites/{}",
                self.client.base_url, org_id, id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<Invite>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/invites",
                self.client.base_url, org_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListInvitesResponse = resp.json().await?;

        Ok(result.invites)
    }
}
