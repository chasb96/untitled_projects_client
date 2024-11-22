pub mod create_source_request;
pub mod approve_sourcce_request;
pub mod complete_source_request;
pub mod list_source_requests;
pub mod get_source_request;
pub mod comments;

use crate::ProjectsClient;
use create_source_request::CreateSourceRequestRequest;
use create_source_request::CreateSourceRequestResponse;
use crate::Error;
use prost::Message;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use approve_sourcce_request::ApproveSourceRequestRequest;
use complete_source_request::CompleteSourceRequestRequest;
use get_source_request::SourceRequest;
use list_source_requests::ListSourceRequestsResponse;
use std::future::Future;

pub trait SourceRequestsClient {
    fn get_source_request(&self, project_id: &str, source_request_id: &str) -> impl Future<Output = Result<SourceRequest, Error>> + Send;

    fn list_source_requests(&self, project_id: &str) -> impl Future<Output = Result<ListSourceRequestsResponse, Error>> + Send;

    fn create_source_request(&self, project_id: &str, request: CreateSourceRequestRequest) -> impl Future<Output = Result<CreateSourceRequestResponse, Error>> + Send;

    fn approve_source_request(&self, project_id: &str, source_request_id: &str, request: ApproveSourceRequestRequest) -> impl Future<Output = Result<(), Error>> + Send;

    fn complete_source_request(&self, project_id: &str, source_request_id: &str, request: CompleteSourceRequestRequest) -> impl Future<Output = Result<(), Error>> + Send;
}


impl SourceRequestsClient for ProjectsClient {
    async fn get_source_request(&self, project_id: &str, source_request_id: &str) -> Result<SourceRequest, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests/{}", self.base_url, project_id, source_request_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(serde_json::from_slice(&response)?)
    }

    async fn list_source_requests(&self, project_id: &str) -> Result<ListSourceRequestsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/source_requests", self.base_url, project_id))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(serde_json::from_slice(&response)?)
    }

    async fn create_source_request(&self, project_id: &str, request: CreateSourceRequestRequest) -> Result<CreateSourceRequestResponse, Error> {
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

        Ok(CreateSourceRequestResponse::decode(response)?)
    }
    
    async fn approve_source_request(&self, project_id: &str, source_request_id: &str, request: ApproveSourceRequestRequest) -> Result<(), Error> {
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
    
    async fn complete_source_request(&self, project_id: &str, source_request_id: &str, request: CompleteSourceRequestRequest) -> Result<(), Error> {
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
}