#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobRequest {
    #[serde(flatten)]
    pub data: crate::orchestrate::models::JobData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "schedule")]
    pub schedule: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobResponse {
    #[serde(rename = "id")]
    pub id: crate::types::JobId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetHistoryResponse {
    #[serde(rename = "items")]
    pub items: Vec<crate::orchestrate::models::HistoryItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetJobResponse {
    #[serde(rename = "job")]
    pub job: crate::orchestrate::models::Job,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryItem {
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "jobId")]
    pub job_id: crate::types::JobId,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "linkedResource")]
    pub linked_resource: String,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "startTime")]
    pub start_time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub data: crate::orchestrate::models::JobData,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: crate::types::JobId,
    #[serde(rename = "organizationId")]
    pub organization_id: crate::types::OrgId,
    #[serde(rename = "projectId")]
    pub project_id: crate::types::ProjectId,
    #[serde(rename = "schedule")]
    pub schedule: String,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobData {
    ScheduledBackup(ScheduledBackupData),
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobType {
    #[serde(rename = "ScheduledBackup")]
    ScheduledBackup,
}

impl JobType {
    pub fn from_str(src: &str) -> Result<JobType, String> {
        match src {
            "ScheduledBackup" => Ok(JobType::ScheduledBackup),
            _ => Err(format!(
                "Unsupported value \"{}\". Supported values: {:?}",
                src,
                ["ScheduledBackup",]
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListJobsResponse {
    #[serde(rename = "jobs")]
    pub jobs: Vec<crate::orchestrate::models::Job>,
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
pub struct ScheduledBackupData {
    #[serde(rename = "clusterId")]
    pub cluster_id: crate::types::ClusterId,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "maxBackupCount")]
    pub max_backup_count: i32,
}
