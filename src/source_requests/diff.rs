use prost::Message;
use std::collections::HashMap;

#[derive(Message)]
pub struct DiffResponse {
    #[prost(map = "string, message", tag = "1")]
    pub diff_items: HashMap<String, DiffItem>
}

#[derive(PartialEq, Message)]
pub struct DiffItem {
    #[prost(string, tag = "1")]
    pub from: String,
    #[prost(string, tag = "2")]
    pub to: String,
}