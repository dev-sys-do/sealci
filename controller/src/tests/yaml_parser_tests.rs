use std::fs::File;
use std::io::Read;
use crate::parser::pipe_parser::{ManifestParser, MockManifestParser, ParsingError, Type};

fn read_yaml_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_parsing() {
        let yaml_content = read_yaml_file("src/tests/data/classic_pipeline.yaml");

        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_ok());
        let pipeline = result.unwrap();

        assert_eq!(pipeline.name, "Simple Web App Pipeline");
        assert_eq!(pipeline.actions.len(), 3);

        let build_action = pipeline.actions.iter().find(|a| a.name == "build").unwrap();
        assert_eq!(build_action.configuration_version, "node:14");
        assert_eq!(build_action.configuration_type, Type::Container);
        assert_eq!(build_action.commands.len(), 2);
        assert_eq!(build_action.commands[0], "npm install");
        assert_eq!(build_action.commands[1], "npm run build");

        let test_action = pipeline.actions.iter().find(|a| a.name == "test").unwrap();
        assert_eq!(test_action.configuration_version, "node:14");
        assert_eq!(test_action.configuration_type, Type::Container);
        assert_eq!(test_action.commands.len(), 2);
        assert_eq!(test_action.commands[0], "npm run test");
        assert_eq!(test_action.commands[1], "npm run lint");

        let deploy_action = pipeline.actions.iter().find(|a| a.name == "deploy").unwrap();
        assert_eq!(deploy_action.configuration_version, "amazon/aws-cli");
        assert_eq!(deploy_action.configuration_type, Type::Container);
        assert_eq!(deploy_action.commands.len(), 2);
        assert!(deploy_action.commands[0].contains("s3://my-app-bucket"));
        assert!(deploy_action.commands[1].contains("aws cloudfront create-invalidation"));
    }

    #[test]
    fn test_yaml_parsing_without_name() {
        let yaml_content = read_yaml_file("src/tests/data/unnamed_pipeline.yaml");

        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsingError::MissingName);
    }

    #[test]
    fn test_yaml_parsing_with_missing_actions() {
        let yaml_content = read_yaml_file("src/tests/data/missing_actions_pipeline.yaml");

        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsingError::MissingActions);
    }

    #[test]
    fn test_yaml_parsing_with_invalid_data() {
        let yaml_content = read_yaml_file("src/tests/data/invalid_pipeline.yaml");

        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsingError::YamlNonCompliant);
    }

    #[test]
    fn test_yaml_parsing_empty_commands() {
        let yaml_content = read_yaml_file("src/tests/data/empty_commands_pipeline.yaml");
        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsingError::MissingCommands);
    }

    #[test]
    fn test_yaml_parsing_special_characters_valid() {
        let yaml_content = read_yaml_file("src/tests/data/valid_special_characters_pipeline.yaml");
        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_ok());
        if let Ok(pipeline) = result {
            assert_eq!(pipeline.name, "Special Characters Pipeline");
            assert_eq!(pipeline.actions.len(), 1);
            assert_eq!(pipeline.actions[0].name, "build & test");
            assert_eq!(pipeline.actions[0].commands, vec!["echo \"Hello, World!\"", "npm run test"]);
            assert_eq!(pipeline.actions[0].configuration_type, Type::Container);
            assert_eq!(pipeline.actions[0].configuration_version, "node:14");
        }
    }

    #[test]
    fn test_yaml_parsing_special_characters_invalid() {
        let yaml_content = read_yaml_file("src/tests/data/invalid_special_characters_pipeline.yaml");
        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert!(matches!(result, Err(ParsingError::InvalidActionName)));
    }

    #[test]
    fn test_inconsistent_command_indentation() {
        let yaml_content = read_yaml_file("src/tests/data/inconsistent_command_indentation.yaml");
        let parser = MockManifestParser {};
        let result = parser.parse(yaml_content);

        assert!(result.is_err());
        assert!(matches!(result, Err(ParsingError::InconsistentCommandIndentation)));
    }
}
