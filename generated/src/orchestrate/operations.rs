use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// creates a new job
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `create_job_request`
pub async fn create_job(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // defines the job to be created
    create_job_request: CreateJobRequest,
) -> Result<CreateJobResponse> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/jobs",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateJobRequest, CreateJobResponse>(
            Method::POST,
            url,
            Some(&create_job_request),
            None,
        )
        .await
}

/// delete a job
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `job_id` - the id of the job
pub async fn delete_job(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    job_id: JobId,
) -> Result<()> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/jobs/{jobId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        jobId = urlencode(job_id),
    );
    client
        .send_request::<(), ()>(Method::DELETE, url, None, Some(()))
        .await
}

/// gets a job
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `job_id` - The id of the job
pub async fn get_job(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    job_id: JobId,
) -> Result<GetJobResponse> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/jobs/{jobId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        jobId = urlencode(job_id),
    );
    client
        .send_request::<(), GetJobResponse>(Method::GET, url, None, None)
        .await
}

/// shows orchestration history
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `job_id` - Filter by job id
pub async fn list_history(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    job_id: Option<JobId>,
) -> Result<GetHistoryResponse> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/history",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    let url = match job_id {
        Some(v) => format!("{url}?jobId={jobId}", jobId = urlencode(v),),
        None => url,
    };
    client
        .send_request::<(), GetHistoryResponse>(Method::GET, url, None, None)
        .await
}

/// returns the list of jobs under a project
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
pub async fn list_jobs(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListJobsResponse> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/jobs",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListJobsResponse>(Method::GET, url, None, None)
        .await
}

/// runs a job immediately
/// # Arguments
///
/// * `organization_id` - The id of the organization
/// * `project_id` - The id of the project
/// * `job_id` - The id of the job
pub async fn run_job(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    job_id: JobId,
) -> Result<RunJobResponse> {
    let url = format!(
        "/orchestrate/v1/organizations/{organizationId}/projects/{projectId}/jobs/{jobId}/run",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        jobId = urlencode(job_id),
    );
    client
        .send_request::<(), RunJobResponse>(Method::POST, url, None, None)
        .await
}
