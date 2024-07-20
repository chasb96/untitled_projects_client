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