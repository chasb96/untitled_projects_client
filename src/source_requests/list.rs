use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListResponse {
    #[serde(rename = "sr")]
    pub source_requests: Vec<SourceRequest>,
}

#[derive(Deserialize)]
pub struct SourceRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sr")]
    pub source_request: Summary,
}

#[derive(Deserialize)]
pub enum Summary {
    #[serde(rename = "n")]
    New(New),
    #[serde(rename = "a")]
    Approved(Approved),
    #[serde(rename = "c")]
    Completed(Completed),
}

#[derive(Deserialize)]
pub struct New {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}
#[derive(Deserialize)]
pub struct Approved {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

#[derive(Deserialize)]
pub struct Completed {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}