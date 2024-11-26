use prost::Message;

#[derive(Message)]
pub struct CreateRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub content: String,
}

#[derive(Message)]
pub struct CreateResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}