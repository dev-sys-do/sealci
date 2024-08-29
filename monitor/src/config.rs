use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    pub fn from_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(&path).map_err(|e| format!("Failed to open config file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| format!("Failed to read config file: {}", e))?;

        let mut config: Config = serde_yaml::from_str(&contents).map_err(|e| format!("Failed to parse YAML: {}", e))?;
        config.file_path = Some(path.to_string());

        // Verify that the actions files exist for each configuration
        for single_config in &config.configurations {
            Self::exists_actions_file(&single_config.actions_path, &single_config.repo_name);
        }
        Ok(config)
    }

    pub(crate) fn exists_actions_file(actions_path: &String, repo_name: &String) {
        if !Path::new(&actions_path).exists() {
            panic!("The actions file '{}' for repo '{}' does not exist.", &actions_path, repo_name);
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        if let Some(file_path) = &self.file_path {
            let serialized = serde_json::to_string_pretty(&self)?;
            std::fs::write(file_path, serialized)?;
            Ok(())
        } else {
            println!("No file path provided, configuration will not be saved to a file.");
            Ok(())
        }
    }
}
