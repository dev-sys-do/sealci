use crate::domain::{
    entities::command::{Command, CommandError},
    repositories::command_repository::CommandRepository,
};
use async_trait::async_trait;
use std::sync::Arc;

use super::postgres::Postgres;

pub struct PostgresCommandRepository {
    pub postgres: Arc<Postgres>,
}

impl PostgresCommandRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

#[async_trait]
impl CommandRepository for PostgresCommandRepository {
    async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Command>, CommandError> {
        let result = sqlx::query_as!(
            Command,
            "SELECT id, action_id, command FROM commands WHERE action_id = $1",
            action_id
        )
        .fetch_all(&*self.postgres.get_pool())
        .await;

        result.map_err(CommandError::DatabaseError)
    }

    async fn create(&self, action_id: i64, command: String) -> Result<Command, CommandError> {
        let result = sqlx::query_as!(Command, "INSERT INTO commands (action_id, command) VALUES ($1, $2) RETURNING id, action_id, command", action_id, command)
          .fetch_one(&*self.postgres.get_pool())
          .await;

        result.map_err(CommandError::DatabaseError)
    }
}
