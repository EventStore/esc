#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod apis;
pub mod command;
mod http;
mod types;
mod utils;

pub use types::*;

pub struct Builder {
    inner: reqwest::ClientBuilder,
    observer: Option<Arc<dyn ClientObserver>>,
}

impl<'a> Builder {
    pub fn set_observer(mut self, observer: Option<Arc<dyn ClientObserver>>) -> Builder {
        self.observer = observer;
        return self;
    }

    pub fn build(self, base_url: String, identity_url: String) -> crate::Result<Client> {
        Ok(Client {
            base_url,
            identity_url,
            inner: self.inner.build()?,
            observer: self.observer,
        })
    }
}

pub trait ClientObserver {
    fn on_request(&self, method: &str, url: &str, body: &str);
    fn on_response(&self, status: &str, body: &str);
}

#[derive(Clone)]
pub struct Client {
    base_url: String,
    identity_url: String,
    pub inner: reqwest::Client,
    observer: Option<Arc<dyn ClientObserver>>,
}

impl Client {
    pub fn builder() -> Builder {
        Builder {
            inner: reqwest::Client::builder(),
            observer: None,
        }
    }

    pub fn new(base_url: String, identity_url: String) -> crate::Result<Self> {
        Client::builder().build(base_url, identity_url)
    }

    // note: this replaces `default_error_handler`

    pub async fn send_request<B: Serialize + ?Sized, R: DeserializeOwned>(&self, token: &Token, method: reqwest::Method, url: String, body: Option<&B>) -> crate::Result<R> {        
        use crate::http::Failure;
        
        if let Some(o) = &self.observer {
            let body_string: String = match body {
                Some(b) => match serde_json::to_string(b) { Ok(s) => s, _ => "<err!>".to_string() },
                None => "".to_string(),
            };
            o.on_request(&method.as_str(), &url, &body_string);
        }
    
        let req = self.inner.request(method, url.as_str()).header("Authorization",
        format!("{} {}", token.token_type, token.access_token)).header("Accept", "application/json");        
        let req = match body {
            Some(b) => req.json(b),
            None => req
        };

        let resp = req.send().await?;
        
        let status = resp.status();        

        if status.is_success() {
            return match &self.observer {
                Some(o) => {
                    let text = resp.text().await?;
                    o.on_response(status.as_str(), &text);
                    let r: R = serde_json::from_str(&text)?;
                    Ok(r)
                },
                None => {
                    let r: R = resp.json().await?;
                    Ok(r)
                }
            };
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

    pub fn tokens(&self) -> command::tokens::Tokens {
        command::tokens::Tokens::new(&self)
    }
}
