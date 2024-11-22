use prost::Message;

#[derive(Message)]
pub struct ListCommentsResponse {
    #[prost(message, repeated, tag = "1")]
    comments: Vec<CommentResponse>,
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    id: String,
    #[prost(string, tag = "2")]
    user_id: String,
    #[prost(string, tag = "3")]
    content: String,
}