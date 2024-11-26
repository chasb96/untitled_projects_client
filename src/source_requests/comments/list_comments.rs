use prost::Message;

#[derive(Message)]
pub struct CommentListResponse {
    #[prost(message, repeated, tag = "1")]
    pub comments: Vec<Comment>,
}

#[derive(Message)]
pub struct Comment {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub content: String,
}