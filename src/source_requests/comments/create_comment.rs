use prost::Message;

#[derive(Message)]
pub struct CreateCommentRequest {
    #[prost(string, tag = "1")]
    pub content: String,
}

#[derive(Message)]
pub struct CreateCommentResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}