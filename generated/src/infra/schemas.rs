use super::formats::*;
use crate::resources::formats::ProjectId;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acl {
    pub cidr_blocks: Vec<String>,
    pub created: String,
    pub description: String,
    pub id: AclId,
    pub project_id: ProjectId,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    pub language: String,
    pub title: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAclRequest {
    pub cidr_blocks: Vec<String>,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAclResponse {
    pub id: AclId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkRequest {
    pub provider: String,
    pub cidr_block: String,
    pub description: String,
    pub region: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkResponse {
    pub id: NetworkId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringCommandsRequest {
    pub provider: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringCommandsResponse {
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringRequest {
    pub network_id: NetworkId,
    pub description: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
    pub peer_network_region: String,
    pub routes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeeringResponse {
    pub id: PeeringId,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAclResponse {
    pub acl: Acl,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkResponse {
    pub network: Network,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeeringResponse {
    pub peering: Peering,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAclsResponse {
    pub acls: Vec<Acl>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNetworksResponse {
    pub networks: Vec<Network>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPeeringsResponse {
    pub peerings: Vec<Peering>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub cidr_block: String,
    pub created: String,
    pub description: String,
    pub id: NetworkId,
    pub project_id: ProjectId,
    pub provider: String,
    pub region: String,
    pub status: NetworkStatus,
}

/// The status of the network
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkStatus {
    Provisioning,
    Defunct,
    Available,
    Deleting,
    Deleted,
}
impl std::fmt::Display for NetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkStatus::Provisioning => write!(f, "provisioning"),
            NetworkStatus::Defunct => write!(f, "defunct"),
            NetworkStatus::Available => write!(f, "available"),
            NetworkStatus::Deleting => write!(f, "deleting"),
            NetworkStatus::Deleted => write!(f, "deleted"),
        }
    }
}
impl std::cmp::PartialEq<&str> for NetworkStatus {
    fn eq(&self, other: &&str) -> bool {
        match self {
            NetworkStatus::Provisioning => *other == "provisioning",
            NetworkStatus::Defunct => *other == "defunct",
            NetworkStatus::Available => *other == "available",
            NetworkStatus::Deleting => *other == "deleting",
            NetworkStatus::Deleted => *other == "deleted",
        }
    }
}
impl std::cmp::PartialEq<NetworkStatus> for &str {
    fn eq(&self, other: &NetworkStatus) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peering {
    pub created: String,
    pub description: String,
    pub id: PeeringId,
    pub network_id: NetworkId,
    pub network_cidr_block: String,
    pub project_id: ProjectId,
    pub provider: String,
    pub peer_account_id: String,
    pub peer_network_id: String,
    pub peer_network_region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_peering_metadata: Option<HashMap<String, String>>,
    pub routes: Vec<String>,
    pub status: PeeringStatus,
}

/// The status of the peering
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PeeringStatus {
    Provisioning,
    Initiated,
    Active,
    Defunct,
    Deleting,
    Deleted,
    Unknown,
}
impl std::fmt::Display for PeeringStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeeringStatus::Provisioning => write!(f, "provisioning"),
            PeeringStatus::Initiated => write!(f, "initiated"),
            PeeringStatus::Active => write!(f, "active"),
            PeeringStatus::Defunct => write!(f, "defunct"),
            PeeringStatus::Deleting => write!(f, "deleting"),
            PeeringStatus::Deleted => write!(f, "deleted"),
            PeeringStatus::Unknown => write!(f, "unknown"),
        }
    }
}
impl std::cmp::PartialEq<&str> for PeeringStatus {
    fn eq(&self, other: &&str) -> bool {
        match self {
            PeeringStatus::Provisioning => *other == "provisioning",
            PeeringStatus::Initiated => *other == "initiated",
            PeeringStatus::Active => *other == "active",
            PeeringStatus::Defunct => *other == "defunct",
            PeeringStatus::Deleting => *other == "deleting",
            PeeringStatus::Deleted => *other == "deleted",
            PeeringStatus::Unknown => *other == "unknown",
        }
    }
}
impl std::cmp::PartialEq<PeeringStatus> for &str {
    fn eq(&self, other: &PeeringStatus) -> bool {
        other == self
    }
}

/// underlying cloud provider
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
impl std::cmp::PartialEq<&str> for Provider {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Provider::Aws => *other == "aws",
            Provider::Azure => *other == "azure",
            Provider::Gcp => *other == "gcp",
        }
    }
}
impl std::cmp::PartialEq<Provider> for &str {
    fn eq(&self, other: &Provider) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAclRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_blocks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNetworkRequest {
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePeeringRequest {
    pub description: String,
}
