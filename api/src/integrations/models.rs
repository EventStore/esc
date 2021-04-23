#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "data")]
    pub data: crate::integrations::models::ConfigurationData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: crate::types::ConfigurationId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "updated")]
    pub updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum ConfigurationData {
    #[serde(rename = "OpsGenie")]
    OpsGenieConfiguration {
        /// API key used with the Ops Genie integration API
        #[serde(rename = "api_key")]
        api_key: String,
    },
    #[serde(rename = "Slack")]
    SlackConfiguration {
        /// Slack Channel to send messages to
        #[serde(rename = "channelId")]
        channel_id: String,
        /// API token for the Slack bot
        #[serde(rename = "token")]
        token: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateConfigurationRequest {
    #[serde(rename = "data")]
    pub data: crate::integrations::models::ConfigurationData,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateConfigurationResponse {
    #[serde(rename = "id")]
    pub id: crate::types::ConfigurationId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetConfigurationResponse {
    #[serde(rename = "configuration")]
    pub configuration: crate::integrations::models::Configuration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListConfigurationsResponse {
    #[serde(rename = "configurations")]
    pub configurations: Vec<crate::integrations::models::Configuration>,
}

/// OpsGenieConfiguration : Configuration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsGenieConfiguration {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "api_key")]
    pub api_key: String,
}

/// OpsGenieIntegrationConfiguration : Configuration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsGenieIntegrationConfiguration {
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

/// SlackConfiguration : Configuration for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackConfiguration {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// API token for the Slack bot
    #[serde(rename = "token")]
    pub token: String,
}

/// SlackIntegrationConfiguration : Configruation for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationConfiguration {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// API token for the Slack bot
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateConfigurationRequest {
    #[serde(rename = "data")]
    pub data: crate::integrations::models::ConfigurationData,
    #[serde(rename = "description")]
    pub description: String,
}
