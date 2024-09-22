use prost::Message;

pub enum ListProjectsQuery {
    Projects { project_ids: Vec<String> },
    Users { user_id: String },
}

impl ListProjectsQuery {
    pub fn to_query_string(&self) -> String {
        match self {
            ListProjectsQuery::Projects { project_ids } => format!("pids={}", project_ids.join(",")),
            ListProjectsQuery::Users { user_id } => format!("uid={}", user_id),
        }
    }
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