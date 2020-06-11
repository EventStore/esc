#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod command;
mod http;
mod types;

pub use types::*;

use hyper::{client::HttpConnector, Body};
use hyper_rustls::HttpsConnector;

pub struct Builder {
    inner: hyper::client::Builder,
}

impl Builder {
    pub fn build(self, base_url: String, identity_url: String) -> Client {
        Client {
            base_url,
            identity_url,
            inner: self.inner.build(HttpsConnector::new()),
        }
    }
}

pub struct Client {
    base_url: String,
    identity_url: String,
    inner: hyper::Client<HttpsConnector<HttpConnector>, Body>,
}

impl Client {
    pub fn builder() -> Builder {
        Builder {
            inner: hyper::client::Client::builder(),
        }
    }

    pub fn new(base_url: String, identity_url: String) -> Self {
        Client::builder().build(base_url, identity_url)
    }

    pub fn groups<'a>(&'a self, token: &'a Token) -> command::groups::Groups<'a> {
        command::groups::Groups::new(&self, token)
    }

    pub fn tokens(&self) -> command::tokens::Tokens {
        command::tokens::Tokens::new(&self)
    }
}
