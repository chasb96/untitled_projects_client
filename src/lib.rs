mod request;
mod query;
mod response;
mod error;
pub mod axum;

use std::env;

use prost::Message;

pub use request::*;
pub use query::*;
pub use response::*;
pub use error::Error;

use reqwest::{header::{ACCEPT, CONTENT_TYPE}, Client};

pub struct ProjectsClient {
    http_client: Client,
    base_url: String,
}

impl ProjectsClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<CreateProjectResponse, Error> {
        let response = self.http_client
            .post(format!("{}/projects", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(ACCEPT, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(CreateProjectResponse::decode(response)?)
    }

    pub async fn list_projects(&self, query: ListProjectsQuery) -> Result<ListProjectsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects?{}", self.base_url, query.to_query_string()))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(ListProjectsResponse::decode(response)?)
    }

    pub async fn get_project_by_id(&self, project_id: &str) -> Result<ProjectResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .header(CONTENT_TYPE, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(ProjectResponse::decode(response)?)
    }

    pub async fn create_tag(&self, project_id: &str, request: CreateTagRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/projects/{}/tags", self.base_url, project_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn list_tags(&self, project_id: &str) -> Result<ListTagsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}/tags", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(ListTagsResponse::decode(response)?)
    }
}

impl Default for ProjectsClient {
    fn default() -> Self {
        let base_url = env::var("PROJECTS_BASE_URL")
            .unwrap_or("http://projects".to_string())
            .trim_end_matches('/')
            .to_string();

        Self { 
            http_client: Default::default(),
            base_url
        }
    }
}