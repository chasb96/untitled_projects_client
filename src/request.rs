use prost::Message;
use serde::Serialize;

use crate::events::*;

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
}

#[derive(Serialize)]
pub enum EventRequest {
    #[serde(rename = "a")]
    AddFiles(AddFilesRequest),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesRequest),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesRequest),
}

#[derive(Message)]
pub struct CreateTagRequest {
    #[prost(string, tag = "1")]
    pub tag: String,
}
