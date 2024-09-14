use std::sync::Arc;

use sqlx::PgPool;

use super::command_repository::CommandRepository;

pub struct CommandService {
    repository: CommandRepository,
}

impl CommandService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            repository: CommandRepository::new(pool),
        }
    }

    pub async fn create(&self, action_id: i64, command: &String) -> Result<(), sqlx::Error> {
        self.repository.create(action_id, command).await?;
        Ok(())
    }
}
