use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct SingleConfig {
    pub event: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub github_token: String,
    pub actions_path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub configurations: Vec<SingleConfig>,
    pub file_path: Option<String>,
}

impl Config {
    pub async fn from_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(&path).await.map_err(|e| format!("Failed to open config file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.map_err(|e| format!("Failed to read config file: {}", e))?;

        let mut config: Config = serde_yaml::from_str(&contents).map_err(|e| format!("Failed to parse YAML: {}", e))?;
        config.file_path = Some(path.to_string());
        // Verify that the actions files exist for each configuration
        for single_config in &config.configurations {
            if !Path::new(&single_config.actions_path).exists() {
                return Err(format!("File {} not found", single_config.actions_path));
            }
        }

        Ok(config)
    } 

    pub(crate) fn exists_actions_file(actions_path: &String, repo_name: &String) -> Result<(), Box<dyn Error>> {
        if !Path::new(&actions_path).exists() {
            return Err(format!("The actions file '{}' for repo '{}' does not exist.", actions_path, repo_name).into());
        }
        Ok(())
    }

    pub async fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        if let Some(file_path) = &self.file_path {
            let serialized = serde_json::to_string_pretty(&self)?;
            fs::write(file_path, serialized).await.map_err(|e| format!("Failed to save file: {}", e))?;
            Ok(())
        } else {
            Err("No file path provided, configuration will not be saved to a file.".into())
        }
    }
}
