use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
  pub id: i64,
  pub action_id: i64,
  pub data: String,
}

impl Log {
  pub fn new(id: i64, action_id: i64, data: String) -> Log {
    Log { id, action_id, data }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum LogError {
  #[error("Database error: {0}")]
  DatabaseError(#[from] sqlx::Error)
}