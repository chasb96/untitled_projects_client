use prost::Message;

#[derive(Message)]
pub struct ListThreadsResponse {
    #[prost(message, repeated, tag = "1")]
    threads: Vec<ThreadResponse>,
}

#[derive(Message)]
pub struct ThreadResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub title: String,
}