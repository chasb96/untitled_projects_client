use prost::Message;

#[derive(Message)]
pub struct CompleteSourceRequestRequest {
    #[prost(string, tag = 1)]
    pub user_id: String,
}