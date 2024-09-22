use serde::Serialize;

use std::collections::HashMap;

#[derive(Serialize)]
pub enum EventRequest {
    #[serde(rename = "a")]
    AddFiles(AddFilesRequest),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesRequest),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesRequest),
}

#[derive(Serialize)]
pub struct AddFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<AddFileRequest>,
}

#[derive(Serialize)]
pub struct AddFileRequest {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Serialize)]
pub struct RemoveFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

#[derive(Serialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}