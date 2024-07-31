use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct AddFilesRequest {
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
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

#[derive(Serialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}