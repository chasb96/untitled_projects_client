pub mod create;
pub mod approve;
pub mod complete;
pub mod list;
pub mod get_by_id;
pub mod comments;
pub mod diff;

use crate::ProjectsClient;
use crate::Error;
use prost::Message;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use std::future::Future;

pub trait SourceRequestsClient {
    fn get_source_request(&self, project_id: &str, source_request_id: &str) -> impl Future<Output = Result<get_by_id::GetByIdResponse, Error>> + Send;

    fn list_source_requests(&self, project_id: &str) -> impl Future<Output = Result<list::ListResponse, Error>> + Send;

    fn create_source_request(&self, project_id: &str, request: create::CreateRequest) -> impl Future<Output = Result<create::CreateResponse, Error>> + Send;

    fn approve_source_request(&self, project_id: &str, source_request_id: &str, request: approve::ApproveRequest) -> impl Future<Output = Result<(), Error>> + Send;

    fn complete_source_request(&self, project_id: &str, source_request_id: &str, request: complete::CompleteRequest) -> impl Future<Output = Result<(), Error>> + Send;

    fn get_source_request_diff(&self, project_id: &str, source_request_id: &str) -> impl Future<Output = Result<diff::DiffResponse, Error>> + Send;
}


impl SourceRequestsClient for ProjectsClient {
    async fn get_source_request(&self, project_id: &str, source_request_id: &str) -> Result<get_by_id::GetByIdResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests/{}", self.base_url, project_id, source_request_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(serde_json::from_slice(&response)?)
    }

    async fn list_source_requests(&self, project_id: &str) -> Result<list::ListResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests", self.base_url, project_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(serde_json::from_slice(&response)?)
    }

    async fn create_source_request(&self, project_id: &str, request: create::CreateRequest) -> Result<create::CreateResponse, Error> {
        let response = self.http_client
            .post(format!("{}/projects/{}/source_requests", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(create::CreateResponse::decode(response)?)
    }
    
    async fn approve_source_request(&self, project_id: &str, source_request_id: &str, request: approve::ApproveRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/projects/{}/source_requests/{}/approve", self.base_url, project_id, source_request_id))
            .header(ACCEPT, "application/octet-stream")
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
    
    async fn complete_source_request(&self, project_id: &str, source_request_id: &str, request: complete::CompleteRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/projects/{}/source_requests/{}/complete", self.base_url, project_id, source_request_id))
            .header(ACCEPT, "application/octet-stream")
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn get_source_request_diff(&self, project_id: &str, source_request_id: &str) -> Result<diff::DiffResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests/{}/diff", self.base_url, project_id, source_request_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(diff::DiffResponse::decode(response)?)
    }
}