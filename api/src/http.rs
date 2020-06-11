use crate::Token;
use http::{Request, Uri};
use hyper::{body::HttpBody, Body};
use serde::export::Formatter;

enum Failure {
    Client(String),
    Server(String),
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Failure::Client(msg) => writeln!(f, "client: {}", msg),
            Failure::Server(msg) => writeln!(f, "server: {}", msg),
        }
    }
}

impl std::fmt::Debug for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Failure::Client(msg) => writeln!(f, "client: {}", msg),
            Failure::Server(msg) => writeln!(f, "server: {}", msg),
        }
    }
}

impl std::error::Error for Failure {}

pub async fn default_error_handler(resp: &mut hyper::Response<Body>) -> crate::Result<()> {
    if resp.status().is_success() {
        return Ok(());
    }

    let body = resp
        .body_mut()
        .data()
        .await
        .transpose()?
        .unwrap_or_default();
    let message = std::string::String::from_utf8_lossy(&body).into_owned();

    if resp.status().is_client_error() {
        if resp.status().as_u16() == 401 {
            return Err(Failure::Client("Action not authorized".to_string()).into());
        }

        if resp.status().as_u16() == 404 {
            return Err(Failure::Client("Resource doesn't exist".to_string()).into());
        }

        return Err(Failure::Client(message).into());
    }

    Err(Failure::Server(message).into())
}

pub fn authenticated_request(token: &Token, uri: Uri) -> http::request::Builder {
    Request::builder().uri(uri).header(
        "Authorization",
        format!("{} {}", token.token_type, token.access_token),
    )
}

pub async fn resp_json_payload<A>(resp: &mut hyper::Response<Body>) -> crate::Result<A>
where
    A: serde::de::DeserializeOwned,
{
    let bytes = resp
        .body_mut()
        .data()
        .await
        .transpose()?
        .unwrap_or_default();

    let value = serde_json::from_reader(std::io::Cursor::new(bytes))?;

    Ok(value)
}
