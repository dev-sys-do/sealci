pub const SERVER_ADDRESS: &'static str = "0.0.0.0:8080";

// pub constants for field names, file paths and event types

pub const EVENT: &'static str = "event";
pub const REPO_OWNER: &'static str = "repo_owner";
pub const REPO_NAME: &'static str = "repo_name";
pub const GITHUB_TOKEN: &'static str = "github_token";
pub const ACTIONS_DIR: &'static str = "./actions/";
pub const VALID_EVENTS: [&'static str; 3] = ["commit", "pull_request", "*"];
pub const CONFIG_NOT_FOUND: &'static str = "Configuration not found";
pub const MISSING_CONFIG: &'static str = "Missing config data or file";
pub const FILE_CREATION_ERROR: &'static str = "Failed to create or overwrite file";
pub const DIRECTORY_CREATION_ERROR: &'static str = "Failed to create directory";
pub const INVALID_EVENT_ERROR: &'static str = "Invalid event";
