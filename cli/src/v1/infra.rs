use super::common::{List, ToV1};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peering {
    pub id: esc_api::infra::PeeringId,
    pub project_id: esc_api::resources::ProjectId,
    pub provider: Provider,
    pub network_id: esc_api::infra::NetworkId,
    pub description: String,
    pub peer_account: Option<String>,
    pub peer_network: Option<String>,
    pub peer_network_region: String,
    pub routes: Vec<String>,
    pub status: String,
}

impl ToV1 for esc_api::infra::Peering {
    type V1Type = Peering;
    fn to_v1(self) -> Self::V1Type {
        Peering {
            description: self.description,
            id: self.id,
            network_id: self.network_id,
            peer_account: Some(self.peer_account_id),
            peer_network: Some(self.peer_network_id),
            peer_network_region: self.peer_network_region,
            project_id: self.project_id,
            provider: Provider::from_string(&self.provider),
            routes: self.routes,
            status: self.status,
        }
    }
}

// The fields are out of order, but otherwise this is identical
impl ToV1 for esc_api::infra::CreatePeeringCommandsResponse {
    type V1Type = Vec<esc_api::infra::Command>;
    fn to_v1(self) -> Self::V1Type {
        self.commands
    }
}

impl ToV1 for esc_api::infra::GetPeeringResponse {
    type V1Type = Peering;
    fn to_v1(self) -> Self::V1Type {
        self.peering.to_v1()
    }
}

impl ToV1 for esc_api::infra::ListPeeringsResponse {
    type V1Type = List<Peering>;
    fn to_v1(self) -> Self::V1Type {
        let v = self.peerings.into_iter().map(|p| p.to_v1()).collect();
        List(v)
    }
}
