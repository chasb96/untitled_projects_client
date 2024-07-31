use prost::Message;

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
}

#[derive(Message)]
pub struct CreateTagRequest {
    #[prost(string, tag = "1")]
    pub tag: String,
}