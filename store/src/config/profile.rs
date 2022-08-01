use esc_client_generated::resources::OrganizationId;
use esc_client_generated::resources::ProjectId;

use super::url_visitor::{deserialize_url, serialize_url};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganizationId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url",
        default
    )]
    pub api_base_url: Option<url::Url>,
}
