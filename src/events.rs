use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct AddFilesRequest {
    #[serde(rename = "peid")]
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
    #[serde(rename = "peid")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

#[derive(Serialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "peid")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}