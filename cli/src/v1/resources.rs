use std::fmt::Formatter;

use super::common::ToV1;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct OrgId(pub String);

impl std::fmt::Display for OrgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl ToV1 for esc_api::resources::OrganizationId {
    type V1Type = OrgId;
    fn to_v1(self) -> Self::V1Type {
        OrgId(self.0)
    }
}
