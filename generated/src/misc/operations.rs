use super::formats::*;
use super::schemas::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use esc_client_base::urlencode;
use esc_client_base::Client;
use esc_client_base::Result;
use reqwest::Method;
/// Create a note
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `create_note_request`
pub async fn create_note(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    // describes a new note
    create_note_request: CreateNoteRequest,
) -> Result<CreateNoteResponse> {
    let url = format!(
        "/misc/v1/organizations/{organizationId}/projects/{projectId}/notes",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<CreateNoteRequest, CreateNoteResponse>(
            Method::POST,
            url,
            Some(&create_note_request),
            None,
        )
        .await
}

/// Delete a note
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `note_id` - The id of the note to be deleted
pub async fn delete_note(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    note_id: NoteId,
) -> Result<DeleteNoteResponse> {
    let url = format!(
        "/misc/v1/organizations/{organizationId}/projects/{projectId}/notes/{noteId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        noteId = urlencode(note_id),
    );
    client
        .send_request::<(), DeleteNoteResponse>(Method::DELETE, url, None, None)
        .await
}

/// Gets a note
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `note_id` - The id of the note
pub async fn get_note(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    note_id: NoteId,
) -> Result<GetNoteResponse> {
    let url = format!(
        "/misc/v1/organizations/{organizationId}/projects/{projectId}/notes/{noteId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        noteId = urlencode(note_id),
    );
    client
        .send_request::<(), GetNoteResponse>(Method::GET, url, None, None)
        .await
}

/// Lists all notes
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
pub async fn list_notes(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
) -> Result<ListNotesResponse> {
    let url = format!(
        "/misc/v1/organizations/{organizationId}/projects/{projectId}/notes",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
    );
    client
        .send_request::<(), ListNotesResponse>(Method::GET, url, None, None)
        .await
}

/// Update a note
///
/// # Arguments
///
/// * `organization_id` - The id of the organization the cluster is owned by
/// * `project_id` - The id of the project the cluster is organized by
/// * `note_id` - The id of the note to be updated
/// * `update_note_request`
pub async fn update_note(
    client: &Client,
    organization_id: OrganizationId,
    project_id: ProjectId,
    note_id: NoteId,
    // describes new note contents
    update_note_request: UpdateNoteRequest,
) -> Result<UpdateNoteResponse> {
    let url = format!(
        "/misc/v1/organizations/{organizationId}/projects/{projectId}/notes/{noteId}",
        organizationId = urlencode(organization_id),
        projectId = urlencode(project_id),
        noteId = urlencode(note_id),
    );
    client
        .send_request::<UpdateNoteRequest, UpdateNoteResponse>(
            Method::PUT,
            url,
            Some(&update_note_request),
            None,
        )
        .await
}
