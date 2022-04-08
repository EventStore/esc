use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, GroupId, OrgId, Token};
use reqwest::Method;

pub struct Groups<'a> {
    client: &'a Client,
    token: &'a Token,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupParams {
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupParams {
    pub name: Option<String>,
    pub members: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CreateGroupResponse {
    id: GroupId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListGroupsResponse {
    groups: Vec<crate::Group>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetGroupResponse {
    group: crate::Group,
}

impl<'a> Groups<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Groups { client, token }
    }

    pub async fn create(&self, params: CreateGroupParams) -> crate::Result<GroupId> {
        let req = authenticated_request(
            self.client,
            Method::POST,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/groups",
                self.client.base_url, params.org_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;

        let result: CreateGroupResponse = resp.json().await?;

        Ok(result.id)
    }

    pub async fn get(self, id: GroupId, org_id: OrgId) -> crate::Result<Option<crate::Group>> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/groups/{}",
                self.client.base_url, org_id, id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        if resp.status().as_u16() == 404 {
            return Ok(None);
        }

        let result: GetGroupResponse = resp.json().await?;

        Ok(Some(result.group))
    }

    pub async fn delete(self, id: GroupId, org_id: OrgId) -> crate::Result<()> {
        let req = authenticated_request(
            self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/groups/{}",
                self.client.base_url, org_id, id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub fn update(&self, id: GroupId, org_id: OrgId) -> UpdateGroup<'a> {
        UpdateGroup {
            client: self.client,
            token: self.token,
            id,
            org_id,
            name_opt: None,
            members_opt: None,
        }
    }

    pub async fn list(&self, org_id: OrgId) -> crate::Result<Vec<crate::Group>> {
        let req = authenticated_request(
            self.client,
            Method::GET,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/groups",
                self.client.base_url, org_id
            ),
        )
        .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListGroupsResponse = resp.json().await?;

        Ok(result.groups)
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

    pub async fn execute(self) -> crate::Result<()> {
        let params = UpdateGroupParams {
            name: self.name_opt,
            members: self.members_opt,
        };

        let req = authenticated_request(
            self.client,
            Method::PUT,
            self.token,
            format!(
                "{}/access/v1/organizations/{}/groups/{}",
                self.client.base_url, self.org_id, self.id
            ),
        )
        .json(&params);

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }
}
