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
