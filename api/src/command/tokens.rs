use crate::{ClientId, Token};

pub struct Tokens<'a> {
    client: &'a reqwest::Client,
    identity_url: &'a str,
}

impl<'a> Tokens<'a> {
    pub fn new(client: &'a reqwest::Client, identity_url: &'a str) -> Self {
        Tokens {
            client,
            identity_url,
        }
    }

    pub async fn create(
        &self,
        client_id: &ClientId,
        username: &str,
        password: &str,
        audience: &str,
    ) -> crate::Result<Token> {
        crate::tokens::create(
            &self.client,
            &self.identity_url,
            client_id,
            username,
            password,
            audience,
        )
        .await
    }

    pub async fn refresh(&self, client_id: &ClientId, refresh_token: &str) -> crate::Result<Token> {
        crate::tokens::refresh(&self.client, &self.identity_url, client_id, refresh_token).await
    }
}
