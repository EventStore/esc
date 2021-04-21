#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "linkedResource", skip_serializing_if = "Option::is_none")]
    pub linked_resource: Option<String>,
    #[serde(rename = "id")]
    pub id: crate::types::BackupId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "serverVersion")]
    pub server_version: String,
    #[serde(rename = "serverVersionTag")]
    pub server_version_tag: String,
    #[serde(rename = "sizeGb")]
    pub size_gb: i32,
    #[serde(rename = "sourceClusterId")]
    pub source_cluster_id: crate::types::ClusterId,
    #[serde(rename = "sourceClusterDescription")]
    pub source_cluster_description: String,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "canExpandDisk")]
    pub can_expand_disk: bool,
    #[serde(rename = "cloudIntegratedAuthentication")]
    pub cloud_integrated_authentication: bool,
    #[serde(rename = "created")]
    pub created: String,
    /// A human-readable description of the cluster
    #[serde(rename = "description")]
    pub description: String,
    /// Total disk capacity in Gigabytes (GB)
    #[serde(rename = "diskSizeGb")]
    pub disk_size_gb: i32,
    #[serde(rename = "diskType")]
    pub disk_type: String,
    #[serde(rename = "id")]
    pub id: crate::types::ClusterId,
    /// Type of instance, based on its hardware. For example, it could be F1 for a micro or C4 for a large instance
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    #[serde(rename = "networkId")]
    pub network_id: crate::types::NetworkId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "projectionLevel")]
    pub projection_level: crate::mesdb::models::ProjectionLevel,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "serverVersion")]
    pub server_version: String,
    #[serde(rename = "serverVersionTag")]
    pub server_version_tag: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "topology")]
    pub topology: crate::mesdb::models::Topology,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBackupRequest {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "sourceClusterId")]
    pub source_cluster_id: crate::types::ClusterId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBackupResponse {
    #[serde(rename = "id")]
    pub id: crate::types::BackupId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    /// A human-readable description of the cluster
    #[serde(rename = "description")]
    pub description: String,
    /// Total disk capacity in Gigabytes (GB)
    #[serde(rename = "diskSizeGb")]
    pub disk_size_gb: i32,
    #[serde(rename = "diskType")]
    pub disk_type: String,
    /// Type of instance, based on its hardware. For example, it could be F1 for a micro or C4 for a large instance
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// The network id the cluster will be set on
    #[serde(rename = "networkId")]
    pub network_id: crate::types::NetworkId,
    #[serde(rename = "projectionLevel")]
    pub projection_level: crate::mesdb::models::ProjectionLevel,
    #[serde(rename = "serverVersion")]
    pub server_version: String,
    /// Optional id of backup to restore
    #[serde(rename = "sourceBackupId", skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(rename = "topology")]
    pub topology: crate::mesdb::models::Topology,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateClusterResponse {
    #[serde(rename = "id")]
    pub id: crate::types::ClusterId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpandClusterDiskRequest {
    /// Total disk capacity in Gigabytes (GB)
    #[serde(rename = "diskSizeGb")]
    pub disk_size_gb: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBackupResponse {
    #[serde(rename = "backup")]
    pub backup: crate::mesdb::models::Backup,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetClusterResponse {
    #[serde(rename = "cluster")]
    pub cluster: crate::mesdb::models::Cluster,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    #[serde(rename = "backups")]
    pub backups: Vec<crate::mesdb::models::Backup>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListClustersResponse {
    #[serde(rename = "clusters")]
    pub clusters: Vec<crate::mesdb::models::Cluster>,
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

/// ProjectionLevel : The projection level of your database. Can be off, system or user

/// The projection level of your database. Can be off, system or user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectionLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

impl ProjectionLevel {
    pub fn from_str(src: &str) -> Result<ProjectionLevel, String> {
        match src {
            "off" => Ok(ProjectionLevel::Off),
            "system" => Ok(ProjectionLevel::System),
            "user" => Ok(ProjectionLevel::User),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["off", "system", "user",]
            )),
        }
    }
}

/// Topology : Either single-node or three-node-multi-zone

/// Either single-node or three-node-multi-zone
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Topology {
    #[serde(rename = "single-node")]
    SingleNode,
    #[serde(rename = "three-node-multi-zone")]
    ThreeNodeMultiZone,
}

impl Topology {
    pub fn from_str(src: &str) -> Result<Topology, String> {
        match src {
            "single-node" => Ok(Topology::SingleNode),
            "three-node-multi-zone" => Ok(Topology::ThreeNodeMultiZone),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["single-node", "three-node-multi-zone",]
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateClusterRequest {
    /// A human-readable description of the cluster
    #[serde(rename = "description")]
    pub description: String,
}
