use chrono::{DateTime, Utc};

use super::common::{List, ToV1};
use super::resources::OrgId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDataScheduledBackup {
    pub description: String,
    pub max_backup_count: i32,
    pub cluster_id: esc_api::mesdb::ClusterId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobData {
    ScheduledBackup(JobDataScheduledBackup),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleField {
    pub expr_type: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: esc_api::orchestrate::JobId,
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub project_id: esc_api::resources::ProjectId,
    pub description: String,
    pub schedule: String,
    pub status: String,
    #[serde(flatten)]
    pub data: JobData,
}

impl ToV1 for esc_api::orchestrate::Job {
    type V1Type = Job;
    fn to_v1(self) -> Self::V1Type {
        let data = match self.data {
            esc_api::orchestrate::JobData::ScheduledBackup(backup) => {
                JobData::ScheduledBackup(JobDataScheduledBackup {
                    cluster_id: backup.cluster_id,
                    description: backup.description,
                    max_backup_count: backup.max_backup_count,
                })
            }
        };
        Job {
            data,
            description: self.description,
            id: self.id,
            org_id: self.organization_id.to_v1(),
            project_id: self.project_id,
            schedule: self.schedule,
            status: self.status,
        }
    }
}

impl ToV1 for esc_api::orchestrate::CreateJobResponse {
    type V1Type = esc_api::orchestrate::JobId;
    fn to_v1(self) -> Self::V1Type {
        self.id
    }
}

impl ToV1 for esc_api::orchestrate::GetJobResponse {
    type V1Type = Job;
    fn to_v1(self) -> Self::V1Type {
        self.job.to_v1()
    }
}

impl ToV1 for esc_api::orchestrate::ListJobsResponse {
    type V1Type = List<Job>;
    fn to_v1(self) -> Self::V1Type {
        List(self.jobs.into_iter().map(|j| j.to_v1()).collect())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    #[serde(rename = "organizationId")]
    pub org_id: OrgId,
    pub project_id: esc_api::resources::ProjectId,
    pub job_id: esc_api::orchestrate::JobId,
    pub status: String,
    pub details: String,
    pub linked_resource: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl ToV1 for esc_api::orchestrate::HistoryItem {
    type V1Type = HistoryItem;
    fn to_v1(self) -> Self::V1Type {
        HistoryItem {
            details: self.details,
            end_time: self.end_time,
            job_id: self.job_id,
            linked_resource: self.linked_resource,
            org_id: self.organization_id.to_v1(),
            project_id: self.project_id,
            start_time: self.start_time,
            status: self.status,
        }
    }
}

impl ToV1 for esc_api::orchestrate::GetHistoryResponse {
    type V1Type = List<HistoryItem>;
    fn to_v1(self) -> Self::V1Type {
        List(self.items.into_iter().map(|i| i.to_v1()).collect())
    }
}
