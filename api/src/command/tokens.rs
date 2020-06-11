use crate::http::default_error_handler;
use crate::{Client, ClientId, Token};
use hyper::{body::HttpBody, Body, Request, Uri};
use url::form_urlencoded;

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
        let form = form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "password")
            .append_pair("username", username)
            .append_pair("password", password)
            .append_pair("scope", "cloud:access offline_access")
            .append_pair("client_id", client_id.as_ref())
            .append_pair("audience", audience)
            .finish();

        let uri: Uri = format!("{}/oauth/token", self.client.identity_url).parse()?;
        let req = Request::post(uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(form))?;

        debug!("Token creation request on: {}", req.uri());

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let bytes = resp
            .body_mut()
            .data()
            .await
            .transpose()?
            .unwrap_or_default();

        let token: Token = serde_json::from_slice(&bytes)?;

        Ok(token)
    }

    pub async fn refresh(&self, client_id: &ClientId, refresh_token: &str) -> crate::Result<Token> {
        let uri: Uri = format!("{}/oauth/token", self.client.identity_url).parse()?;
        let form = form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "refresh_token")
            .append_pair("client_id", client_id.as_ref())
            .append_pair("refresh_token", refresh_token)
            .finish();

        let req = Request::post(uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(form))?;

        debug!("Token refresh on : {:?}", req);

        let mut resp = self.client.inner.request(req).await?;

        default_error_handler(&mut resp).await?;

        let bytes = resp
            .body_mut()
            .data()
            .await
            .transpose()?
            .unwrap_or_default();

        let token: Token = serde_json::from_slice(&bytes)?;
        debug!("Token expires_in: {}", token.expires_in);

        Ok(token)
    }
}
