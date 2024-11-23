use prost::Message;
use std::collections::HashMap;

#[derive(Message)]
pub struct SourceRequestDiffResponse {
    #[prost(map = "string, message", tag = "1")]
    pub diff_items: HashMap<String, SourceRequestDiffItem>
}

#[derive(PartialEq, Message)]
pub struct SourceRequestDiffItem {
    #[prost(string, tag = "1")]
    pub from: String,
    #[prost(string, tag = "2")]
    pub to: String,
}