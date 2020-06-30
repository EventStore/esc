use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct GroupId(pub String);

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct OrgId(pub String);

impl std::fmt::Display for OrgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct ClientId(pub String);

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ClientId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct ProjectId(pub String);

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct NetworkId(pub String);

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NetworkId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct PeeringId(pub String);

impl std::fmt::Display for PeeringId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for PeeringId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct ClusterId(pub String);

impl std::fmt::Display for ClusterId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ClusterId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub(crate) access_token: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) scope: String,
    pub(crate) expires_in: i64,
    pub(crate) token_type: String,
}

impl Token {
    pub fn refresh_token(&self) -> Option<&String> {
        self.refresh_token.as_ref()
    }

    pub fn access_token(&self) -> &str {
        self.access_token.as_str()
    }

    pub fn update_access_token(self, access_token: &str) -> Self {
        Token {
            access_token: access_token.to_string(),
            ..self
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    AWS,
    GCP,
    AZURE,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub project_id: ProjectId,
    #[serde(rename = "id")]
    pub id: NetworkId,
    pub provider: Provider,
    pub region: String,
    pub cidr_block: String,
    pub description: String,
    pub status: String,
}

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardClaims {
    #[serde(rename = "aud")]
    pub audience: Option<String>,

    #[serde(rename = "exp")]
    pub expires_at: Option<i64>,

    #[serde(rename = "jti")]
    pub id: Option<String>,

    #[serde(rename = "iat")]
    pub issue_at: Option<i64>,

    #[serde(rename = "iss")]
    pub issuer: Option<String>,

    #[serde(rename = "nbf")]
    pub not_before: Option<i64>,

    #[serde(rename = "sub")]
    pub subject: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: OrgId,
    pub name: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: GroupId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub created: String,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: ProjectId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peering {
    pub id: PeeringId,
    pub project_id: ProjectId,
    pub provider: Provider,
    pub network_id: NetworkId,
    pub description: String,
    pub peer_account: String,
    pub peer_network: String,
    pub peer_network_region: String,
    pub routes: Vec<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Topology {
    SingleNode,
    ThreeNodeMultiZone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub id: ClusterId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub project_id: ProjectId,
    pub network_id: NetworkId,
    pub description: String,
    pub provider: Provider,
    pub region: String,
    pub topology: Topology,
    pub instance_type: String,
    pub disk_size_gb: usize,
    pub disk_type: String,
    pub server_version: String,
    pub status: String,
    pub created: String,
}

struct EmailVisitor {}

impl<'de> serde::de::Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
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

impl Email {
    pub fn parse(str: &str) -> Option<Self> {
        if validator::validate_email(str) {
            return Some(Email(str.to_string()));
        }

        None
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct InviteId(pub String);

impl std::fmt::Display for InviteId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for InviteId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub id: InviteId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub email: Email,
    pub groups: Option<Vec<GroupId>>,
    pub accepted: bool,
    pub created: String, // FIXME - Move to a proper date data-structure.
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PolicyId(pub String);

impl std::fmt::Display for PolicyId {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for PolicyId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub id: PolicyId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub created: String,
    pub subjects: Vec<String>,
    pub resources: Vec<String>,
    pub actions: Vec<String>,
    pub effect: String,
}
