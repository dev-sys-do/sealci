use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Config {
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

        let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
        config
    }
}