use serde::export::Formatter;

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
