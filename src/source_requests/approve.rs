use prost::Message;

#[derive(Message)]
pub struct ApproveRequest {
    #[prost(string, tag = 1)]
    pub user_id: String,
}