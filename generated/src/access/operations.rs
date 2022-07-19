use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// creates a new group
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `create_group_request`
pub async fn create_group(
    client: &Client,
    organization_id: OrganizationId,
    // describes the group to create
    create_group_request: CreateGroupRequest,
) -> Result<CreateGroupResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/groups",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<CreateGroupRequest, CreateGroupResponse>(
            Method::POST,
            url,
            Some(&create_group_request),
            None,
        )
        .await
}

/// creates a new invite
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `create_invite_request`
pub async fn create_invite(
    client: &Client,
    organization_id: OrganizationId,
    // describes the new invite
    create_invite_request: CreateInviteRequest,
) -> Result<CreateInviteResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/invites",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<CreateInviteRequest, CreateInviteResponse>(
            Method::POST,
            url,
            Some(&create_invite_request),
            None,
        )
        .await
}

/// creates a new policy
/// # Arguments
///
/// * `organization_id` - The organization id the policy will relate to
/// * `create_policy_request`
pub async fn create_policy(
    client: &Client,
    organization_id: OrganizationId,
    // describes the policy to create
    create_policy_request: CreatePolicyRequest,
) -> Result<CreatePolicyResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/policies",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<CreatePolicyRequest, CreatePolicyResponse>(
            Method::POST,
            url,
            Some(&create_policy_request),
            None,
        )
        .await
}

