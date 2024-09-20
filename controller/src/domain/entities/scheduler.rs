use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Error while executing pipeline: {0}")]
    Error(String),
}
