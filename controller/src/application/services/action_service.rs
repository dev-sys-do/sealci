use crate::{
    application::ports::{action_service::ActionService, command_service::CommandService},
    domain::{
        entities::{
            action::{Action, ActionError, ActionType},
            command,
        },
        repositories::action_repository::ActionRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ActionServiceImpl {
    repository: Arc<Box<dyn ActionRepository + Send + Sync>>,
    command_service: Arc<Box<dyn CommandService + Send + Sync>>,
}

impl ActionServiceImpl {
    pub fn new(
        repository: Arc<Box<dyn ActionRepository + Send + Sync>>,
        command_service: Arc<Box<dyn CommandService + Send + Sync>>,
    ) -> Self {
        Self {
            repository,
            command_service,
        }
    }
}

#[async_trait]
impl ActionService for ActionServiceImpl {
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
        commands: Option<Vec<String>>,
    ) -> Result<Action, ActionError> {
        let action = self
            .repository
            .create(pipeline_id, name, container_uri, r#type, status)
            .await?;

        if let Some(commands_vec) = commands {
            for command_str in commands_vec {
                let _command = self.command_service.create(action.id, command_str).await;
            }
        }

        Ok(action)
    }

    async fn find_by_id(&self, action_id: i64) -> Result<Action, ActionError> {
        self.repository.find_by_id(action_id).await
    }

    async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, ActionError> {
        self.repository.find_by_pipeline_id(pipeline_id).await
    }
}
