use super::formats::*;
use crate::infra::formats::NetworkId;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub available_node_count: i32,
    pub created: DateTime<Utc>,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub can_expand_disk: bool,
    pub cloud_integrated_authentication: bool,
    pub created: DateTime<Utc>,
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
    pub status: ClusterStatus,
    pub topology: Topology,
    pub protected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ClusterStatus {
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "disks available")]
    DisksAvailable,
    #[serde(rename = "expanding disks")]
    ExpandingDisks,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "defunct")]
    Defunct,
    #[serde(rename = "inconsistent")]
    Inconsistent,
    #[serde(rename = "deleting instances")]
    DeletingInstances,
    #[serde(rename = "instances deleted")]
    InstancesDeleted,
    #[serde(rename = "deleting disks")]
    DeletingDisks,
    #[serde(rename = "deleted")]
    Deleted,
}
impl std::fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClusterStatus::Provisioning => write!(f, "provisioning"),
            ClusterStatus::DisksAvailable => write!(f, "disks available"),
            ClusterStatus::ExpandingDisks => write!(f, "expanding disks"),
            ClusterStatus::Restarting => write!(f, "restarting"),
            ClusterStatus::Available => write!(f, "available"),
            ClusterStatus::Defunct => write!(f, "defunct"),
            ClusterStatus::Inconsistent => write!(f, "inconsistent"),
            ClusterStatus::DeletingInstances => write!(f, "deleting instances"),
            ClusterStatus::InstancesDeleted => write!(f, "instances deleted"),
            ClusterStatus::DeletingDisks => write!(f, "deleting disks"),
            ClusterStatus::Deleted => write!(f, "deleted"),
        }
    }
}
impl std::cmp::PartialEq<&str> for ClusterStatus {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ClusterStatus::Provisioning => *other == "provisioning",
            ClusterStatus::DisksAvailable => *other == "disks available",
            ClusterStatus::ExpandingDisks => *other == "expanding disks",
            ClusterStatus::Restarting => *other == "restarting",
            ClusterStatus::Available => *other == "available",
            ClusterStatus::Defunct => *other == "defunct",
            ClusterStatus::Inconsistent => *other == "inconsistent",
            ClusterStatus::DeletingInstances => *other == "deleting instances",
            ClusterStatus::InstancesDeleted => *other == "instances deleted",
            ClusterStatus::DeletingDisks => *other == "deleting disks",
            ClusterStatus::Deleted => *other == "deleted",
        }
    }
}
impl std::cmp::PartialEq<ClusterStatus> for &str {
    fn eq(&self, other: &ClusterStatus) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupRequest {
    pub description: String,
    pub source_cluster_id: ClusterId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBackupResponse {
    pub id: BackupId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterResponse {
    pub id: ClusterId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBackupResponse {
    pub backup: Backup,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClusterResponse {
    pub cluster: Cluster,
}

/// The health of the database
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
impl std::cmp::PartialEq<&str> for Health {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Health::Degraded => *other == "degraded",
            Health::Down => *other == "down",
            Health::HealthReportingError => *other == "health-reporting-error",
            Health::Ok => *other == "ok",
            Health::IssuesDetected => *other == "issues-detected",
        }
    }
}
impl std::cmp::PartialEq<Health> for &str {
    fn eq(&self, other: &Health) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBackupsResponse {
    pub backups: Vec<Backup>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersResponse {
    pub clusters: Vec<Cluster>,
}

/// The projection level of your database. Can be off, system or user
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
impl std::cmp::PartialEq<&str> for ProjectionLevel {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ProjectionLevel::Off => *other == "off",
            ProjectionLevel::System => *other == "system",
            ProjectionLevel::User => *other == "user",
        }
    }
}
impl std::cmp::PartialEq<ProjectionLevel> for &str {
    fn eq(&self, other: &ProjectionLevel) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartClusterResponse {
    pub id: String,
}

/// Either single-node or three-node-multi-zone
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
impl std::cmp::PartialEq<&str> for Topology {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Topology::SingleNode => *other == "single-node",
            Topology::ThreeNodeMultiZone => *other == "three-node-multi-zone",
        }
    }
}
impl std::cmp::PartialEq<Topology> for &str {
    fn eq(&self, other: &Topology) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClusterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}
