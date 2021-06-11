#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum CreateIntegrationData {
    #[serde(rename = "opsGenie")]
    CreateOpsGenieIntegrationData {
        /// API key used with the Ops Genie integration API
        #[serde(rename = "apiKey")]
        api_key: String,
        #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
        source: Option<String>,
    },
    #[serde(rename = "pagerDuty")]
    CreatePagerDutyIntegrationData {
        /// API token for PagerDuty
        #[serde(rename = "authToken")]
        auth_token: String,
        /// The name of the service for the integration
        #[serde(rename = "service")]
        service: String,
        /// The name for the author of incident
        #[serde(rename = "user")]
        user: String,
        #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
        source: Option<String>,
    },
    #[serde(rename = "slack")]
    CreateSlackIntegrationData {
        /// Slack Channel to send messages to
        #[serde(rename = "channelId")]
        channel_id: String,
        /// API token for the Slack bot
        #[serde(rename = "token")]
        token: String,
        #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
        source: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegrationRequest {
    #[serde(rename = "data")]
    pub data: crate::integrate::models::CreateIntegrationData,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegrationResponse {
    #[serde(rename = "id")]
    pub id: crate::types::IntegrationId,
}

/// CreateOpsGenieIntegrationData : Create integration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOpsGenieIntegrationData {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// CreatePagerDutyIntegrationData : Create integration for the PagerDuty API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePagerDutyIntegrationData {
    /// API token for PagerDuty
    #[serde(rename = "authToken")]
    pub auth_token: String,
    /// The name of the service for the integration
    #[serde(rename = "service")]
    pub service: String,
    /// The name for the author of incident
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// CreateSlackIntegrationData : Create integration for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSlackIntegrationData {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// API token for the Slack bot
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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
    #[serde(rename = "status")]
    pub status: crate::integrate::models::IntegrationStatus,
    #[serde(rename = "updated")]
    pub updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum IntegrationData {
    #[serde(rename = "opsGenie")]
    OpsGenieIntegrationData {
        /// API key used with the Ops Genie integration API
        #[serde(rename = "apiKeyDisplay")]
        api_key_display: String,
        /// Source of data for integration
        #[serde(rename = "source")]
        source: String,
    },
    #[serde(rename = "pagerDuty")]
    PagerDutyIntegrationData {
        /// API token for PagerDuty
        #[serde(rename = "authTokenDisplay", skip_serializing_if = "Option::is_none")]
        auth_token_display: Option<String>,
        /// The name of the service for the integration
        #[serde(rename = "service")]
        service: String,
        /// The name for the author of incident
        #[serde(rename = "user")]
        user: String,
        #[serde(rename = "source")]
        source: String,
    },
    #[serde(rename = "slack")]
    SlackIntegrationData {
        /// Slack Channel to send messages to
        #[serde(rename = "channelId")]
        channel_id: String,
        /// API token for the Slack bot
        #[serde(rename = "tokenDisplay")]
        token_display: String,
        /// Source of data for integration
        #[serde(rename = "source")]
        source: String,
    },
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntegrationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
}

impl IntegrationStatus {
    pub fn from_str(src: &str) -> Result<IntegrationStatus, String> {
        match src {
            "active" => Ok(IntegrationStatus::Active),
            "deleted" => Ok(IntegrationStatus::Deleted),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["active", "deleted",]
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIntegrationsResponse {
    #[serde(rename = "integrations")]
    pub integrations: Vec<crate::integrate::models::Integration>,
}

/// OpsGenieIntegrationData : Integration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsGenieIntegrationData {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "apiKeyDisplay")]
    pub api_key_display: String,
    /// Source of data for integration
    #[serde(rename = "source")]
    pub source: String,
}

/// PagerDutyIntegrationData : Integration for the PagerDuty API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyIntegrationData {
    /// API token for PagerDuty
    #[serde(rename = "authTokenDisplay", skip_serializing_if = "Option::is_none")]
    pub auth_token_display: Option<String>,
    /// The name of the service for the integration
    #[serde(rename = "service")]
    pub service: String,
    /// The name for the author of incident
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "source")]
    pub source: String,
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

/// SlackIntegrationData : Integration for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackIntegrationData {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// API token for the Slack bot
    #[serde(rename = "tokenDisplay")]
    pub token_display: String,
    /// Source of data for integration
    #[serde(rename = "source")]
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationData {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Slack Channel to send messages to
    #[serde(rename = "channelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// API token for the Slack bot
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// API token for PagerDuty
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// The name of the service for the integration
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The name for the author of incident
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::integrate::models::UpdateIntegrationData>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// UpdateOpsGenieIntegrationData : Integration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOpsGenieIntegrationData {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// UpdatePagerDutyIntegrationData : Create integration for the PagerDuty API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePagerDutyIntegrationData {
    /// API token for PagerDuty
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// The name of the service for the integration
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The name for the author of incident
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// UpdateSlackIntegrationData : Integration for a Slack bot used by this integration.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSlackIntegrationData {
    /// Slack Channel to send messages to
    #[serde(rename = "channelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// API token for the Slack bot
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
