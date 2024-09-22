use serde::Serialize;

use crate::events::*;

#[derive(Serialize)]
pub enum EventRequest {
    #[serde(rename = "a")]
    AddFiles(AddFilesRequest),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesRequest),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesRequest),
}