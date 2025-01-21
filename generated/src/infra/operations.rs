use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// creates a new acl
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_acl_request`
pub async fn create_acl(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // Describes the new acl
    create_acl_request: CreateAclRequest,
) -> Result<CreateAclResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/acls",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateAclRequest, CreateAclResponse>(
            Method::POST,
            url,
            Some(&create_acl_request),
            None,
        )
        .await
}

/// creates a new network
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_network_request`
pub async fn create_network(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // Describes the new network
    create_network_request: CreateNetworkRequest,
) -> Result<CreateNetworkResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/networks",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateNetworkRequest, CreateNetworkResponse>(
            Method::POST,
            url,
            Some(&create_network_request),
            None,
        )
        .await
}

/// Creates a peering request
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_peering_request`
pub async fn create_peering(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // Describes the new peering
    create_peering_request: CreatePeeringRequest,
) -> Result<CreatePeeringResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreatePeeringRequest, CreatePeeringResponse>(
            Method::POST,
            url,
            Some(&create_peering_request),
            None,
        )
        .await
}

/// Creates peerings commands
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_peering_commands_request`
pub async fn create_peering_commands(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // TODO
    create_peering_commands_request: CreatePeeringCommandsRequest,
) -> Result<CreatePeeringCommandsResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings/commands",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreatePeeringCommandsRequest, CreatePeeringCommandsResponse>(
            Method::POST,
            url,
            Some(&create_peering_commands_request),
            None,
        )
        .await
}

/// deletes a acl
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `acl_id` - The id of the acl
pub async fn delete_acl(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    acl_id: AclId,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/acls/{aclId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        aclId = urlencode(acl_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes a network
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `network_id` - The id of the network
pub async fn delete_network(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    network_id: NetworkId,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/networks/{networkId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        networkId = urlencode(network_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes a peering
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `peering_id` - The id of the peering
pub async fn delete_peering(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    peering_id: PeeringId,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings/{peeringId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        peeringId = urlencode(peering_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// gets a single acl
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `acl_id` - The id of the acl
pub async fn get_acl(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    acl_id: AclId,
) -> Result<GetAclResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/acls/{aclId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        aclId = urlencode(acl_id),
    );
    client
        .send_request::<(), GetAclResponse>(Method::GET, url, None, None)
        .await
}

/// gets a single network
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `network_id` - The id of the network
pub async fn get_network(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    network_id: NetworkId,
) -> Result<GetNetworkResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/networks/{networkId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        networkId = urlencode(network_id),
    );
    client
        .send_request::<(), GetNetworkResponse>(Method::GET, url, None, None)
        .await
}

/// retrieves a peering
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `peering_id` - The id of the peering
pub async fn get_peering(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    peering_id: PeeringId,
) -> Result<GetPeeringResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings/{peeringId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        peeringId = urlencode(peering_id),
    );
    client
        .send_request::<(), GetPeeringResponse>(Method::GET, url, None, None)
        .await
}

/// lists all acls under the given project
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
pub async fn list_acls(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListAclsResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/acls",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListAclsResponse>(Method::GET, url, None, None)
        .await
}

/// lists all networks under the given project
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
pub async fn list_networks(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListNetworksResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/networks",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListNetworksResponse>(Method::GET, url, None, None)
        .await
}

/// list all peerings
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
pub async fn list_peerings(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListPeeringsResponse> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListPeeringsResponse>(Method::GET, url, None, None)
        .await
}

/// updates the given acl
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `acl_id` - The id of the acl
/// * `update_acl_request`
pub async fn update_acl(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    acl_id: AclId,
    // describes changes to make to an acl
    update_acl_request: UpdateAclRequest,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/acls/{aclId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        aclId = urlencode(acl_id),
    );
    client
        .send_request::<UpdateAclRequest, ()>(Method::PUT, url, Some(&update_acl_request), Some(()))
        .await
}

/// updates the given network
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `network_id` - The id of the network
/// * `update_network_request`
pub async fn update_network(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    network_id: NetworkId,
    // describes changes to make to a network
    update_network_request: UpdateNetworkRequest,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/networks/{networkId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        networkId = urlencode(network_id),
    );
    client
        .send_request::<UpdateNetworkRequest, ()>(
            Method::PUT,
            url,
            Some(&update_network_request),
            Some(()),
        )
        .await
}

/// updates a peering
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `peering_id` - The id of the peering
/// * `update_peering_request`
pub async fn update_peering(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    peering_id: PeeringId,
    // describes changes to make to a peering
    update_peering_request: UpdatePeeringRequest,
) -> Result<()> {
    let url = format!(
        "/infra/v1/organizations/{organizationId}/projects/{projectId}/peerings/{peeringId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        peeringId = urlencode(peering_id),
    );
    client
        .send_request::<UpdatePeeringRequest, ()>(
            Method::PUT,
            url,
            Some(&update_peering_request),
            Some(()),
        )
        .await
}
