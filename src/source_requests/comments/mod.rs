pub mod create_comment;
pub mod list_comments;

use std::future::Future;
use crate::Error;
use crate::ProjectsClient;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;
use prost::Message;

pub trait CommentsClient {
    fn create_comment(&self, project_id: &str, source_request_id: &str, request: create_comment::CreateRequest) -> impl Future<Output = Result<create_comment::CreateResponse, Error>> + Send;

    fn list_comments(&self, project_id: &str, source_request_id: &str) -> impl Future<Output = Result<list_comments::CommentListResponse, Error>> + Send;
}

impl CommentsClient for ProjectsClient {
    async fn create_comment(&self, project_id: &str, source_request_id: &str, request: create_comment::CreateRequest) -> Result<create_comment::CreateResponse, Error> {
        let response = self.http_client
            .post(format!("{}/projects/{}/source_requests/{}/comments", self.base_url, project_id, source_request_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(ACCEPT, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(create_comment::CreateResponse::decode(response)?)
    }

    async fn list_comments(&self, project_id: &str, source_request_id: &str) -> Result<list_comments::CommentListResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests/{}/comments", self.base_url, project_id, source_request_id))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(list_comments::CommentListResponse::decode(response)?)
    }
}