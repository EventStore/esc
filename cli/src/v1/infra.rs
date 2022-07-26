use chrono::{DateTime, Utc};

use super::common::{List, StringNoQuotes, ToV1};
use super::resources::OrgId;

impl ToV1 for esc_api::infra::CreateNetworkResponse {
    type V1Type = esc_api::infra::NetworkId;
    fn to_v1(self) -> Self::V1Type {
        self.id
    }
}

// insert gif of cat scratching frantically on window here
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Aws,
    Gcp,
    Azure,
}

impl Provider {
    pub fn from_string(s: &str) -> Self {
        match s {
            "aws" => Self::Aws,
            "azure" => Self::Azure,
            "gcp" => Self::Gcp,
            _ => {
                eprintln!("unknown provider type: {s}");
                std::process::exit(1);
            }
        }
    }
}

impl ToV1 for esc_api::infra::Provider {
    type V1Type = Provider;
    fn to_v1(self) -> Self::V1Type {
        match self {
            Self::Aws => Provider::Aws,
            Self::Azure => Provider::Azure,
            Self::Gcp => Provider::Gcp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub project_id: esc_api::resources::ProjectId,
    #[serde(rename = "id")]
    pub id: esc_api::infra::NetworkId,
    pub provider: Provider,
    pub region: String,
    pub cidr_block: String,
    pub description: String,
    pub status: String,
}

impl ToV1 for esc_api::infra::Network {
    type V1Type = Network;
    fn to_v1(self) -> Self::V1Type {
        Network {
            cidr_block: self.cidr_block,
            description: self.description,
            id: self.id,
            project_id: self.project_id,
            provider: Provider::from_string(&self.provider),
            region: self.region,
            status: self.status,
        }
    }
}
impl ToV1 for esc_api::infra::GetNetworkResponse {
    type V1Type = Network;
    fn to_v1(self) -> Self::V1Type {
        self.network.to_v1()
    }
}

impl ToV1 for esc_api::infra::ListNetworksResponse {
    type V1Type = List<Network>;
    fn to_v1(self) -> Self::V1Type {
        let l = self.networks.into_iter().map(|n| n.to_v1()).collect();
        List(l)
    }
}
