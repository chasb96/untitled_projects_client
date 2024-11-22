use prost::Message;

#[derive(Message)]
pub struct ListCommentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub comments: Vec<CommentResponse>,
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub content: String,
}