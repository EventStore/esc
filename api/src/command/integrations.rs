use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, Token};
use reqwest::Method;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegration {
    #[serde(rename = "data")]
    pub data: IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum CreateIntegrationData {
    #[serde(rename = "opsGenie")]
    CreateOpsGenieIntegrationData {
        /// API key used with the Ops Genie integration API
        #[serde(rename = "apiKey")]
        api_key: String,
    },
    #[serde(rename = "slack")]
    CreateSlackIntegrationData {
        /// Slack Channel to send messages to
        #[serde(rename = "channelId")]
        channel_id: String,
        /// API token for the Slack bot
        #[serde(rename = "token")]
        token: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIntegrationRequest {
    #[serde(rename = "data")]
    pub data: CreateIntegrationData,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIntegrationResponse {
    #[serde(rename = "integration")]
    pub integration: Integration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "data")]
    pub data: IntegrationData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: crate::types::IntegrationId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "status")]
    pub status: IntegrationStatus,
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
    pub integrations: Vec<Integration>,
}

/// OpsGenieIntegration : Integration for the Ops Genie API integration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsGenieIntegration {
    /// API key used with the Ops Genie integration API
    #[serde(rename = "api_key")]
    pub api_key: String,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateIntegrationData>,
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

pub struct Integrations<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Integrations<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Integrations { client, token }
    }

    pub async fn create(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        create_integration_request: CreateIntegrationRequest,
    ) -> crate::Result<CreateIntegrationResponse> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations",
                self.client.base_url,
                organizationId = organization_id,
                projectId = project_id,
            ),
        )
        .json(&create_integration_request);

        let resp = default_error_handler(req.send().await?).await?;
        let result: CreateIntegrationResponse = resp.json().await?;
        Ok(result)
    }

    pub async fn delete(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        integration_id: crate::types::IntegrationId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}", 
                self.client.base_url,
                organizationId=organization_id, projectId=project_id, integrationId=integration_id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn get(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        integration_id: crate::types::IntegrationId,
    ) -> crate::Result<GetIntegrationResponse> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!("{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}", self.client.base_url, organizationId=organization_id, projectId=project_id, integrationId=integration_id),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;
        let result: GetIntegrationResponse = resp.json().await?;
        Ok(result)
    }

    pub async fn list(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
    ) -> crate::Result<ListIntegrationsResponse> {
        let url = format!(
            "{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations",
            self.client.base_url,
            organizationId = organization_id,
            projectId = project_id,
        );

        let req = authenticated_request(&self.client, Method::GET, self.token, url)
            .header("Accept", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListIntegrationsResponse = resp.json().await?;

        Ok(result)
    }

    pub async fn test(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        integration_id: crate::types::IntegrationId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!("{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}/test", self.client.base_url, organizationId=organization_id, projectId=project_id, integrationId=integration_id),
        );
        let _ = default_error_handler(req.send().await?).await?;
        Ok(())
    }

    pub async fn update(
        &self,
        organization_id: crate::types::OrgId,
        project_id: crate::types::ProjectId,
        integration_id: crate::types::IntegrationId,
        update_integration_request: UpdateIntegrationRequest,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::PUT,
            self.token,
            format!("{}/integrate/v1/organizations/{organizationId}/projects/{projectId}/integrations/{integrationId}", self.client.base_url, organizationId=organization_id, projectId=project_id, integrationId=integration_id),
        )
        .header("Content-Type", "application/json")
        .json(&update_integration_request);

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }
}
