use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub event: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub github_token: String,
    pub actions_path: String,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(&path).expect("Failed to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read config file");

        let config: Config = serde_yaml::from_str(&mut contents).expect("Failed to parse YAML");

        // Verify that the actions file exists
        Self::exists_actions_file(&config);

        config
    }

    pub(crate) fn exists_actions_file(config: &Config) {
        if !Path::new(&config.actions_path).exists() {
            panic!("The actions file '{}' does not exist.", config.actions_path);
        }
    }
}
