use super::formats::*;
use crate::infra::formats::NetworkId;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub available_node_count: i32,
    pub created: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_resource: Option<String>,
    pub id: BackupId,
    pub project_id: ProjectId,
    pub provider: String,
    pub region: String,
    pub server_version: String,
    pub server_version_tag: String,
    pub size_gb: i32,
    pub source_cluster_id: ClusterId,
    pub source_cluster_description: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub can_expand_disk: bool,
    pub cloud_integrated_authentication: bool,
    pub created: String,
    pub description: String,
    pub disk_size_gb: i32,
    pub disk_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throughput: Option<i32>,
    pub id: ClusterId,
    pub instance_type: String,
    pub health: Health,
    pub network_id: NetworkId,
    pub organization_id: OrganizationId,
    pub project_id: ProjectId,
    pub projection_level: ProjectionLevel,
    pub provider: String,
    pub region: String,
    pub server_version: String,
    pub server_version_tag: String,
    pub status: String,
    pub topology: Topology,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupRequest {
    pub description: String,
    pub source_cluster_id: ClusterId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupResponse {
    pub id: BackupId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterRequest {
    pub description: String,
    pub disk_size_gb: i32,
    pub disk_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throughput: Option<i32>,
    pub instance_type: String,
    pub network_id: NetworkId,
    pub projection_level: ProjectionLevel,
    pub server_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_node_index: Option<i32>,
    pub topology: Topology,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterResponse {
    pub id: ClusterId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandClusterDiskRequest {
    pub disk_size_gb: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throughput: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBackupResponse {
    pub backup: Backup,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClusterResponse {
    pub cluster: Cluster,
}

/// The health of the database
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Health {
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "health-reporting-error")]
    HealthReportingError,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "issues-detected")]
    IssuesDetected,
}
impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Health::Degraded => write!(f, "degraded"),
            Health::Down => write!(f, "down"),
            Health::HealthReportingError => write!(f, "health-reporting-error"),
            Health::Ok => write!(f, "ok"),
            Health::IssuesDetected => write!(f, "issues-detected"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBackupsResponse {
    pub backups: Vec<Backup>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersResponse {
    pub clusters: Vec<Cluster>,
}

/// The projection level of your database. Can be off, system or user
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProjectionLevel {
    Off,
    System,
    User,
}
impl std::fmt::Display for ProjectionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectionLevel::Off => write!(f, "off"),
            ProjectionLevel::System => write!(f, "system"),
            ProjectionLevel::User => write!(f, "user"),
        }
    }
}

/// Either single-node or three-node-multi-zone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Topology {
    #[serde(rename = "single-node")]
    SingleNode,
    #[serde(rename = "three-node-multi-zone")]
    ThreeNodeMultiZone,
}
impl std::fmt::Display for Topology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Topology::SingleNode => write!(f, "single-node"),
            Topology::ThreeNodeMultiZone => write!(f, "three-node-multi-zone"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClusterRequest {
    pub description: String,
}
