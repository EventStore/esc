use super::config::TokenConfig;
use super::errors::IdentityError;
use super::tokens::Token;
use serde::de::DeserializeOwned;

pub type Result<T> = core::result::Result<T, IdentityError>;

async fn parse_result<T>(resp: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();
    if status.is_success() {
        let result: T = resp.json().await?;
        return Ok(result);
    }
    let message = resp.text().await?;
    Err(IdentityError {
        message,
        status_code: Some(status),
    })
}

pub async fn create(
    client: &reqwest::Client,
    config: &TokenConfig,
    user_name: &str,
    password: &str,
) -> Result<Token> {
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "password");
    form.insert("username", user_name);
    form.insert("password", password);
    form.insert("scope", "cloud:access offline_access");
    form.insert("client_id", config.client_id.as_ref());
    form.insert("audience", &config.audience);

    let url = format!("{}/oauth/token", &config.identity_url);
    let req = client.post(url.as_str()).form(&form);

    parse_result(req.send().await?).await
}

pub async fn refresh(
    client: &reqwest::Client,
    config: &TokenConfig,
    refresh_token: &str,
) -> Result<Token> {
    let url = format!("{}/oauth/token", &config.identity_url);
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "refresh_token");
    form.insert("client_id", config.client_id.as_ref());
    form.insert("refresh_token", refresh_token);

    let req = client.post(url.as_str()).form(&form);

    debug!("Token refresh on : {:?}", req);

    let token: Token = parse_result(req.send().await?).await?;

    debug!("Token expires_in: {}", token.expires_in);

    Ok(token)
}
