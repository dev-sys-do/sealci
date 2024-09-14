use std::{fmt, sync::Arc};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

use crate::{
    command::command_service::CommandService, grpc_scheduler::ActionStatus,
    parser::pipe_parser::Type,
};

use super::action_repository::{Action, ActionRepository};

#[derive(Debug)]
pub enum ActionCreationError {
    WrongTypeError,
    DatabaseInsertionError,
}

impl fmt::Display for ActionCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionCreationError::WrongTypeError => write!(f, "Wrong type error"),
            ActionCreationError::DatabaseInsertionError => write!(f, "Database insertion error"),
        }
    }
}

impl std::error::Error for ActionCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionDTO {
    pub id: Option<i64>,
    pub pipeline_id: i64,
    pub name: String,
    pub container_uri: String,
    pub r#type: Type,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandDTO {
    pub id: Option<i64>,
    pub action_id: i64,
    pub command: String,
}

pub struct ActionService {
    repository: Arc<ActionRepository>,
    command_service: Arc<CommandService>,
}

impl ActionService {
    pub fn new(pool: Arc<PgPool>, command_service: Arc<CommandService>) -> Self {
        let repository = Arc::new(ActionRepository::new(pool.clone()));
        Self {
            repository,
            command_service,
        }
    }

    pub async fn create(
        &self,
        action_dto: &ActionDTO,
        commands: Vec<String>,
    ) -> Result<Action, ActionCreationError> {
        let action_dto = self
            .repository
            .create(
                action_dto.pipeline_id,
                &action_dto.name,
                &action_dto.container_uri,
                &action_dto.r#type,
                &action_dto.status,
            )
            .await
            .map_err(|e| {
                info!("Error creating action: {:?}", e);
                return ActionCreationError::DatabaseInsertionError;
            })?;

        for command in &commands {
            self.command_service
                .create(action_dto.id.unwrap(), &command)
                .await
                .map_err(|_| {
                    return ActionCreationError::DatabaseInsertionError;
                })?;
        }

        Action::new(
            action_dto.id.unwrap(),
            action_dto.pipeline_id,
            action_dto.name.clone(),
            action_dto.container_uri.clone(),
            commands,
            action_dto.r#type.clone(),
            action_dto.status.clone(),
        )
        .map_err(|_| {
            return ActionCreationError::WrongTypeError;
        })
    }

    pub async fn update_status(&self, id: i64, status: &ActionStatus) -> Result<(), sqlx::Error> {
        self.repository.alter_status(status.as_str_name(), id).await
    }
}
