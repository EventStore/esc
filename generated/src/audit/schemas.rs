use chrono::DateTime;
use chrono::Utc;

use crate::resources::OrganizationId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditByOrgRequest {
    pub org_id: String,
    pub before: String,
    pub after: String,
    pub limit: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditByUserRequest {
    pub org: String,
    pub before: String,
    pub after: String,
    pub limit: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditResponse {
    pub logs: Vec<Log>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub organization_id: OrganizationId,
    pub message: String,
    pub user: String,
    pub service: String,
    pub urn: String,
    pub time: DateTime<Utc>,
}
