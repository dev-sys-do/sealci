use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::parser::pipe_parser::Type;

use super::{action_repository::ActionRepository};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionDTO {
    pub id: Option<i64>,
    pub pipeline_id: i64,
    pub name: String,
    pub container_uri: String,
    pub r#type: Type,
    pub status: String,
}

pub struct ActionService {
    repository: Arc<ActionRepository>
}

impl ActionService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        let repository = Arc::new(ActionRepository::new(pool.clone()));
        Self { repository }
    }

    pub async fn create(&self, action_dto: &ActionDTO) {
        self.repository.create(action_dto.pipeline_id, &action_dto.name, &action_dto.container_uri, &action_dto.r#type, &action_dto.status).await;
    }
}