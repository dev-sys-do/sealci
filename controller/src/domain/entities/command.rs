use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: i64,
    pub action_id: i64,
    pub command: String,
}

impl Command {
    pub fn new(id: i64, action_id: i64, command: String) -> Command {
        Command {
            id,
            action_id,
            command,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Command not found")]
    NotFound,
}
