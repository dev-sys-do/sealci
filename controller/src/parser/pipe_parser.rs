use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    name: String,
    actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    name: String,
    commands: Vec<String>,
}

pub trait ManifestParser {
    fn parse(&self, yaml: String) -> Result<Pipeline, ParsingError>;
}

#[derive(Debug)]
pub enum ParsingError {
    YamlNonCompliant = 0,
}

#[derive(Clone)]
pub struct MockManifestParser {}

impl ManifestParser for MockManifestParser {
    fn parse(&self, _: String) -> Result<Pipeline, ParsingError> {
        Ok(Pipeline {
            name: "Fake name".to_string(),
            actions: vec![Action {
                commands: vec!["npm run dev".to_string()],
                name: "test".to_string(),
            }],
        })
    }
}
