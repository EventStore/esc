pub use esc_client_base::identity::TokenConfig;
pub use esc_client_base::Authorization;
pub use esc_client_base::Client;
pub use esc_client_base::Error;
pub use esc_client_base::RequestObserver;
pub use esc_client_base::RequestSender;
pub use esc_client_base::Result;
pub use esc_client_base::Token;

pub mod access {
    pub use esc_client_generated::access::*;
}

pub mod infra {
    pub use esc_client_generated::infra::*;
}

pub mod integrate {
    pub use esc_client_generated::integrate::*;
}

pub mod mesdb {
    pub use esc_client_generated::mesdb::*;
}

pub mod orchestrate {
    pub use esc_client_generated::orchestrate::*;
}

pub mod resources {
    pub use esc_client_generated::resources::*;
}

// All of the following using statements are just to keep compatability with the
// test code. In the future they'll be scrapped.
pub use access::GroupId;
pub use access::PolicyId;
pub use esc_client_base::Client as EscRequestSender;
pub use infra::NetworkId;
pub use infra::PeeringId;
pub use infra::Provider;
pub use integrate::IntegrationId;
pub use mesdb::BackupId;
pub use mesdb::ClusterId;
pub use orchestrate::JobId;
pub use resources::OrganizationId as OrgId;
pub use resources::ProjectId;
