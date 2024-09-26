use prost::Message;

#[derive(Message)]
pub struct CreateSourceRequestRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub title: String,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(message, repeated, tag = "4")]
    pub files: Vec<FileMap>,
}

#[derive(Message)]
pub struct FileMap {
    #[prost(string, tag = "1")]
    pub path: String,
    #[prost(string, tag = "2")]
    pub file_id: String,
}

#[derive(Message)]
pub struct CreateSourceRequestResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}