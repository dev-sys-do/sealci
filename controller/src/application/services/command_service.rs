use crate::{
    application::ports::command_service::CommandService,
    domain::{
        entities::command::{Command, CommandError},
        repositories::command_repository::CommandRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CommandServiceImpl {
    repository: Arc<Box<dyn CommandRepository + Send + Sync>>,
}

impl CommandServiceImpl {
    pub fn new(repository: Arc<Box<dyn CommandRepository + Send + Sync>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandService for CommandServiceImpl {
    async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Command>, CommandError> {
        self.repository.find_by_action_id(action_id).await
    }

    async fn create(&self, action_id: i64, command: String) -> Result<Command, CommandError> {
        self.repository.create(action_id, command).await
    }
}
