#![allow(clippy::enum_variant_names)]

use super::common::ToV1;
use chrono::{DateTime, Utc};

use super::resources::OrgId;

// #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
// pub enum IntegrationStatus {
//     #[serde(rename = "active")]
//     Active,
//     #[serde(rename = "deleted")]
//     Deleted,
// }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "not supported: switch to CLI option `--fmt api` to see this data")]
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    #[serde(rename = "created")]
    pub created: DateTime<Utc>,
    #[serde(rename = "data")]
    pub data: IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: esc_api::integrate::IntegrationId,
    #[serde(rename = "organizationId")]
    pub organization_id: OrgId,
    #[serde(rename = "projectId")]
    pub project_id: esc_api::resources::ProjectId,
    #[serde(rename = "status")]
    pub status: esc_api::integrate::IntegrationStatus,
    #[serde(rename = "updated")]
    pub updated: DateTime<Utc>,
}

impl ToV1 for esc_api::integrate::IntegrationData {
    type V1Type = IntegrationData;
    fn to_v1(self) -> Self::V1Type {
        match self {
            esc_api::integrate::IntegrationData::OpsGenie(data) => {
                IntegrationData::OpsGenieIntegrationData {
                    api_key_display: data.api_key_display,
                    source: data.source,
                }
            }
            esc_api::integrate::IntegrationData::Slack(data) => {
                IntegrationData::SlackIntegrationData {
                    channel_id: data.channel_id,
                    source: data.source,
                    token_display: data.token_display,
                }
            }
            _ => IntegrationData::Unsupported,
        }
    }
}

impl ToV1 for esc_api::integrate::Integration {
    type V1Type = Integration;
    fn to_v1(self) -> Self::V1Type {
        Integration {
            created: self.created,
            data: self.data.to_v1(),
            description: self.description,
            id: self.id,
            organization_id: self.organization_id.to_v1(),
            project_id: self.project_id,
            status: self.status,
            updated: self.updated,
        }
    }
}

impl ToV1 for esc_api::integrate::CreateIntegrationResponse {
    type V1Type = esc_api::integrate::CreateIntegrationResponse;
    fn to_v1(self) -> Self::V1Type {
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIntegrationResponse {
    #[serde(rename = "integration")]
    pub integration: Integration,
}

impl ToV1 for esc_api::integrate::GetIntegrationResponse {
    type V1Type = GetIntegrationResponse;
    fn to_v1(self) -> Self::V1Type {
        GetIntegrationResponse {
            integration: self.integration.to_v1(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIntegrationsResponse {
    #[serde(rename = "integrations")]
    pub integrations: Vec<Integration>,
}

impl ToV1 for esc_api::integrate::ListIntegrationsResponse {
    type V1Type = ListIntegrationsResponse;
    fn to_v1(self) -> Self::V1Type {
        ListIntegrationsResponse {
            integrations: self.integrations.into_iter().map(|i| i.to_v1()).collect(),
        }
    }
}
