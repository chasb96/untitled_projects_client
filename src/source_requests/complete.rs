use prost::Message;

#[derive(Message)]
pub struct CompleteRequest {
    #[prost(string, tag = 1)]
    pub user_id: String,
}