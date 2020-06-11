use serde::export::Formatter;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GroupId(pub String);

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct OrgId(pub String);

impl std::fmt::Display for OrgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
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
