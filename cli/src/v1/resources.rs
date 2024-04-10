use chrono::{DateTime, Utc};
use esc_api::resources::MfaStatus;
use std::fmt::Formatter;

use super::common::{List, ToV1};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: OrgId,
    pub name: String,
    pub created: DateTime<Utc>,
}

impl ToV1 for esc_api::resources::Organization {
    type V1Type = Organization;
    fn to_v1(self) -> Self::V1Type {
        Organization {
            created: self.created,
            id: self.id.to_v1(),
            name: self.name,
        }
    }
}

impl ToV1 for esc_api::resources::CreateOrganizationResponse {
    type V1Type = OrgId;
    fn to_v1(self) -> Self::V1Type {
        self.id.to_v1()
    }
}

impl ToV1 for esc_api::resources::GetOrganizationResponse {
    type V1Type = Organization;
    fn to_v1(self) -> Self::V1Type {
        self.organization.to_v1()
    }
}

impl ToV1 for esc_api::resources::ListOrganizationsResponse {
    type V1Type = List<Organization>;
    fn to_v1(self) -> Self::V1Type {
        List(self.organizations.into_iter().map(|o| o.to_v1()).collect())
    }
}

impl ToV1 for esc_api::resources::MfaStatus {
    type V1Type = MfaStatus;
    fn to_v1(self) -> Self::V1Type {
        MfaStatus {
            mfa_enabled: self.mfa_enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: esc_api::resources::ProjectId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub name: String,
    pub created: DateTime<Utc>,
}

impl ToV1 for esc_api::resources::Project {
    type V1Type = Project;
    fn to_v1(self) -> Self::V1Type {
        Project {
            created: self.created,
            id: self.id,
            name: self.name,
            org_id: self.organization_id.to_v1(),
        }
    }
}

impl ToV1 for esc_api::resources::CreateProjectResponse {
    type V1Type = esc_api::resources::ProjectId;
    fn to_v1(self) -> Self::V1Type {
        self.id
    }
}

impl ToV1 for esc_api::resources::GetProjectResponse {
    type V1Type = Project;
    fn to_v1(self) -> Self::V1Type {
        self.project.to_v1()
    }
}

impl ToV1 for esc_api::resources::ListProjectsResponse {
    type V1Type = List<Project>;
    fn to_v1(self) -> Self::V1Type {
        List(self.projects.into_iter().map(|p| p.to_v1()).collect())
    }
}
