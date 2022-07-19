use super::formats::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

/// Integration for AWS CloudWatch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsCloudWatchLogsIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id_display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
    pub group_name: String,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key_display: Option<String>,
    pub source: String,
}

/// Integration for AWS CloudWatch metrics
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsCloudWatchMetricsIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id_display: Option<String>,
    pub cluster_ids: Vec<String>,
    pub namespace: String,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key_display: Option<String>,
    pub source: String,
}

/// Create AWS CloudWatch logs integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAwsCloudWatchLogsIntegrationData {
    pub access_key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
    pub group_name: String,
    pub region: String,
    pub secret_access_key: String,
    pub source: String,
}

/// Create AWS CloudWatch metrics integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAwsCloudWatchMetricsIntegrationData {
    pub access_key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
    pub namespace: String,
    pub region: String,
    pub secret_access_key: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum CreateIntegrationData {
    #[serde(rename = "awsCloudWatchLogs")]
    AwsCloudWatchLogs(CreateAwsCloudWatchLogsIntegrationData),
    #[serde(rename = "awsCloudWatchMetrics")]
    AwsCloudWatchMetrics(CreateAwsCloudWatchMetricsIntegrationData),
    #[serde(rename = "opsGenie")]
    OpsGenie(CreateOpsGenieIntegrationData),
    #[serde(rename = "slack")]
    Slack(CreateSlackIntegrationData),
    #[serde(rename = "pagerDuty")]
    PagerDuty(CreatePagerDutyIntegrationData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIntegrationRequest {
    pub data: CreateIntegrationData,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIntegrationResponse {
    pub id: IntegrationId,
}

/// Create integration for the Ops Genie API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOpsGenieIntegrationData {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Create integration for the PagerDuty API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePagerDutyIntegrationData {
    pub auth_token: String,
    pub service: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Create integration for a Slack bot used by this integration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSlackIntegrationData {
    pub channel_id: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIntegrationResponse {
    pub integration: Integration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    pub created: DateTime<Utc>,
    pub data: IntegrationData,
    pub description: String,
    pub id: IntegrationId,
    pub organization_id: OrganizationId,
    pub project_id: ProjectId,
    pub status: IntegrationStatus,
    pub updated: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sink")]
pub enum IntegrationData {
    #[serde(rename = "awsCloudWatchLogs")]
    AwsCloudWatchLogs(AwsCloudWatchLogsIntegrationData),
    #[serde(rename = "awsCloudWatchMetrics")]
    AwsCloudWatchMetrics(AwsCloudWatchMetricsIntegrationData),
    #[serde(rename = "opsGenie")]
    OpsGenie(OpsGenieIntegrationData),
    #[serde(rename = "slack")]
    Slack(SlackIntegrationData),
    #[serde(rename = "pagerDuty")]
    PagerDuty(PagerDutyIntegrationData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationsOptionsResponse {
    pub sources: Vec<SourceSummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IntegrationStatus {
    Active,
    Deleted,
}
impl std::fmt::Display for IntegrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationStatus::Active => write!(f, "active"),
            IntegrationStatus::Deleted => write!(f, "deleted"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIntegrationsResponse {
    pub integrations: Vec<Integration>,
}

/// Integration for the Ops Genie API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpsGenieIntegrationData {
    pub api_key_display: String,
    pub source: String,
}

/// Integration for the PagerDuty API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagerDutyIntegrationData {
    pub auth_token_display: String,
    pub service: String,
    pub user: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SinkSummary {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub supports_test_api: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Integration for a Slack bot used by this integration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlackIntegrationData {
    pub channel_id: String,
    pub token_display: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSummary {
    pub enabled: bool,
    pub id: String,
    pub name: String,
    pub sinks: Vec<SinkSummary>,
}

/// Update request data for AWS CloudWatch logs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAwsCloudWatchLogsIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

/// Update request data for AWS CloudWatch metrics
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAwsCloudWatchMetricsIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIntegrationData {
    UpdateAwsCloudWatchLogsIntegrationData(UpdateAwsCloudWatchLogsIntegrationData),
    UpdateAwsCloudWatchMetricsIntegrationData(UpdateAwsCloudWatchMetricsIntegrationData),
    UpdateOpsGenieIntegrationData(UpdateOpsGenieIntegrationData),
    UpdateSlackIntegrationData(UpdateSlackIntegrationData),
    UpdatePagerDutyIntegrationData(UpdatePagerDutyIntegrationData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIntegrationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateIntegrationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Integration for the Ops Genie API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOpsGenieIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// Create integration for the PagerDuty API integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePagerDutyIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Integration for a Slack bot used by this integration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSlackIntegrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
