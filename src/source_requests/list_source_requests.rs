use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct ListSourceRequestsResponse {
    #[serde(rename = "sr")]
    pub source_requests: Vec<ListSourceRequestItem>,
}

#[derive(Deserialize)]
pub struct ListSourceRequestItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sr")]
    pub source_request: SourceRequestSummary,
}

#[derive(Deserialize)]
pub enum SourceRequest {
    #[serde(rename = "n")]
    New(NewSourceRequest),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequest),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequest),
}

#[derive(Deserialize)]
pub enum SourceRequestSummary {
    #[serde(rename = "n")]
    New(NewSourceRequestSummary),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequestSummary),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequestSummary),
}

#[derive(Deserialize)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Deserialize)]
pub struct NewSourceRequest {
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
pub struct NewSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

#[derive(Deserialize)]
pub struct ApprovedSourceRequest {
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
pub struct ApprovedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

#[derive(Deserialize)]
pub struct CompletedSourceRequest {
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
pub struct CompletedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}