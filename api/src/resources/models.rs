#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationResponse {
    #[serde(rename = "id")]
    pub id: crate::types::OrgId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectResponse {
    #[serde(rename = "id")]
    pub id: crate::types::ProjectId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationResponse {
    #[serde(rename = "organization")]
    pub organization: crate::resources::models::Organization,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectResponse {
    #[serde(rename = "project")]
    pub project: crate::resources::models::Project,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOrganizationsResponse {
    #[serde(rename = "organizations")]
    pub organizations: Vec<crate::resources::models::Organization>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    #[serde(rename = "projects")]
    pub projects: Vec<crate::resources::models::Project>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "id")]
    pub id: crate::types::OrgId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "provisioningEnabled")]
    pub provisioning_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "id")]
    pub id: crate::types::ProjectId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrganizationRequest {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    #[serde(rename = "name")]
    pub name: String,
}
