use chrono::{DateTime, Utc};

use super::common::{List, ToV1};
use super::resources::OrgId;
use esc_api::access::GroupId;

impl ToV1 for GroupId {
    type V1Type = GroupId;
    fn to_v1(self) -> Self::V1Type {
        self
    }
}

impl ToV1 for esc_api::access::CreateGroupResponse {
    type V1Type = GroupId;
    fn to_v1(self) -> Self::V1Type {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: GroupId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub members: Vec<String>,
}

impl ToV1 for esc_api::access::Group {
    type V1Type = Group;
    fn to_v1(self) -> Self::V1Type {
        Group {
            created: self.created,
            id: self.id,
            members: self.members.into_iter().map(|m| m.0).collect(),
            name: self.name,
            org_id: self.organization_id.to_v1(),
        }
    }
}

impl ToV1 for esc_api::access::GetGroupResponse {
    type V1Type = Group;
    fn to_v1(self) -> Self::V1Type {
        self.group.to_v1()
    }
}

impl ToV1 for esc_api::access::ListGroupsResponse {
    type V1Type = List<Group>;
    fn to_v1(self) -> Self::V1Type {
        let l: Vec<Group> = self.groups.into_iter().map(|g| g.to_v1()).collect();
        List(l)
    }
}
