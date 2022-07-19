use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// Creates a new integration
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_integration_request`
pub async fn create_integration(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // Describes the new integration
    create_integration_request: CreateIntegrationRequest,
) -> Result<CreateIntegrationResponse> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateIntegrationRequest, CreateIntegrationResponse>(
            Method::POST,
            url,
            Some(&create_integration_request),
            None,
        )
        .await
}

/// deletes a integration
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `integration_id` - The id of the integration
pub async fn delete_integration(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    integration_id: IntegrationId,
) -> Result<()> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        integrationId = urlencode(integration_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// retrieves a integration
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `integration_id` - The id of the integration
pub async fn get_integration(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    integration_id: IntegrationId,
) -> Result<GetIntegrationResponse> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        integrationId = urlencode(integration_id),
    );
    client
        .send_request::<(), GetIntegrationResponse>(Method::GET, url, None, None)
        .await
}

/// returns options for the integrations API
/// # Arguments
///
/// * `organization_id` - the id of the organization
pub async fn get_integrations_options(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<IntegrationsOptionsResponse> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/integrations/options",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), IntegrationsOptionsResponse>(Method::GET, url, None, None)
        .await
}

/// list all integrations
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
pub async fn list_integrations(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListIntegrationsResponse> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListIntegrationsResponse>(Method::GET, url, None, None)
        .await
}

/// Sends a message to an integration sink
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `integration_id` - The id of the integration
pub async fn test_integration(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    integration_id: IntegrationId,
) -> Result<()> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}/test",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        integrationId = urlencode(integration_id),
    );
    client
        .send_request::<(), ()>(Method::POST, url, None, Some(()))
        .await
}

/// updates a integration
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `integration_id` - The id of the integration
/// * `update_integration_request`
pub async fn update_integration(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    integration_id: IntegrationId,
    // describes changes to make to a integration
    update_integration_request: UpdateIntegrationRequest,
) -> Result<()> {
    let url = format!(
        "/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        integrationId = urlencode(integration_id),
    );
    client
        .send_request::<UpdateIntegrationRequest, ()>(
            Method::PUT,
            url,
            Some(&update_integration_request),
            Some(()),
        )
        .await
}
