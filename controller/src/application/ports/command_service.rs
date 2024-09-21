use async_trait::async_trait;

use crate::domain::entities::command::{Command, CommandError};

#[async_trait]
pub trait CommandService: Send + Sync {
    async fn create(&self, action_id: i64, command: String) -> Result<Command, CommandError>;
    async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Command>, CommandError>;
}
