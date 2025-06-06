use std::net::AddrParseError;
#[derive(Debug)]
pub enum Error {
    Unhandled,
    RestartAgentError(agent::models::error::Error),
    RestartControllerError(controller::application::AppError),
    RestartMonitorError(monitor::error::Error),
    RestartSchedulerError(sealci_scheduler::errors::Error),
    ToggleAgentError(agent::models::error::Error),
    ToggleControllerError(controller::application::AppError),
    ToggleMonitorError(monitor::error::Error),
    ConfigureControllerError(controller::application::AppError),
    ConfigureAgentError(agent::models::error::Error),
    ConfigureMonitorError(monitor::error::Error),
    ConfigureSchedulerError(sealci_scheduler::errors::Error),
    ParseError(AddrParseError),
    StartGrpcError(String),
}

impl Into<String> for Error {
    fn into(self) -> String {
        match self {
            Error::Unhandled => "An unhandled error occurred".to_string(),
            Error::RestartAgentError(e) => format!("Failed to restart agent: {}", e),
            Error::RestartControllerError(e) => format!("Failed to restart controller: {}", e),
            Error::RestartMonitorError(e) => format!("Failed to restart monitor: {}", e),
            Error::ToggleAgentError(e) => format!("Failed to toggle agent: {}", e),
            Error::ToggleControllerError(e) => format!("Failed to toggle controller: {}", e),
            Error::ToggleMonitorError(e) => format!("Failed to toggle monitor: {}", e),
            Error::RestartSchedulerError(_e) => "Failed to restart scheduler".to_string(),
            Error::ConfigureAgentError(error) => {
                format!("Failed to configure agent: {}", error)
            }
            Error::ConfigureControllerError(error) => {
                format!("Failed to configure controller: {}", error)
            }
            Error::ConfigureMonitorError(error) => {
                format!("Failed to configure monitor: {}", error)
            }
            Error::ConfigureSchedulerError(_e) => "Failed to configure scheduler".to_string(),
            Error::ParseError(e) => format!("Failed to parse address: {}", e),
            Error::StartGrpcError(e) => format!("Failed to start gRPC: {}", e),
        }
    }
}
