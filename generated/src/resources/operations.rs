use super::formats::*;
use super::schemas::*;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// Creates a new organization
///
/// # Arguments
///
/// * `create_organization_request`
pub async fn create_organization(
    client: &Client,
    // describes new organization
    create_organization_request: CreateOrganizationRequest,
) -> Result<CreateOrganizationResponse> {
    client
        .send_request::<CreateOrganizationRequest, CreateOrganizationResponse>(
            Method::POST,
            "/resources/v1/organizations".to_string(),
            Some(&create_organization_request),
            None,
        )
        .await
}

/// Creates a new project
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to retrieve
/// * `create_project_request`
pub async fn create_project(
    client: &Client,
    organization_id: OrganizationId,
    // describes a new project
    create_project_request: CreateProjectRequest,
) -> Result<CreateProjectResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/projects",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<CreateProjectRequest, CreateProjectResponse>(
            Method::POST,
            url,
            Some(&create_project_request),
            None,
        )
        .await
}

/// Deletes an organization by ID.
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to delete
pub async fn delete_organization(client: &Client, organization_id: OrganizationId) -> Result<()> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// Deletes project
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to delete
/// * `project_id` - The id of the project to delete
pub async fn delete_project(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<()> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/projects/{projectId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// Gets the mfa status of an organization
///
/// # Arguments
///
/// * `organization_id` - The id of the organization
pub async fn get_mfa_status(client: &Client, organization_id: OrganizationId) -> Result<MfaStatus> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/mfa",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), MfaStatus>(Method::GET, url, None, None)
        .await
}

/// Gets a single organization by ID.
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to retrieve
pub async fn get_organization(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<GetOrganizationResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), GetOrganizationResponse>(Method::GET, url, None, None)
        .await
}

/// Get project
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to retrieve
/// * `project_id` - The id of the project to retrieve
pub async fn get_project(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<GetProjectResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/projects/{projectId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), GetProjectResponse>(Method::GET, url, None, None)
        .await
}

/// Lists organizations under the account owned by the credentials
///
/// # Arguments
///
pub async fn list_organizations(client: &Client) -> Result<ListOrganizationsResponse> {
    client
        .send_request::<(), ListOrganizationsResponse>(
            Method::GET,
            "/resources/v1/organizations".to_string(),
            None,
            None,
        )
        .await
}

/// List projects
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to retrieve
pub async fn list_projects(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<ListProjectsResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/projects",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ListProjectsResponse>(Method::GET, url, None, None)
        .await
}

/// Changes the status of MFA for an organization
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `mfa_status`
pub async fn update_mfa(
    client: &Client,
    organization_id: OrganizationId,
    // The desired status of MFA
    mfa_status: MfaStatus,
) -> Result<UpdateMfaResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/mfa",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<MfaStatus, UpdateMfaResponse>(Method::POST, url, Some(&mfa_status), None)
        .await
}

/// Deletes an organization by ID.
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to alter
/// * `update_organization_request`
pub async fn update_organization(
    client: &Client,
    organization_id: OrganizationId,
    // describes new organization
    update_organization_request: UpdateOrganizationRequest,
) -> Result<GetOrganizationResponse> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<UpdateOrganizationRequest, GetOrganizationResponse>(
            Method::PUT,
            url,
            Some(&update_organization_request),
            None,
        )
        .await
}

/// Updates a project
///
/// # Arguments
///
/// * `organization_id` - The id of the organization to delete
/// * `project_id` - The id of the project to delete
/// * `update_project_request`
pub async fn update_project(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // describes a new project
    update_project_request: UpdateProjectRequest,
) -> Result<()> {
    let url = format!(
        "/resources/v1/organizations/{organizationId}/projects/{projectId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<UpdateProjectRequest, ()>(
            Method::PUT,
            url,
            Some(&update_project_request),
            Some(()),
        )
        .await
}
