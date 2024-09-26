use serde::Deserialize;

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
pub enum SourceRequestSummary {
    #[serde(rename = "n")]
    New(NewSourceRequestSummary),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequestSummary),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequestSummary),
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
pub struct ApprovedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
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