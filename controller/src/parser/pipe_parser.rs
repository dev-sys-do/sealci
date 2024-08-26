use serde::{Deserialize, Serialize};
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub name: String,
    pub commands: Vec<String>,
    pub configuration_type: Type,
    pub configuration_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Type {
    Container,
}

pub trait ManifestParser: Sync + Send {
    fn parse(&self, yaml: String) -> Result<Pipeline, ParsingError>;
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    InconsistentCommandIndentation,
    YamlNotCompliant,
    InvalidActionName,
    MissingName,
    MissingActions,
    MissingConfiguration,
    MissingCommands,
    MissingStepName,
}

#[derive(Clone)]
pub struct MockManifestParser {}

impl ManifestParser for MockManifestParser {
    fn parse(&self, yaml: String) -> Result<Pipeline, ParsingError> {
        check_command_indentation(&yaml)?;
        let doc = parse_yaml(&yaml)?;
        let name = parse_pipeline_name(&doc)?;
        let actions = parse_actions(&doc)?;

        Ok(Pipeline { name, actions })
    }
}

fn parse_yaml(yaml: &str) -> Result<Yaml, ParsingError> {
    let docs = YamlLoader::load_from_str(yaml).map_err(|_| ParsingError::YamlNotCompliant)?;
    docs.get(0).cloned().ok_or(ParsingError::YamlNotCompliant)
}

fn parse_pipeline_name(doc: &Yaml) -> Result<String, ParsingError> {
    doc["name"]
        .as_str()
        .ok_or(ParsingError::MissingName)
        .map(String::from)
}

fn parse_actions(doc: &Yaml) -> Result<Vec<Action>, ParsingError> {
    let actions_yaml = doc["actions"]
        .as_hash()
        .ok_or(ParsingError::MissingActions)?;
    actions_yaml
        .iter()
        .map(|(name, action)| parse_action(name, action))
        .collect()
}

fn parse_action(name: &Yaml, action: &Yaml) -> Result<Action, ParsingError> {
    let name = parse_action_name(name)?;
    let configuration = parse_configuration(action)?;
    let commands = parse_commands(action)?;

    Ok(Action {
        name,
        commands,
        configuration_type: Type::Container,
        configuration_version: configuration,
    })
}

fn parse_action_name(name: &Yaml) -> Result<String, ParsingError> {
    let name = name
        .as_str()
        .ok_or(ParsingError::MissingStepName)?
        .to_string();
    if !is_valid_action_name(&name) {
        return Err(ParsingError::InvalidActionName);
    }
    Ok(name)
}

fn parse_configuration(action: &Yaml) -> Result<String, ParsingError> {
    let config = action["configuration"]
        .as_hash()
        .ok_or(ParsingError::MissingConfiguration)?;
    if !config.keys().all(|k| k.as_str() == Some("container")) {
        return Err(ParsingError::YamlNotCompliant);
    }
    config
        .get(&Yaml::String("container".to_string()))
        .and_then(|v| v.as_str())
        .ok_or(ParsingError::MissingConfiguration)
        .map(String::from)
}

fn parse_commands(action: &Yaml) -> Result<Vec<String>, ParsingError> {
    let commands = action["commands"]
        .as_vec()
        .ok_or(ParsingError::MissingCommands)?;
    if commands.is_empty() {
        return Err(ParsingError::MissingCommands);
    }
    commands
        .iter()
        .map(|cmd| {
            cmd.as_str()
                .ok_or(ParsingError::YamlNotCompliant)
                .map(String::from)
        })
        .collect()
}

fn is_valid_action_name(name: &str) -> bool {
    let valid_chars = |c: char| c.is_alphanumeric() || c == ' ' || c == '&' || c == '-' || c == '_';
    !name.is_empty() && name.chars().all(valid_chars)
}

fn check_command_indentation(yaml: &str) -> Result<(), ParsingError> {
    let lines: Vec<&str> = yaml.lines().collect();
    let mut in_commands = false;
    let mut command_indent = None;

    for line in lines {
        if line.trim().starts_with("commands:") {
            in_commands = true;
            continue;
        }
        if in_commands && line.trim().starts_with('-') {
            let indent = line.chars().take_while(|&c| c == ' ').count();
            if let Some(prev_indent) = command_indent {
                if indent != prev_indent {
                    return Err(ParsingError::InconsistentCommandIndentation);
                }
            } else {
                command_indent = Some(indent);
            }
        }
    }
    Ok(())
}
