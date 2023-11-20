use super::GetAuditResponse;
use crate::resources::formats::OrganizationId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// gets audit log for an entire organization
/// # Arguments
///
/// * `org_id` - The id of the organization for which to get audit logs
/// * `before` - Only get audit logs before this timestamp
/// * `after` - Only get audit logs after this timestamp
/// * `limit` - The maximum number of records to retrieve
/// * `get_audit_by_org`
pub async fn get_audit_by_org(
    client: &Client,
    org_id: OrganizationId,
    before: String,
    after: String,
    limit: String,
) -> Result<GetAuditResponse> {
    let mut url = format!(
        "/audit/v1/organizations/{organizationId}/log?",
        organizationId = urlencode(org_id)
    );
    if before != "" {
        let query = format!("before={beforeDate}&", beforeDate = urlencode(before));
        url = [url, query].join("");
    }
    if after != "" {
        let query = format!("after={afterDate}&", afterDate = urlencode(after));
        url = [url, query].join("");
    }
    if limit != "" {
        let query = format!("limit={limitVal}&", limitVal = urlencode(limit));
        url = [url, query].join("");
    }

    client
        .send_request::<(), GetAuditResponse>(Method::GET, url, None, None)
        .await
}

/// gets audit log for a user
/// # Arguments
/// * `org_id` - The id of the organization for which to get audit logs
/// * `before` - Only get audit logs before this timestamp
/// * `after` - Only get audit logs after this timestamp
/// * `limit` - The maximum number of records to retrieve
/// * `get_audit_by_user`
pub async fn get_audit_by_user(
    client: &Client,
    org_id: String,
    before: String,
    after: String,
    limit: String,
) -> Result<GetAuditResponse> {
    let mut url = "/audit/v1/user/log?".to_string();
    if before != "" {
        let query = format!("before={beforeDate}&", beforeDate = urlencode(before));
        url = [url, query].join("");
    }
    if after != "" {
        let query = format!("after={afterDate}&", afterDate = urlencode(after));
        url = [url, query].join("");
    }
    if limit != "" {
        let query = format!("limit={limitVal}&", limitVal = urlencode(limit));
        url = [url, query].join("");
    }
    if org_id != "" {
        let query = format!("org={orgVal}&", orgVal = urlencode(org_id));
        url = [url, query].join("");
    }
    client
        .send_request::<(), GetAuditResponse>(Method::GET, url, None, None)
        .await
}
