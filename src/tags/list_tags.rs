use prost::Message;

#[derive(Message)]
pub struct ListTagsResponse {
    #[prost(string, repeated, tag = "1")]
    pub tags: Vec<String>,
}