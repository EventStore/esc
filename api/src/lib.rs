#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

pub mod command;
mod http;
mod types;

pub use types::*;

pub struct Builder {
    inner: reqwest::ClientBuilder,
}

impl Builder {
    pub fn build(self, base_url: String, identity_url: String) -> crate::Result<Client> {
        Ok(Client {
            base_url,
            identity_url,
            inner: self.inner.build()?,
        })
    }
}

pub struct Client {
    base_url: String,
    identity_url: String,
    pub inner: reqwest::Client,
}

impl Client {
    pub fn builder() -> Builder {
        Builder {
            inner: reqwest::Client::builder(),
        }
    }

    pub fn new(base_url: String, identity_url: String) -> crate::Result<Self> {
        Client::builder().build(base_url, identity_url)
    }

    pub fn groups<'a>(&'a self, token: &'a Token) -> command::groups::Groups<'a> {
        command::groups::Groups::new(&self, token)
    }

    pub fn tokens(&self) -> command::tokens::Tokens {
        command::tokens::Tokens::new(&self)
    }

    pub fn networks<'a>(&'a self, token: &'a Token) -> command::networks::Networks<'a> {
        command::networks::Networks::new(&self, token)
    }

    pub fn organizations<'a>(
        &'a self,
        token: &'a Token,
    ) -> command::organizations::Organizations<'a> {
        command::organizations::Organizations::new(&self, token)
    }

    pub fn projects<'a>(&'a self, token: &'a Token) -> command::projects::Projects<'a> {
        command::projects::Projects::new(&self, token)
    }

    pub fn peerings<'a>(&'a self, token: &'a Token) -> command::peerings::Peerings<'a> {
        command::peerings::Peerings::new(&self, token)
    }

    pub fn clusters<'a>(&'a self, token: &'a Token) -> command::clusters::Clusters<'a> {
        command::clusters::Clusters::new(&self, token)
    }

    pub fn backups<'a>(&'a self, token: &'a Token) -> command::backups::Backups<'a> {
        command::backups::Backups::new(&self, token)
    }

    pub fn invites<'a>(&'a self, token: &'a Token) -> command::invites::Invites<'a> {
        command::invites::Invites::new(&self, token)
    }

    pub fn policies<'a>(&'a self, token: &'a Token) -> command::policies::Policies<'a> {
        command::policies::Policies::new(&self, token)
    }

    pub fn jobs<'a>(&'a self, token: &'a Token) -> command::jobs::Jobs<'a> {
        command::jobs::Jobs::new(&self, token)
    }

    pub fn history<'a>(&'a self, token: &'a Token) -> command::history::History<'a> {
        command::history::History::new(&self, token)
    }

    pub fn integrations<'a>(&'a self, token: &'a Token) -> command::integrations::Integrations<'a> {
        command::integrations::Integrations::new(&self, token)
    }
}
