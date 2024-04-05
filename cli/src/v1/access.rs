use chrono::{DateTime, Utc};

use super::common::{List, StringNoQuotes, ToV1};
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

impl ToV1 for esc_api::access::CreateInviteResponse {
    type V1Type = StringNoQuotes;
    fn to_v1(self) -> Self::V1Type {
        StringNoQuotes(self.id.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub struct Email(String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EmailVisitor {})
    }
}

struct EmailVisitor {}

impl<'de> serde::de::Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid email")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if validator::validate_email(v) {
            return Ok(Email(v.to_string()));
        }

        Err(serde::de::Error::custom("Invalid email"))
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub id: esc_api::access::InviteId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub email: Email,
    pub groups: Option<Vec<GroupId>>,
    pub accepted: bool,
    pub created: DateTime<Utc>,
}

impl ToV1 for esc_api::access::Invite {
    type V1Type = Invite;
    fn to_v1(self) -> Self::V1Type {
        Invite {
            accepted: self.accepted,
            created: self.created,
            email: Email(self.email),
            groups: self
                .groups
                .map(|groups| groups.into_iter().map(|g| g.to_v1()).collect()),
            id: self.id,
            org_id: self.organization_id.to_v1(),
        }
    }
}

impl ToV1 for esc_api::access::ListInvitesResponse {
    type V1Type = List<Invite>;
    fn to_v1(self) -> Self::V1Type {
        let invites = self.invites.into_iter().map(|i| i.to_v1()).collect();
        List(invites)
    }
}

impl ToV1 for esc_api::access::CreatePolicyResponse {
    type V1Type = esc_api::access::PolicyId;
    fn to_v1(self) -> Self::V1Type {
        self.id
    }
}

impl ToV1 for esc_api::access::GetPolicyResponse {
    type V1Type = Self;
    fn to_v1(self) -> Self::V1Type {
        self
    }
}

impl ToV1 for esc_api::access::ListPoliciesResponse {
    type V1Type = Self;
    fn to_v1(self) -> Self::V1Type {
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: esc_api::access::MemberId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub active: bool,
    pub created: String,
    pub email: String,
    pub name: String,
}

impl ToV1 for esc_api::access::Member {
    type V1Type = Member;
    fn to_v1(self) -> Self::V1Type {
        Member {
            created: self.created,
            id: self.id,
            name: self.name,
            org_id: self.organization_id.to_v1(),
            active: self.active,
            email: self.email,
        }
    }
}

impl ToV1 for esc_api::access::ListMembersResponse {
    type V1Type = List<Member>;
    fn to_v1(self) -> Self::V1Type {
        let l: Vec<Member> = self.members.into_iter().map(|m| m.to_v1()).collect();
        List(l)
    }
}
