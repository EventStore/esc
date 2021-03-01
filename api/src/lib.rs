#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

pub mod apis;
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
    ) -> apis::resources::apis::organizations_api::Organizations<'a> {
        apis::resources::apis::organizations_api::Organizations::new(&self, token)
    }

    pub fn projects<'a>(&'a self, token: &'a Token) -> apis::resources::apis::projects_api::Projects<'a> {
        apis::resources::apis::projects_api::Projects::new(&self, token)
    }

    pub fn peerings<'a>(&'a self, token: &'a Token) -> command::peerings::Peerings<'a> {
        command::peerings::Peerings::new(&self, token)
    }

    pub fn clusters<'a>(&'a self, token: &'a Token) -> apis::mesdb::apis::clusters_api::Clusters<'a> {
        apis::mesdb::apis::clusters_api::Clusters::new(&self, token)
    }

    pub fn backups<'a>(&'a self, token: &'a Token) -> apis::mesdb::apis::backups_api::Backups<'a> {
        apis::mesdb::apis::backups_api::Backups::new(&self, token)
    }

    pub fn invites<'a>(&'a self, token: &'a Token) -> command::invites::Invites<'a> {
        command::invites::Invites::new(&self, token)
    }

    pub fn policies<'a>(&'a self, token: &'a Token) -> command::policies::Policies<'a> {
        command::policies::Policies::new(&self, token)
    }
}
