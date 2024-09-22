use prost::Message;

#[derive(Message)]
pub struct CreateTagRequest {
    #[prost(string, tag = "1")]
    pub tag: String,
}