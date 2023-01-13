use super::formats::*;
use crate::mesdb::formats::ClusterId;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobRequest {
    #[serde(flatten)]
    pub data: JobData,
    pub description: String,
    pub schedule: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobResponse {
    pub id: JobId,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryResponse {
    pub items: Vec<HistoryItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobResponse {
    pub job: Job,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub details: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    pub job_id: JobId,
    pub id: String,
    pub linked_resource: String,
    pub organization_id: OrganizationId,
    pub project_id: ProjectId,
    pub start_time: DateTime<Utc>,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    #[serde(flatten)]
    pub data: JobData,
    pub description: String,
    pub id: JobId,
    pub organization_id: OrganizationId,
    pub project_id: ProjectId,
    pub schedule: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobData {
    ScheduledBackup(ScheduledBackupData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobType {
    ScheduledBackup,
}
impl std::fmt::Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::ScheduledBackup => write!(f, "ScheduledBackup"),
        }
    }
}
impl std::cmp::PartialEq<&str> for JobType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            JobType::ScheduledBackup => *other == "ScheduledBackup",
        }
    }
}
impl std::cmp::PartialEq<JobType> for &str {
    fn eq(&self, other: &JobType) -> bool {
        other == self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListJobsResponse {
    pub jobs: Vec<Job>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunJobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledBackupData {
    pub cluster_id: ClusterId,
    pub description: String,
    pub max_backup_count: i32,
}
