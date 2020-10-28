use crate::Token;
use bytes::{Bytes, BytesMut};
use http::{Request, Uri};
use hyper::{body::HttpBody, Body};
use serde::export::Formatter;
use serde::Serialize;

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

    let bytes = read_all_bytes(resp.body_mut()).await?;
    let message = std::string::String::from_utf8_lossy(bytes.as_ref()).into_owned();

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
    let bytes = read_all_bytes(resp.body_mut()).await?;
    let value = serde_json::from_reader(std::io::Cursor::new(bytes))?;

    Ok(value)
}

pub fn req_json_payload<A: Serialize>(payload: &A) -> crate::Result<Body> {
    let bytes = serde_json::to_vec(payload)?;
    Ok(Body::from(bytes))
}

async fn read_all_bytes(body: &mut Body) -> crate::Result<Bytes> {
    let mut buffer = BytesMut::new();

    while let Some(bytes) = body.data().await.transpose()? {
        buffer.extend(bytes);
    }

    Ok(buffer.freeze())
}
