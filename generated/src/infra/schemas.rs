use super::formats::*;
use crate::resources::formats::ProjectId;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    pub language: String,
    pub title: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkRequest {
    pub provider: String,
    pub cidr_block: String,
    pub description: String,
    pub region: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkResponse {
    pub id: NetworkId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringCommandsRequest {
    pub provider: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringCommandsResponse {
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringRequest {
    pub network_id: String,
    pub description: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
    pub peer_network_region: String,
    pub routes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringResponse {
    pub id: String,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkResponse {
    pub network: Network,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeeringResponse {
    pub peering: Peering,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNetworksResponse {
    pub networks: Vec<Network>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPeeringsResponse {
    pub peerings: Vec<Peering>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub cidr_block: String,
    pub created: String,
    pub description: String,
    pub id: NetworkId,
    pub project_id: ProjectId,
    pub provider: String,
    pub region: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peering {
    pub created: String,
    pub description: String,
    pub id: String,
    pub network_id: String,
    pub network_cidr_block: String,
    pub project_id: String,
    pub provider: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
    pub peer_network_region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_peering_metadata: Option<HashMap<String, String>>,
    pub routes: Vec<String>,
    pub status: String,
}

/// underlying cloud provider
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Provider {
    Aws,
    Azure,
    Gcp,
}
impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Aws => write!(f, "aws"),
            Provider::Azure => write!(f, "azure"),
            Provider::Gcp => write!(f, "gcp"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNetworkRequest {
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePeeringRequest {
    pub description: String,
}
