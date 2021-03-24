#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use std::sync::Arc;

pub mod apis;
pub mod command;
mod http;
pub mod sender;
pub mod tokens;
mod types;
mod utils;

pub use sender::EscRequestSender;
pub use sender::RequestObserver;
pub use sender::RequestSender;
pub use types::*;

pub struct Builder {
    inner: reqwest::ClientBuilder,
    observer: Option<Arc<dyn RequestObserver + Send + Sync>>,
}

impl<'a> Builder {
    pub fn set_observer(
        mut self,
        observer: Option<Arc<dyn RequestObserver + Send + Sync>>,
    ) -> Builder {
        self.observer = observer;
        return self;
    }

    pub fn build(self, base_url: String, identity_url: String) -> crate::Result<Client> {
        let sender = crate::sender::RequestSender::new(self.inner.build()?, self.observer);
        Ok(Client {
            base_url,
            identity_url,
            sender,
        })
    }
}

#[derive(Clone)]
pub struct Client {
    pub base_url: String,
    pub identity_url: String,
    pub sender: crate::sender::RequestSender,
    // pub inner: reqwest::Client,
    // observer: Option<Arc<dyn RequestObserver + Send + Sync>>,
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

    pub fn esc_request_sender(&self, token: Token) -> crate::EscRequestSender {
        crate::EscRequestSender {
            base_url: self.base_url.clone(),
            sender: self.sender.clone(),
            token,
        }
    }

    // note: this replaces `default_error_handler`

    // pub async fn send_request<B: Serialize + ?Sized, R: DeserializeOwned>(
    //     &self,
    //     token: &Token,
    //     method: reqwest::Method,
    //     url: String,
    //     body: Option<&B>,
    // ) -> crate::Result<R> {
    //     use crate::http::Failure;

    //     if let Some(o) = &self.observer {
    //         let body_string: String = match body {
    //             Some(b) => match serde_json::to_string(b) {
    //                 Ok(s) => s,
    //                 _ => "<err!>".to_string(),
    //             },
    //             None => "".to_string(),
    //         };
    //         o.on_request(&method.as_str(), &url, &body_string);
    //     }

    //     let req = self
    //         .inner
    //         .request(method, url.as_str())
    //         .header(
    //             "Authorization",
    //             format!("{} {}", token.token_type, token.access_token),
    //         )
    //         .header("Accept", "application/json");
    //     let req = match body {
    //         Some(b) => req.json(b),
    //         None => req,
    //     };

    //     let resp = req.send().await?;

    //     let status = resp.status();

    //     if status.is_success() {
    //         return match &self.observer {
    //             Some(o) => {
    //                 let text = resp.text().await?;
    //                 o.on_response(status.as_str(), &text);
    //                 let r: R = serde_json::from_str(&text)?;
    //                 Ok(r)
    //             }
    //             None => {
    //                 let r: R = resp.json().await?;
    //                 Ok(r)
    //             }
    //         };
    //     }

    //     let message = resp.text().await?;

    //     if status.is_client_error() {
    //         if status.as_u16() == 401 {
    //             return Err(Failure::Client("Action not authorized".to_string()).into());
    //         }

    //         if status.as_u16() == 404 {
    //             return Err(Failure::Client("Resource doesn't exist".to_string()).into());
    //         }

    //         return Err(Failure::Client(message).into());
    //     }

    //     Err(Failure::Server(message).into())
    // }

    pub fn groups<'a>(&'a self, token: &'a Token) -> command::groups::Groups<'a> {
        command::groups::Groups::new(&self, &token)
    }

    pub fn tokens(&self) -> command::tokens::Tokens {
        command::tokens::Tokens::new(&self.sender.client, &self.identity_url)
    }

    pub fn networks<'a>(&'a self, token: &'a Token) -> command::networks::Networks<'a> {
        command::networks::Networks::new(&self, &token)
    }

    pub fn organizations<'a>(
        &'a self,
        token: &'a Token,
    ) -> command::organizations::Organizations<'a> {
        command::organizations::Organizations::new(&self, &token)
    }

    pub fn projects<'a>(&'a self, token: &'a Token) -> command::projects::Projects<'a> {
        command::projects::Projects::new(&self, &token)
    }

    pub fn peerings<'a>(&'a self, token: &'a Token) -> command::peerings::Peerings<'a> {
        command::peerings::Peerings::new(&self, &token)
    }

    pub fn clusters<'a>(&'a self, token: &'a Token) -> command::clusters::Clusters<'a> {
        command::clusters::Clusters::new(&self, &token)
    }

    pub fn backups<'a>(&'a self, token: &'a Token) -> command::backups::Backups<'a> {
        command::backups::Backups::new(&self, &token)
    }

    pub fn invites<'a>(&'a self, token: &'a Token) -> command::invites::Invites<'a> {
        command::invites::Invites::new(&self, &token)
    }

    pub fn policies<'a>(&'a self, token: &'a Token) -> command::policies::Policies<'a> {
        command::policies::Policies::new(&self, &token)
    }

    pub fn jobs<'a>(&'a self, token: &'a Token) -> command::jobs::Jobs<'a> {
        command::jobs::Jobs::new(&self, &token)
    }

    pub fn history<'a>(&'a self, token: &'a Token) -> command::history::History<'a> {
        command::history::History::new(&self, &token)
    }
}
