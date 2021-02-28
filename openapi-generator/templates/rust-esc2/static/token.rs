
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