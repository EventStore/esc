use crate::http::{authenticated_request, default_error_handler, resp_json_payload};
use crate::{Client, GroupId, OrgId, Token};
use hyper::{Body, Uri};

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
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups",
            self.client.base_url, params.org_id
        )
        .parse()?;
        let payload = serde_json::to_vec(&params)?;

        let req = authenticated_request(self.token, uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: CreateGroupResponse = resp_json_payload(&mut resp).await?;

        Ok(result.id)
    }

    pub async fn get(self, id: GroupId, org_id: OrgId) -> crate::Result<Option<crate::Group>> {
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups/{}",
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
        let result: GetGroupResponse = resp_json_payload(&mut resp).await?;

        Ok(Some(result.group))
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
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups",
            self.client.base_url, org_id
        )
        .parse()?;

        let req = authenticated_request(self.token, uri)
            .method("GET")
            .header("Accept", "application/json")
            .body(Body::empty())?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;
        let result: ListGroupsResponse = resp_json_payload(&mut resp).await?;

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
        let uri: Uri = format!(
            "{}/access/v1/organizations/{}/groups/{}",
            self.client.base_url, self.org_id, self.id
        )
        .parse()?;
        let params = UpdateGroupParams {
            name: self.name_opt,
            members: self.members_opt,
        };
        let payload = serde_json::to_vec(&params)?;

        let req = authenticated_request(self.token, uri)
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(payload))?;

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        Ok(())
    }
}
