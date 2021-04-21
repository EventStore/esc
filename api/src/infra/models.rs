#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "cidrBlock")]
    pub cidr_block: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "region")]
    pub region: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkResponse {
    #[serde(rename = "id")]
    pub id: crate::types::NetworkId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePeeringCommandsRequest {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "peerAccountId")]
    pub peer_account_id: String,
    #[serde(rename = "peerNetworkId")]
    pub peer_network_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePeeringCommandsResponse {
    #[serde(rename = "commands")]
    pub commands: Vec<crate::infra::models::Command>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePeeringRequest {
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "peerAccountId")]
    pub peer_account_id: String,
    #[serde(rename = "peerNetworkId")]
    pub peer_network_id: String,
    #[serde(rename = "peerNetworkRegion")]
    pub peer_network_region: String,
    #[serde(rename = "routes")]
    pub routes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePeeringResponse {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkResponse {
    #[serde(rename = "network")]
    pub network: crate::infra::models::Network,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPeeringResponse {
    #[serde(rename = "peering")]
    pub peering: crate::infra::models::Peering,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListNetworksResponse {
    #[serde(rename = "networks")]
    pub networks: Vec<crate::infra::models::Network>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPeeringsResponse {
    #[serde(rename = "peerings")]
    pub peerings: Vec<crate::infra::models::Peering>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "cidrBlock")]
    pub cidr_block: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: crate::types::NetworkId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peering {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(rename = "networkCidrBlock")]
    pub network_cidr_block: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "peerAccountId")]
    pub peer_account_id: String,
    #[serde(rename = "peerNetworkId")]
    pub peer_network_id: String,
    #[serde(rename = "peerNetworkRegion")]
    pub peer_network_region: String,
    #[serde(
        rename = "providerPeeringMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_peering_metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "routes")]
    pub routes: Vec<String>,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateNetworkRequest {
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePeeringRequest {
    #[serde(rename = "description")]
    pub description: String,
}
