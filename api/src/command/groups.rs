use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, GroupId, OrgId, Token};
use hyper::{body::HttpBody, Body, Uri};

pub struct Groups<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug, Deserialize)]
struct CreateGroupResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UpdateGroupResponse {
    id: String,
}

impl<'a> Groups<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Groups { client, token }
    }

    pub async fn create(
        self,
        name: String,
        org_id: OrgId,
        members: Vec<String>,
    ) -> crate::Result<GroupId> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups",
            self.client.base_url, org_id
        )
        .parse()?;
        let payload = serde_json::to_vec(&json!({
            "org_id": org_id.0,
            "name": name,
            "members": members,
        }))?;

        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let bytes = resp
            .body_mut()
            .data()
            .await
            .transpose()?
            .unwrap_or_default();
        let result: CreateGroupResponse = serde_json::from_slice(&bytes)?;

        Ok(GroupId(result.id))
    }

    pub async fn delete(self, id: GroupId, org_id: OrgId) -> crate::Result<()> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups/{}",
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

    pub fn update(self, id: GroupId, org_id: OrgId) -> UpdateGroup<'a> {
        UpdateGroup {
            client: self.client,
            token: self.token,
            id,
            org_id,
            name_opt: None,
            members_opt: None,
        }
    }
}

pub struct UpdateGroup<'a> {
    client: &'a Client,
    token: &'a Token,
    id: GroupId,
    org_id: OrgId,
    name_opt: Option<String>,
    members_opt: Option<Vec<String>>,
}

impl<'a> UpdateGroup<'a> {
    pub fn name(self, name: String) -> Self {
        UpdateGroup {
            name_opt: Some(name),
            ..self
        }
    }

    pub fn members(self, members: Vec<String>) -> Self {
        UpdateGroup {
            members_opt: Some(members),
            ..self
        }
    }

    pub fn set_name(&mut self, name_opt: Option<String>) {
        self.name_opt = name_opt;
    }

    pub fn set_members(&mut self, members_opt: Option<Vec<String>>) {
        self.members_opt = members_opt;
    }

    pub async fn execute(self) -> crate::Result<GroupId> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups/{}",
            self.client.base_url, self.org_id, self.id
        )
        .parse()?;
        let payload = serde_json::to_vec(&json!({
            "name": self.name_opt,
            "members": self.members_opt,
        }))?;

        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let bytes = resp
            .body_mut()
            .data()
            .await
            .transpose()?
            .unwrap_or_default();
        let result: UpdateGroupResponse = serde_json::from_slice(&bytes)?;

        Ok(GroupId(result.id))
    }
}
