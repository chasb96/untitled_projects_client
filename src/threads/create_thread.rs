use prost::Message;

#[derive(Message)]
pub struct CreateThreadRequest {
    #[prost(string, tag = "1")]
    pub title: String,
}

#[derive(Message)]
pub struct CreateThreadResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}