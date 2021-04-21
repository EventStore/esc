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

/// Creates a new backup
pub async fn create_backup(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    create_backup_request: crate::mesdb::models::CreateBackupRequest,
) -> crate::Result<crate::mesdb::models::CreateBackupResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id)
    );

    sender.send_request::<
        super::super::CreateBackupRequest,
        crate::mesdb::models::CreateBackupResponse
    >(
        Method::POST,
        url,
        Some(&create_backup_request), None
    ).await
}
/// deletes a backup
pub async fn delete_backup(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    backup_id: crate::types::BackupId,
) -> crate::Result<()> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups/{backupId}",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id),
        backupId = crate::utils::urlencode(backup_id)
    );

    sender
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}
/// get a single backup
pub async fn get_backup(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
    backup_id: crate::types::BackupId,
) -> crate::Result<crate::mesdb::models::GetBackupResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups/{backupId}",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id),
        backupId = crate::utils::urlencode(backup_id)
    );

    sender
        .send_request::<(), crate::mesdb::models::GetBackupResponse>(Method::GET, url, None, None)
        .await
}
/// List backups
pub async fn list_backups(
    sender: &EscRequestSender,
    organization_id: crate::types::OrgId,
    project_id: crate::types::ProjectId,
) -> crate::Result<crate::mesdb::models::ListBackupsResponse> {
    let url = format!(
        "/mesdb/v1/organizations/{organizationId}/projects/{projectId}/backups",
        organizationId = crate::utils::urlencode(organization_id),
        projectId = crate::utils::urlencode(project_id)
    );

    sender
        .send_request::<(), crate::mesdb::models::ListBackupsResponse>(Method::GET, url, None, None)
        .await
}

pub struct Backups {
    sender: EscRequestSender,
}

impl<'a> Backups {
    pub fn new(sender: EscRequestSender) -> Self {
        Self { sender }
    }

    /// Creates a new backup
    pub async fn create(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        create_backup_request: crate::mesdb::models::CreateBackupRequest,
    ) -> crate::Result<crate::mesdb::models::CreateBackupResponse> {
        create_backup(
            &self.sender,
            organization_id,
            project_id,
            create_backup_request,
        )
        .await
    }

    /// deletes a backup
    pub async fn delete(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        backup_id: crate::types::BackupId,
    ) -> crate::Result<()> {
        delete_backup(&self.sender, organization_id, project_id, backup_id).await
    }

    /// get a single backup
    pub async fn get(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        backup_id: crate::types::BackupId,
    ) -> crate::Result<crate::mesdb::models::GetBackupResponse> {
        get_backup(&self.sender, organization_id, project_id, backup_id).await
    }

    /// List backups
    pub async fn list(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
    ) -> crate::Result<crate::mesdb::models::ListBackupsResponse> {
        list_backups(&self.sender, organization_id, project_id).await
    }
} // end Backups
