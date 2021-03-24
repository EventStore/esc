use crate::http::default_error_handler;

pub async fn create(
    client: &reqwest::Client,
    identity_url: &str,
    client_id: &crate::ClientId,
    username: &str,
    password: &str,
    audience: &str,
) -> crate::Result<crate::Token> {
    debug!("Audience: {}", audience);
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "password");
    form.insert("username", username);
    form.insert("password", password);
    form.insert("scope", "cloud:access offline_access");
    form.insert("client_id", client_id.as_ref());
    form.insert("audience", audience);

    let url = format!("{}/oauth/token", identity_url);
    let req = client.post(url.as_str()).form(&form);

    debug!("Token creation request on: {}", url);

    let resp = default_error_handler(req.send().await?).await?;
    let token = resp.json().await?;

    Ok(token)
}

pub async fn refresh(
    client: &reqwest::Client,
    identity_url: &str,
    client_id: &crate::ClientId,
    refresh_token: &str,
) -> crate::Result<crate::Token> {
    let url = format!("{}/oauth/token", identity_url);
    let mut form = std::collections::HashMap::new();

    form.insert("grant_type", "refresh_token");
    form.insert("client_id", client_id.as_ref());
    form.insert("refresh_token", refresh_token);

    let req = client.post(url.as_str()).form(&form);

    debug!("Token refresh on : {:?}", req);

    let resp = default_error_handler(req.send().await?).await?;
    let token: crate::Token = resp.json().await?;

    debug!("Token expires_in: {}", token.expires_in);

    Ok(token)
}
