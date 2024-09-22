pub mod list_threads;
pub mod create_thread;
pub mod get_thread_by_id;
pub mod create_comment;

use crate::ProjectsClient;
use crate::Error;
use prost::Message;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;

pub trait ProjectThreadsClient {
    async fn list_threads(&self, project_id: &str) -> Result<list_threads::ListThreadsResponse, Error>;

    async fn create_thread(&self, project_id: &str, request: create_thread::CreateThreadRequest) -> Result<(), Error>;

    async fn get_thread_by_id(&self, project_id: &str, thread_id: &str) -> Result<get_thread_by_id::ThreadResponse, Error>;

    async fn create_comment(&self, project_id: &str, thread_id: &str, request: create_comment::CreateCommentRequest) -> Result<create_comment::CreateCommentResponse, Error>;
}

impl ProjectThreadsClient for ProjectsClient {
    async fn list_threads(&self, project_id: &str) -> Result<list_threads::ListThreadsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/threads", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(list_threads::ListThreadsResponse::decode(response)?)
    }

    async fn create_thread(&self, project_id: &str, request: create_thread::CreateThreadRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/projects/{}/threads", self.base_url, project_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn get_thread_by_id(&self, project_id: &str, thread_id: &str) -> Result<get_thread_by_id::ThreadResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/threads/{}", self.base_url, project_id, thread_id))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(get_thread_by_id::ThreadResponse::decode(response)?)
    }
    
    async fn create_comment(&self, project_id: &str, thread_id: &str, request: create_comment::CreateCommentRequest) -> Result<create_comment::CreateCommentResponse, Error> {
        let response = self.http_client
            .post(format!("{}/projects/{}/threads/{}/comments", self.base_url, project_id, thread_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(ACCEPT, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(create_comment::CreateCommentResponse::decode(response)?)
    }
}