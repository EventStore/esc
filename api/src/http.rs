use crate::Token;

pub enum Failure {
    Client(String),
    Server(String),
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        match self {
            Failure::Client(msg) => writeln!(f, "client: {}", msg),
            Failure::Server(msg) => writeln!(f, "server: {}", msg),
        }
    }
}

impl std::fmt::Debug for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        match self {
            Failure::Client(msg) => writeln!(f, "client: {}", msg),
            Failure::Server(msg) => writeln!(f, "server: {}", msg),
        }
    }
}

impl std::error::Error for Failure {}

pub async fn default_error_handler(resp: reqwest::Response) -> crate::Result<reqwest::Response> {
    let status = resp.status();

    if status.is_success() {
        return Ok(resp);
    }

    let message = resp.text().await?;

    if status.is_client_error() {
        if status.as_u16() == 401 {
            return Err(Failure::Client("Action not authorized".to_string()).into());
        }

        if status.as_u16() == 404 {
            return Err(Failure::Client("Resource doesn't exist".to_string()).into());
        }

        return Err(Failure::Client(message).into());
    }

    Err(Failure::Server(message).into())
}

pub fn authenticated_request(
    client: &crate::Client,
    meth: reqwest::Method,
    token: &Token,
    url: String,
) -> reqwest::RequestBuilder {
    client.sender.client.request(meth, url.as_str()).header(
        "Authorization",
        format!("{} {}", token.token_type, token.access_token),
    )
}
