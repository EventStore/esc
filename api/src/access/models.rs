/// Action : describes what a subject of athe policy's actions

/// describes what a subject of athe policy's actions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
}

impl Action {
    pub fn from_str(src: &str) -> Result<Action, String> {
        match src {
            "create" => Ok(Action::Create),
            "delete" => Ok(Action::Delete),
            "modify" => Ok(Action::Modify),
            "none" => Ok(Action::None),
            "read" => Ok(Action::Read),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["create", "delete", "modify", "none", "read",]
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "id")]
    pub id: crate::types::GroupId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::types::GroupId>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInviteResponse {
    #[serde(rename = "id")]
    pub id: crate::types::InviteId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicy {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::access::models::Action>,
    #[serde(rename = "effect")]
    pub effect: crate::access::models::Effect,
    /// the policy's name
    #[serde(rename = "name")]
    pub name: String,
    /// the policy's resources
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// the policy's name
    #[serde(rename = "subjects")]
    pub subjects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    #[serde(rename = "policy")]
    pub policy: crate::access::models::CreatePolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePolicyResponse {
    #[serde(rename = "id")]
    pub id: String,
}

/// Effect : the policy's effect

/// the policy's effect
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

impl Effect {
    pub fn from_str(src: &str) -> Result<Effect, String> {
        match src {
            "allow" => Ok(Effect::Allow),
            "deny" => Ok(Effect::Deny),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["allow", "deny",]
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupResponse {
    #[serde(rename = "group")]
    pub group: crate::access::models::Group,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMemberResponse {
    #[serde(rename = "member")]
    pub member: crate::access::models::Member,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPolicyResponse {
    #[serde(rename = "policy")]
    pub policy: crate::access::models::Policy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSettingsResponse {
    #[serde(rename = "settings")]
    pub settings: crate::access::models::Settings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "id")]
    pub id: crate::types::GroupId,
    #[serde(rename = "linkedResource")]
    pub linked_resource: String,
    #[serde(rename = "members")]
    pub members: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invite {
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "groups")]
    pub groups: Vec<crate::types::GroupId>,
    #[serde(rename = "id")]
    pub id: crate::types::InviteId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "groups")]
    pub groups: Vec<crate::access::models::Group>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListInvitesResponse {
    #[serde(rename = "invites")]
    pub invites: Vec<crate::access::models::Invite>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMembersResponse {
    #[serde(rename = "members")]
    pub members: Vec<crate::access::models::Member>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPoliciesResponse {
    #[serde(rename = "policies")]
    pub policies: Vec<crate::access::models::Policy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUserRefreshTokensResponse {
    #[serde(rename = "tokens")]
    pub tokens: Vec<crate::access::models::UserRefreshToken>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::access::models::Action>>,
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<crate::access::models::Effect>,
    #[serde(rename = "linkedResource")]
    pub linked_resource: String,
    #[serde(rename = "id")]
    pub id: crate::types::PolicyId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "subjects", skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAllowedRequest {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "resource")]
    pub resource: crate::access::models::Action,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAllowedResponse {
    #[serde(rename = "allowed")]
    pub allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResendInviteRequest {
    #[serde(rename = "id")]
    pub id: crate::types::InviteId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "requireMfa")]
    pub require_mfa: bool,
    #[serde(rename = "restrictInviteDomain")]
    pub restrict_invite_domain: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    #[serde(rename = "active")]
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePolicy {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::access::models::Action>,
    #[serde(rename = "effect")]
    pub effect: crate::access::models::Effect,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    #[serde(rename = "subjects")]
    pub subjects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePolicyRequest {
    #[serde(rename = "policy")]
    pub policy: crate::access::models::UpdatePolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    #[serde(rename = "requireMfa", skip_serializing_if = "Option::is_none")]
    pub require_mfa: Option<bool>,
    #[serde(
        rename = "restrictInviteDomain",
        skip_serializing_if = "Option::is_none"
    )]
    pub restrict_invite_domain: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRefreshToken {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "lastUsed")]
    pub last_used: String,
}
