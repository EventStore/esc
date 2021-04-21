/*
 * Managed Event Store Databases API
 *
 * Works with Event Store databases
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: esc@eventstore.com
 * Generated by: https://openapi-generator.tech
 */

use crate::sender::EscRequestSender;
use reqwest::Method;

/// Create a cluster
pub async fn create_cluster(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    create_cluster_request: crate::mesdb::models::CreateClusterRequest,
) -> crate::Result<crate::mesdb::models::CreateClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id)
    );

    sender.send_request::<
        super::super::CreateClusterRequest,
        crate::mesdb::models::CreateClusterResponse
    >(
        Method::POST,
        url,
        Some(&create_cluster_request), None
    ).await
}
/// Deletes a cluster
pub async fn delete_cluster(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    cluster_id: crate::types::ClusterId,
) -> crate::Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id),
        clusterId = crate::utils::urlencode(cluster_id)
    );

    sender
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}
/// expands a cluster's disk
pub async fn expand_cluster_disk(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    cluster_id: crate::types::ClusterId,
    expand_cluster_disk_request: crate::mesdb::models::ExpandClusterDiskRequest,
) -> crate::Result<()> {
    let url = format!("/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}/disk/expand", organizationId=crate::utils::urlencode(organization_id), projectId=crate::utils::urlencode(project_id), clusterId=crate::utils::urlencode(cluster_id));

    sender
        .send_request::<super::super::ExpandClusterDiskRequest, ()>(
            Method::PUT,
            url,
            Some(&expand_cluster_disk_request),
            Some(()),
        )
        .await
}
/// Get a single cluster
pub async fn get_cluster(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    cluster_id: crate::types::ClusterId,
) -> crate::Result<crate::mesdb::models::GetClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id),
        clusterId = crate::utils::urlencode(cluster_id)
    );

    sender
        .send_request::<(), crate::mesdb::models::GetClusterResponse>(Method::GET, url, None, None)
        .await
}
/// List clusters
pub async fn list_clusters(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
) -> crate::Result<crate::mesdb::models::ListClustersResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id)
    );

    sender
        .send_request::<(), crate::mesdb::models::ListClustersResponse>(
            Method::GET,
            url,
            None,
            None,
        )
        .await
}
/// Get a single cluster
pub async fn update_cluster(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    cluster_id: crate::types::ClusterId,
    update_cluster_request: crate::mesdb::models::UpdateClusterRequest,
) -> crate::Result<crate::mesdb::models::GetClusterResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/clusters/{clusterId}",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id),
        clusterId = crate::utils::urlencode(cluster_id)
    );

    sender.send_request::<
        super::super::UpdateClusterRequest,
        crate::mesdb::models::GetClusterResponse
    >(
        Method::PUT,
        url,
        Some(&update_cluster_request), None
    ).await
}

pub struct Clusters {
    sender: EscRequestSender,
}

impl<'a> Clusters {
    pub fn new(sender: EscRequestSender) -> Self {
        Self { sender }
    }

    /// Create a cluster
    pub async fn create(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        create_cluster_request: crate::mesdb::models::CreateClusterRequest,
    ) -> crate::Result<crate::mesdb::models::CreateClusterResponse> {
        create_cluster(
            &self.sender,
            organization_id,
            project_id,
            create_cluster_request,
        )
        .await
    }

    /// Deletes a cluster
    pub async fn delete(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        cluster_id: crate::types::ClusterId,
    ) -> crate::Result<()> {
        delete_cluster(&self.sender, organization_id, project_id, cluster_id).await
    }

    /// expands a cluster's disk
    pub async fn expand_cluster_disk(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        cluster_id: crate::types::ClusterId,
        expand_cluster_disk_request: crate::mesdb::models::ExpandClusterDiskRequest,
    ) -> crate::Result<()> {
        expand_cluster_disk(
            &self.sender,
            organization_id,
            project_id,
            cluster_id,
            expand_cluster_disk_request,
        )
        .await
    }

    /// Get a single cluster
    pub async fn get(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        cluster_id: crate::types::ClusterId,
    ) -> crate::Result<crate::mesdb::models::GetClusterResponse> {
        get_cluster(&self.sender, organization_id, project_id, cluster_id).await
    }

    /// List clusters
    pub async fn list(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
    ) -> crate::Result<crate::mesdb::models::ListClustersResponse> {
        list_clusters(&self.sender, organization_id, project_id).await
    }

    /// Get a single cluster
    pub async fn update(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        cluster_id: crate::types::ClusterId,
        update_cluster_request: crate::mesdb::models::UpdateClusterRequest,
    ) -> crate::Result<crate::mesdb::models::GetClusterResponse> {
        update_cluster(
            &self.sender,
            organization_id,
            project_id,
            cluster_id,
            update_cluster_request,
        )
        .await
    }
} // end Clusters
