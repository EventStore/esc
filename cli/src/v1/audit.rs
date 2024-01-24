use chrono::{DateTime, Utc};
use esc_api::OrgId;

use super::common::{List, ToV1};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Log {
    pub organization_id: OrgId,
    pub message: String,
    pub user: String,
    pub service: String,
    pub urn: String,
    pub time: DateTime<Utc>,
}

impl ToV1 for esc_api::audit::Log {
    type V1Type = V1Log;
    fn to_v1(self) -> Self::V1Type {
        V1Log {
            time: self.time,
            message: self.message,
            user: self.user,
            service: self.service,
            urn: self.urn,
            organization_id: self.organization_id,
        }
    }
}

impl ToV1 for esc_api::audit::GetAuditResponse {
    type V1Type = List<V1Log>;
    fn to_v1(self) -> Self::V1Type {
        List(self.logs.into_iter().map(|p| p.to_v1()).collect())
    }
}
