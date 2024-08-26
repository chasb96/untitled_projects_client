use prost::Message;

#[derive(Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

#[derive(Message)]
pub struct ListProjectsResponse {
    #[prost(message, repeated, tag = "1")]
    pub projects: Vec<ListProjectsResponseProject>,
}

#[derive(Message)]
pub struct ListProjectsResponseProject {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
}

#[derive(Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[prost(string, tag = "4")]
    pub event_id: String,
    #[prost(message, repeated, tag = "5")]
    pub files: Vec<ProjectFileResponse>,
}

#[derive(Message)]
pub struct ProjectFileResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
}

#[derive(Message)]
pub struct ListTagsResponse {
    #[prost(string, repeated, tag = "1")]
    pub tags: Vec<String>,
}