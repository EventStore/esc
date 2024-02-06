use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// Creates a new backup
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the backup is owned by
/// * `project_id` - The id of the project the backup is grouped under
/// * `create_backup_request`
pub async fn create_backup(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // describes a new backup
    create_backup_request: CreateBackupRequest,
) -> Result<CreateBackupResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateBackupRequest, CreateBackupResponse>(
            Method::POST,
            url,
            Some(&create_backup_request),
            None,
        )
        .await
}

/// Create a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `create_cluster_request`
pub async fn create_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // describes a new cluster
    create_cluster_request: CreateClusterRequest,
) -> Result<CreateClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateClusterRequest, CreateClusterResponse>(
            Method::POST,
            url,
            Some(&create_cluster_request),
            None,
        )
        .await
}

/// deletes a backup
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the backup is owned by
/// * `project_id` - The id of the project the backup is grouped under
/// * `backup_id` - The id of the backup to retrieve
pub async fn delete_backup(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    backup_id: BackupId,
) -> Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups/{backupId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        backupId = urlencode(backup_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// Deletes a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster to delete
pub async fn delete_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
) -> Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// expands a cluster's disk
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster
/// * `expand_cluster_disk_request`
pub async fn expand_cluster_disk(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
    // describes how to expand the disk
    expand_cluster_disk_request: ExpandClusterDiskRequest,
) -> Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}/disk/expand",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<ExpandClusterDiskRequest, ()>(
            Method::PUT,
            url,
            Some(&expand_cluster_disk_request),
            Some(()),
        )
        .await
}

/// get a single backup
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the backup is owned by
/// * `project_id` - The id of the project the backup is grouped under
/// * `backup_id` - The id of the backup to retrieve
pub async fn get_backup(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    backup_id: BackupId,
) -> Result<GetBackupResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups/{backupId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        backupId = urlencode(backup_id),
    );
    client
        .send_request::<(), GetBackupResponse>(Method::GET, url, None, None)
        .await
}

/// Get a single cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster to retrieve
pub async fn get_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
) -> Result<GetClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<(), GetClusterResponse>(Method::GET, url, None, None)
        .await
}

/// List backups
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the backup is owned by
/// * `project_id` - The id of the project the backup is grouped under
pub async fn list_backups(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListBackupsResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListBackupsResponse>(Method::GET, url, None, None)
        .await
}

/// List clusters
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
pub async fn list_clusters(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListClustersResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListClustersResponse>(Method::GET, url, None, None)
        .await
}

/// resize a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster
/// * `resize_cluster_request`
pub async fn resize_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
    // describes how to resize the cluster
    resize_cluster_request: ResizeClusterRequest,
) -> Result<ResizeClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}/commands/resize",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<ResizeClusterRequest, ResizeClusterResponse>(
            Method::PUT,
            url,
            Some(&resize_cluster_request),
            None,
        )
        .await
}

/// restart a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster
pub async fn restart_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
) -> Result<RestartClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}/commands/restart",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<(), RestartClusterResponse>(Method::PUT, url, None, None)
        .await
}

/// Updates a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster to update
/// * `update_cluster_request`
pub async fn update_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
    // describes changes to make to a cluster
    update_cluster_request: UpdateClusterRequest,
) -> Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<UpdateClusterRequest, ()>(
            Method::PUT,
            url,
            Some(&update_cluster_request),
            Some(()),
        )
        .await
}

/// upgrades a cluster
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `cluster_id` - The id of the cluster
/// * `upgrade_cluster_request`
pub async fn upgrade_cluster(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    cluster_id: ClusterId,
    // describes how to upgrade the cluster
    upgrade_cluster_request: UpgradeClusterRequest,
) -> Result<UpgradeClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}/commands/upgrade",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        clusterId = urlencode(cluster_id),
    );
    client
        .send_request::<UpgradeClusterRequest, UpgradeClusterResponse>(
            Method::PUT,
            url,
            Some(&upgrade_cluster_request),
            None,
        )
        .await
}
