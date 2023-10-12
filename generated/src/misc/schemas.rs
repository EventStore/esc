use super::formats::*;
use crate::resources::formats::OrganizationId;
use crate::resources::formats::ProjectId;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteResponse {
    pub id: NoteId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNoteResponse {
    pub id: NoteId,
}

pub type Fields = HashMap<String, String>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNoteResponse {
    pub id: NoteId,
    pub created: DateTime<Utc>,
    pub organization_id: OrganizationId,
    pub project_id: ProjectId,
    pub text: String,
    pub updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNotesResponse {
    pub notes: Vec<GetNoteResponse>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest {
    pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteResponse {
    pub id: NoteId,
}
