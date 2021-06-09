use crate::http::{authenticated_request, default_error_handler};
use crate::{Client, Job, JobData, JobId, OrgId, ProjectId, Token};
use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobParams {
    pub description: String,
    pub schedule: String,
    #[serde(flatten)]
    pub data: JobData,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateJobResponse {
    id: JobId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetJobResponse {
    job: Job,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListJobsResponse {
    jobs: Vec<Job>,
}

pub struct Jobs<'a> {
    client: &'a Client,
    token: &'a Token,
}

impl<'a> Jobs<'a> {
    pub fn new(client: &'a Client, token: &'a Token) -> Self {
        Jobs { client, token }
    }

    pub async fn create(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        params: CreateJobParams,
    ) -> crate::Result<JobId> {
        let req = authenticated_request(
            &self.client,
            Method::POST,
            self.token,
            format!(
                "{}/orchestrate/v1/organizations/{}/projects/{}/jobs",
                self.client.base_url, org_id, project_id
            ),
        )
        .json(&params);

        let resp = default_error_handler(req.send().await?).await?;
        let result: CreateJobResponse = resp.json().await?;
        Ok(result.id)
    }

    pub async fn get(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        job_id: JobId,
    ) -> crate::Result<Job> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/orchestrate/v1/organizations/{}/projects/{}/jobs/{}",
                self.client.base_url, org_id, project_id, job_id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;
        // works around serde_json issue 505
        let data: serde_json::Value = resp.json().await?;
        let result: GetJobResponse = serde_json::from_value(data)?;
        Ok(result.job)
    }

    pub async fn delete(
        &self,
        org_id: OrgId,
        project_id: ProjectId,
        id: JobId,
    ) -> crate::Result<()> {
        let req = authenticated_request(
            &self.client,
            Method::DELETE,
            self.token,
            format!(
                "{}/orchestrate/v1/organizations/{}/projects/{}/jobs/{}",
                self.client.base_url, org_id, project_id, id
            ),
        );

        let _ = default_error_handler(req.send().await?).await?;

        Ok(())
    }

    pub async fn list(&self, org_id: OrgId, project_id: ProjectId) -> crate::Result<Vec<Job>> {
        let req = authenticated_request(
            &self.client,
            Method::GET,
            self.token,
            format!(
                "{}/orchestrate/v1/organizations/{}/projects/{}/jobs",
                self.client.base_url, org_id, project_id
            ),
        )
        .header("Content-Type", "application/json");

        let resp = default_error_handler(req.send().await?).await?;

        let result: ListJobsResponse = resp.json().await?;

        Ok(result.jobs)
    }
}

#[test]
fn test_job_data_serialization() {
    use crate::{ClusterId, JobDataScheduledBackup, ScheduleField, ScheduleInfo};
    use serde_json;

    let d = Job {
        id: JobId("jobid".to_string()),
        org_id: OrgId("orgId".to_string()),
        project_id: ProjectId("projectId".to_string()),
        description: "desc".to_string(),
        schedule: "cron".to_string(),
        schedule_info: Some(ScheduleInfo {
            rate_in_minutes: 30,
            minute: ScheduleField {
                expr_type: "rate".to_string(),
                number: 38,
            },
            hour: ScheduleField {
                expr_type: "wildcard".to_string(),
                number: 0,
            },
            day_of_week: ScheduleField {
                expr_type: "number".to_string(),
                number: 1,
            },
        }),
        data: JobData::ScheduledBackup(JobDataScheduledBackup {
            description: "{some-expr-here}".to_string(),
            max_backup_count: 3,
            cluster_id: ClusterId("source cluster ID".to_string()),
        }),
        status: "running".to_string(),
    };

    let expected: serde_json::Value = serde_json::from_str(
        r#"{
  "id": "jobid",
  "organizationId": "orgId",
  "projectId": "projectId",
  "description": "desc",
  "schedule": "cron",
  "scheduleInfo": {
    "rateInMinutes": 30,
    "minute": {
        "exprType": "rate",
        "number": 38
    },
    "hour": {        
        "exprType": "wildcard",
        "number": 0
    },
    "dayOfWeek": {        
        "exprType": "number",
        "number": 1
    }
  },
  "type": "ScheduledBackup",
  "data": {
    "description": "{some-expr-here}",
    "maxBackupCount": 3,
    "clusterId": "source cluster ID"
  },
  "status": "running"
}"#,
    )
    .unwrap();

    let actual = serde_json::to_value(&d).unwrap();
    // let bytes = serde_json::to_vec_pretty(&d).unwrap();
    // let actual = String::from_utf8_lossy(&bytes);
    println!("{}", expected);
    println!("{}", actual);
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn test_job_data_deserialization() {
    use serde_json;

    let input = r#"{
  "id": "jobid",
  "organizationId": "orgId",
  "projectId": "projectId",
  "description": "desc",
  "schedule": "cron",
  "data": {
    "description": "{some-expr-here}",
    "maxBackupCount": 3,
    "clusterId": "source cluster ID"
  },
  "type": "ScheduledBackup",
  "status": "running"
}"#;

    let actual: Job = serde_json::from_str(input).unwrap();

    assert_eq!(actual.id, JobId("jobid".to_string()));
    assert_eq!(actual.org_id, OrgId("orgId".to_string()));
    assert_eq!(actual.project_id, ProjectId("projectId".to_string()));
    assert_eq!(actual.description, "desc".to_string());
    assert_eq!(actual.schedule, "cron".to_string());
    assert_eq!(actual.status, "running".to_string());
    match actual.data {
        JobData::ScheduledBackup(data) => {
            assert_eq!(data.description, "{some-expr-here}".to_string());
            assert_eq!(data.max_backup_count, 3);
            assert_eq!(data.cluster_id.0, "source cluster ID".to_string());
        }
    }
}

#[test]
fn test_job_data_deserialization_with_other_type() {
    use serde_json;

    let input = r#"{
  "id": "jobid",
  "organizationId": "orgId",
  "projectId": "projectId",
  "description": "desc",
  "schedule": "cron",
  "data": {
    "description": "dark roast",
    "tablespoons": 5
  },
  "type": "coffeepot",
  "status": "running"
}"#;

    let actual = serde_json::from_str::<Job>(input);
    assert!(actual.is_err());
}
