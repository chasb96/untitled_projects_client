use std::collections::HashSet;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum GetByIdResponse {
    #[serde(rename = "n")]
    New(New),
    #[serde(rename = "a")]
    Approved(Approved),
    #[serde(rename = "c")]
    Completed(Completed),
}

#[derive(Deserialize)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Deserialize)]
pub struct New {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Deserialize)]
pub struct Approved {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "a")]
    pub approvers: HashSet<String>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Deserialize)]
pub struct Completed {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "a")]
    pub approvers: HashSet<String>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}
