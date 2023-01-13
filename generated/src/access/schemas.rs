use super::formats::*;
use crate::resources::formats::OrganizationId;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

/// describes what a subject of athe policy's actions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Create,
    Delete,
    Modify,
    _None,
    Read,
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "create"),
            Action::Delete => write!(f, "delete"),
            Action::Modify => write!(f, "modify"),
            Action::_None => write!(f, "none"),
            Action::Read => write!(f, "read"),
        }
    }
}
impl std::cmp::PartialEq<&str> for Action {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Action::Create => *other == "create",
            Action::Delete => *other == "delete",
            Action::Modify => *other == "modify",
            Action::_None => *other == "none",
            Action::Read => *other == "read",
        }
    }
}
impl std::cmp::PartialEq<Action> for &str {
    fn eq(&self, other: &Action) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<MemberId>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupResponse {
    pub id: GroupId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInviteRequest {
    pub user_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupId>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInviteResponse {
    pub id: InviteId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicy {
    pub actions: Vec<Action>,
    pub effect: Effect,
    pub name: String,
    pub resources: Vec<String>,
    pub subjects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyRequest {
    pub policy: CreatePolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyResponse {
    pub id: PolicyId,
}

/// the policy's effect
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Effect {
    Allow,
    Deny,
}
impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Allow => write!(f, "allow"),
            Effect::Deny => write!(f, "deny"),
        }
    }
}
impl std::cmp::PartialEq<&str> for Effect {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Effect::Allow => *other == "allow",
            Effect::Deny => *other == "deny",
        }
    }
}
impl std::cmp::PartialEq<Effect> for &str {
    fn eq(&self, other: &Effect) -> bool {
        other == self
    }
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupResponse {
    pub group: Group,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemberResponse {
    pub member: Member,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyResponse {
    pub policy: Policy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettingsResponse {
    pub settings: Settings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub created: DateTime<Utc>,
    pub id: GroupId,
    pub linked_resource: String,
    pub members: Vec<MemberId>,
    pub name: String,
    pub organization_id: OrganizationId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub accepted: bool,
    pub created: DateTime<Utc>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupId>>,
    pub id: InviteId,
    pub organization_id: OrganizationId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupsResponse {
    pub groups: Vec<Group>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInvitesResponse {
    pub invites: Vec<Invite>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMembersResponse {
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPoliciesResponse {
    pub policies: Vec<Policy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUserRefreshTokensResponse {
    pub tokens: Vec<UserRefreshToken>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub active: bool,
    pub created: String,
    pub email: String,
    pub id: MemberId,
    pub name: String,
    pub organization_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    pub created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effect>,
    pub linked_resource: String,
    pub id: PolicyId,
    pub name: String,
    pub organization_id: OrganizationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyAllowedRequest {
    pub action: String,
    pub resource: Action,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyAllowedResponse {
    pub allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendInviteRequest {
    pub id: InviteId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub require_mfa: bool,
    pub restrict_invite_domain: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemberRequest {
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicy {
    pub actions: Vec<Action>,
    pub effect: Effect,
    pub name: String,
    pub resources: Vec<String>,
    pub subjects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolicyRequest {
    pub policy: UpdatePolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_mfa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_invite_domain: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRefreshToken {
    pub id: String,
    pub client_id: String,
    pub last_used: String,
}
