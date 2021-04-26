#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegrationRequest {
    #[serde(rename = "data")]
    pub data: crate::integrate::models::IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegrationResponse {
    #[serde(rename = "id")]
    pub id: crate::types::IntegrationId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIntegrationResponse {
    #[serde(rename = "integration")]
    pub integration: crate::integrate::models::Integration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "data")]
    pub data: crate::integrate::models::IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: crate::types::IntegrationId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "updated")]
    pub updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum IntegrationData {
    #[serde(rename = "OpsGenie")]
    OpsGenieIntegration {
        /// API key used with the Ops Genie integration API
        #[serde(rename = "api_key")]
        api_key: String,
    },
    #[serde(rename = "Slack")]
    SlackIntegration {
        /// Slack Channel to send messages to
        #[serde(rename = "channelId")]
        channel_id: String,
        /// API token for the Slack bot
        #[serde(rename = "token")]
        token: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIntegrationsResponse {
    #[serde(rename = "integrations")]
    pub integrations: Vec<crate::integrate::models::Integration>,
}

/// OpsGenieIntegration : Integration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsGenieIntegration {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "api_key")]
    pub api_key: String,
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

/// SlackIntegration : Integration for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegration {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// API token for the Slack bot
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRequest {
    #[serde(rename = "data")]
    pub data: crate::integrate::models::IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
}
