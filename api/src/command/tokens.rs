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
        crate::tokens::create(&self.client.inner, &self.client.identity_url, client_id, username, password, audience).await
    }

    pub async fn refresh(&self, client_id: &ClientId, refresh_token: &str) -> crate::Result<Token> {
        crate::tokens::refresh(&self.client.inner, &self.client.identity_url, client_id, refresh_token).await
    }
}
