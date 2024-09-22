mod error;
mod events;
mod tags;
mod threads;
pub mod axum;
pub mod create_project;
pub mod create_event;
pub mod get_project_by_id;
pub mod list_projects;

use std::env;

use prost::Message;

pub use events::*;
pub use error::Error;
pub use tags::*;
pub use threads::*;

use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;

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

    pub async fn create_project(&self, request: create_project::CreateProjectRequest) -> Result<create_project::CreateProjectResponse, Error> {
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

        Ok(create_project::CreateProjectResponse::decode(response)?)
    }

    pub async fn list_projects(&self, query: list_projects::ListProjectsQuery) -> Result<list_projects::ListProjectsResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects?{}", self.base_url, query.to_query_string()))
            .header(ACCEPT, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(list_projects::ListProjectsResponse::decode(response)?)
    }

    pub async fn get_project_by_id(&self, project_id: &str) -> Result<get_project_by_id::ProjectResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}", self.base_url, project_id))
            .header(ACCEPT, "application/octet-stream")
            .header(CONTENT_TYPE, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(get_project_by_id::ProjectResponse::decode(response)?)
    }

    pub async fn create_event(&self, project_id: &str, event: create_event::EventRequest) -> Result<(), Error> {
        self.http_client
            .put(format!("{}/projects/{}", self.base_url, project_id))
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&event)?)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
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