/// deletes a group
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `group_id` - the id of the group
pub async fn delete_group(
    client: &Client,
    organization_id: OrganizationId,
    group_id: GroupId,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/groups/{groupId}",
        organizationId = urlencode(organization_id),
        groupId = urlencode(group_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes an invite
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `invite_id` - the id of the invite
pub async fn delete_invite(
    client: &Client,
    organization_id: OrganizationId,
    invite_id: InviteId,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/invites/{inviteId}",
        organizationId = urlencode(organization_id),
        inviteId = urlencode(invite_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes a member
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `member_id` - the id of the member
pub async fn delete_member(
    client: &Client,
    organization_id: OrganizationId,
    member_id: MemberId,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/members/{memberId}",
        organizationId = urlencode(organization_id),
        memberId = urlencode(member_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes a policy
/// # Arguments
///
/// * `organization_id` - The organization id the policy will relate to
/// * `policy_id` - the ID of the policy to delete
pub async fn delete_policy(
    client: &Client,
    organization_id: OrganizationId,
    policy_id: PolicyId,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/policies/{policyId}",
        organizationId = urlencode(organization_id),
        policyId = urlencode(policy_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// deletes a token TODO - this likely needs to change the security stuff as well
/// # Arguments
///
/// * `token_id` - the id of the token
pub async fn delete_user_token(client: &Client, token_id: TokenId) -> Result<()> {
    let url = format!(
        "/access/v1/tokens/user/{tokenId}",
        tokenId = urlencode(token_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// gets a single group
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `group_id` - the id of the group
pub async fn get_group(
    client: &Client,
    organization_id: OrganizationId,
    group_id: GroupId,
) -> Result<GetGroupResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/groups/{groupId}",
        organizationId = urlencode(organization_id),
        groupId = urlencode(group_id),
    );
    client
        .send_request::<(), GetGroupResponse>(Method::GET, url, None, None)
        .await
}

/// gets a member
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `member_id` - the id of the member
pub async fn get_member(
    client: &Client,
    organization_id: OrganizationId,
    member_id: MemberId,
) -> Result<GetMemberResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/members/{memberId}",
        organizationId = urlencode(organization_id),
        memberId = urlencode(member_id),
    );
    client
        .send_request::<(), GetMemberResponse>(Method::GET, url, None, None)
        .await
}

/// gets a single policy
/// # Arguments
///
/// * `organization_id` - The organization id the policy will relate to
/// * `policy_id` - the id of the policy
pub async fn get_policy(
    client: &Client,
    organization_id: OrganizationId,
    policy_id: PolicyId,
) -> Result<GetPolicyResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/policies/{policyId}",
        organizationId = urlencode(organization_id),
        policyId = urlencode(policy_id),
    );
    client
        .send_request::<(), GetPolicyResponse>(Method::GET, url, None, None)
        .await
}

/// gets the settings of an organization
/// # Arguments
///
/// * `organization_id` - The id of the organization
pub async fn get_settings(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<GetSettingsResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/settings",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), GetSettingsResponse>(Method::GET, url, None, None)
        .await
}

/// list groups
/// # Arguments
///
/// * `organization_id` - The id of the organization
pub async fn list_groups(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<ListGroupsResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/groups",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ListGroupsResponse>(Method::GET, url, None, None)
        .await
}

/// lists invites
/// # Arguments
///
/// * `organization_id` - The id of the organization
pub async fn list_invites(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<ListInvitesResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/invites",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ListInvitesResponse>(Method::GET, url, None, None)
        .await
}

/// lists members
/// # Arguments
///
/// * `organization_id` - The id of the organization
pub async fn list_members(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<ListMembersResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/members",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ListMembersResponse>(Method::GET, url, None, None)
        .await
}

/// gets a list of policies
/// # Arguments
///
/// * `organization_id` - The organization id the policy will relate to
pub async fn list_policies(
    client: &Client,
    organization_id: OrganizationId,
) -> Result<ListPoliciesResponse> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/policies",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<(), ListPoliciesResponse>(Method::GET, url, None, None)
        .await
}

/// fetchs a token user. TODO - this is probably not described correctly. Likely the security setting will need to change here to work correctly.
/// # Arguments
///
pub async fn list_user_tokens(client: &Client) -> Result<ListUserRefreshTokensResponse> {
    client
        .send_request::<(), ListUserRefreshTokensResponse>(
            Method::GET,
            "/access/v1/tokens/user".to_string(),
            None,
            None,
        )
        .await
}

/// checks to see if a policy is accepted
/// # Arguments
///
/// * `policy_allowed_request`
pub async fn policy_allowed(
    client: &Client,
    // describes the policy
    policy_allowed_request: PolicyAllowedRequest,
) -> Result<PolicyAllowedResponse> {
    client
        .send_request::<PolicyAllowedRequest, PolicyAllowedResponse>(
            Method::POST,
            "/access/v1/policy/allowed".to_string(),
            Some(&policy_allowed_request),
            None,
        )
        .await
}

/// resends an invite
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `resend_invite_request`
pub async fn resend_invite(
    client: &Client,
    organization_id: OrganizationId,
    // describes the new invite to be resent
    resend_invite_request: ResendInviteRequest,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/invites",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<ResendInviteRequest, ()>(
            Method::PUT,
            url,
            Some(&resend_invite_request),
            Some(()),
        )
        .await
}

/// updates a group
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `group_id` - the id of the group
/// * `update_group_request`
pub async fn update_group(
    client: &Client,
    organization_id: OrganizationId,
    group_id: GroupId,
    // describes the updates to the group
    update_group_request: UpdateGroupRequest,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/groups/{groupId}",
        organizationId = urlencode(organization_id),
        groupId = urlencode(group_id),
    );
    client
        .send_request::<UpdateGroupRequest, ()>(
            Method::PUT,
            url,
            Some(&update_group_request),
            Some(()),
        )
        .await
}

/// updates the member
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `member_id` - TODO
/// * `update_member_request`
pub async fn update_member(
    client: &Client,
    organization_id: OrganizationId,
    member_id: MemberId,
    // describes what to update
    update_member_request: UpdateMemberRequest,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/members/{memberId}",
        organizationId = urlencode(organization_id),
        memberId = urlencode(member_id),
    );
    client
        .send_request::<UpdateMemberRequest, ()>(
            Method::PUT,
            url,
            Some(&update_member_request),
            Some(()),
        )
        .await
}

/// modifies the policy
/// # Arguments
///
/// * `organization_id` - The organization id the policy will relate to
/// * `policy_id` - the id of the policy
/// * `update_policy_request`
pub async fn update_policy(
    client: &Client,
    organization_id: OrganizationId,
    policy_id: PolicyId,
    // describes the elements of the policy to update
    update_policy_request: UpdatePolicyRequest,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/policies/{policyId}",
        organizationId = urlencode(organization_id),
        policyId = urlencode(policy_id),
    );
    client
        .send_request::<UpdatePolicyRequest, ()>(
            Method::PUT,
            url,
            Some(&update_policy_request),
            Some(()),
        )
        .await
}

/// updates the settings of an organization
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `update_settings_request`
pub async fn update_setting(
    client: &Client,
    organization_id: OrganizationId,
    // describes what to update
    update_settings_request: UpdateSettingsRequest,
) -> Result<()> {
    let url = format!(
        "/access/v1/organizations/{organizationId}/settings",
        organizationId = urlencode(organization_id),
    );
    client
        .send_request::<UpdateSettingsRequest, ()>(
            Method::PUT,
            url,
            Some(&update_settings_request),
            Some(()),
        )
        .await
}
