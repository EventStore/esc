use crate::http::default_error_handler;
use crate::{Client, ClientId, Token};

pub struct Tokens<'a> {
    client: &'a Client,
}

impl<'a> Tokens<'a> {
    pub fn new(client: &'a Client) -> Self {
        Tokens { client }
    }

    pub async fn create(
        &self,
        client_id: &ClientId,
        username: &str,
        password: &str,
        audience: &str,
    ) -> crate::Result<Token> {
        debug!("Audience: {}", audience);
        let mut form = std::collections::HashMap::new();

        form.insert("grant_type", "password");
        form.insert("username", username);
        form.insert("password", password);
        form.insert("scope", "cloud:access offline_access");
        form.insert("client_id", client_id.as_ref());
        form.insert("audience", audience);

        let url = format!("{}/oauth/token", self.client.identity_url);
        let req = self.client.inner.post(url.as_str()).form(&form);

        debug!("Token creation request on: {}", url);

        let resp = default_error_handler(req.send().await?).await?;
        let token = resp.json().await?;

        Ok(token)
    }

    pub async fn refresh(&self, client_id: &ClientId, refresh_token: &str) -> crate::Result<Token> {
        let url = format!("{}/oauth/token", self.client.identity_url);
        let mut form = std::collections::HashMap::new();

        form.insert("grant_type", "refresh_token");
        form.insert("client_id", client_id.as_ref());
        form.insert("refresh_token", refresh_token);

        let req = self.client.inner.post(url.as_str()).form(&form);

        debug!("Token refresh on : {:?}", req);

        let resp = default_error_handler(req.send().await?).await?;
        let token: Token = resp.json().await?;

        debug!("Token expires_in: {}", token.expires_in);

        Ok(token)
    }
}
