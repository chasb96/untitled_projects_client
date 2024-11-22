pub mod create_tag;
pub mod list_tags;

use crate::Error;
use crate::ProjectsClient;
use reqwest::header::CONTENT_TYPE;
use prost::Message;
use reqwest::header::ACCEPT;
use std::future::Future;

pub trait TagsClient {
    fn create_tag(&self, project_id: &str, request: create_tag::CreateTagRequest) -> impl Future<Output = Result<(), Error>> + Send;
    
    fn list_tags(&self, project_id: &str) -> impl Future<Output = Result<list_tags::ListTagsResponse, Error>> + Send;

    fn delete_tag(&self, project_id: &str, tag: &str) -> impl Future<Output = Result<(), Error>> + Send;
}

impl TagsClient for ProjectsClient {
    async fn create_tag(&self, project_id: &str, request: create_tag::CreateTagRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/projects/{}/tags", self.base_url, project_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn list_tags(&self, project_id: &str) -> Result<list_tags::ListTagsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/tags", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(list_tags::ListTagsResponse::decode(response)?)
    }

    async fn delete_tag(&self, project_id: &str, tag: &str) -> Result<(), Error> {
        self.http_client
            .delete(format!("{}/projects/{}/tags/{}", self.base_url, project_id, tag))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

}