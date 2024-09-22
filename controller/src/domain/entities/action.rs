use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub enum ActionType {
    Container,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Container => write!(f, "container"),
        }
    }
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "container" => Ok(ActionType::Container),
            _ => Err(()),
        }
    }
}

impl From<String> for ActionType {
    fn from(s: String) -> Self {
        ActionType::from_str(&s).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq)]
pub enum ActionStatus {
    Pending,
    Running,
    Completed,
    Error,
}

impl fmt::Display for ActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ActionStatus::Pending => "Pending",
            ActionStatus::Running => "Scheduled",
            ActionStatus::Completed => "Completed",
            ActionStatus::Error => "Error",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for ActionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(ActionStatus::Pending),
            "Scheduled" => Ok(ActionStatus::Running),
            "Running" => Ok(ActionStatus::Running),
            "Completed" => Ok(ActionStatus::Completed),
            "Error" => Ok(ActionStatus::Error),
            _ => Err(()),
        }
    }
}

impl From<String> for ActionStatus {
    fn from(s: String) -> Self {
        ActionStatus::from_str(&s).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ActionRequest {
    pub action_id: u32,
    pub commands: Vec<String>,
    pub context: ExecutionContext,
    pub repo_url: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub r#type: i32,
    pub container_image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ActionResponse {
    pub action_id: u32,
    pub log: String,
    pub result: Option<ActionResult>,
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub completion: ActionStatus,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub r#type: ActionType,
    pub container_uri: String,
    #[sqlx(default)]
    pub commands: Vec<String>,
    pub status: ActionStatus,
}

impl Action {
    pub fn new(
        id: i64,
        pipeline_id: i64,
        name: String,
        status: ActionStatus,
        r#type: ActionType,
        container_uri: String,
        commands: Vec<String>,
    ) -> Self {
        Self {
            id,
            pipeline_id,
            name,
            status,
            r#type,
            container_uri,
            commands: commands,
        }
    }
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Error while creating action: {0}")]
    CreateError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    InvalidStatus(String),
    #[error("Invalid input: {0}")]
    InvalidType(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActionDTO {
    pub action_id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub r#type: String,
    pub container_uri: String,
    pub status: String,
    pub command: String,
    pub command_id: i64,
}
