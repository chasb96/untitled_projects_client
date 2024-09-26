use prost::Message;

#[derive(Message)]
pub struct ApproveSourceRequestRequest {
    #[prost(string, tag = 1)]
    pub user_id: String,
}