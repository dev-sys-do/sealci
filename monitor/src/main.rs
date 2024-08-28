mod config;
mod event_listener;
mod controller;

use crate::config::{Config, SingleConfig};
use crate::event_listener::{get_github_repo_url, listen_to_commits, listen_to_pull_requests};
use crate::controller::send_to_controller;
use clap::{Arg, Command};
use std::path::Path;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    // CLI arguments
    let matches = Command::new("GitHub Monitor")
        .version("1.0")
        .about("Monitors a GitHub repository for changes")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .required(false)
            .help("The path to the config file"))
        .arg(Arg::new("event")
            .short('e')
            .long("event")
            .required(false)
            .help("The event to listen to (commit, pull_request, *)"))
        .arg(Arg::new("repo_owner")
            .short('o')
            .long("repo_owner")
            .required(false)
            .help("The owner of the repo to watch"))
        .arg(Arg::new("repo_name")
            .short('n')
            .long("repo_name")
            .required(false)
            .help("The name of the repo to watch"))
        .arg(Arg::new("github_token")
            .short('t')
            .long("github_token")
            .required(false)
            .help("The GitHub token"))
        .arg(Arg::new("actions_path")
            .short('a')
            .long("actions_path")
            .required(false)
            .help("The path to the actions file"))
        .get_matches();

    let config: Config = if let Some(config_path) = matches.get_one::<String>("config") {
        println!("-- SealCI - Loading config from file: {:?}", config_path);
        Config::from_file(config_path).expect("Failed to load config from file")
    } else {
        println!("-- SealCI - Loading config from CLI arguments");
        Config {
            configurations: vec![SingleConfig {
                event: matches.get_one::<String>("event").expect("--event argument is required").to_string(),
                repo_owner: matches.get_one::<String>("repo_owner").expect("--repo_owner argument is required").to_string(),
                repo_name: matches.get_one::<String>("repo_name").expect("--repo_name argument is required").to_string(),
                github_token: matches.get_one::<String>("github_token").expect("--github_token argument is required").to_string(),
                actions_path: {
                    let path = matches.get_one::<String>("actions_path").expect("--actions_path argument is required").to_string();
                    let temp_config = SingleConfig {
                        event: String::new(),
                        repo_owner: String::new(),
                        repo_name: String::new(),
                        github_token: String::new(),
                        actions_path: path.clone(),
                    };
                    Config::exists_actions_file(&temp_config);
                    path
                }
            }],
        }
    };

    println!("-- SealCI - Config loaded !");
    println!("{:#?}", config);

    let mut handles = vec![];

    // Iterate over each configuration
    for single_config in config.configurations {
        let config = Arc::new(single_config);

        let repo_url = get_github_repo_url(&config.repo_owner, &config.repo_name);

        // Create a listener for commits
        let commit_listener = {
            let config = Arc::clone(&config);
            let repo_url = repo_url.clone();
            tokio::spawn(async move {
                if config.event == "commit" || config.event == "*" {
                    let callback = {
                        let config = Arc::clone(&config);
                        let repo_url = repo_url.clone();
                        move || {
                            let config = Arc::clone(&config);
                            let repo_url = repo_url.clone();
                            tokio::spawn(async move {
                                match send_to_controller("pipeline_name", &repo_url, Path::new(&config.actions_path)).await {
                                    Ok(_) => println!("Pipeline sent successfully"),
                                    Err(e) => eprintln!("Failed to send pipeline: {}", e),
                                }
                            });
                        }
                    };

                    listen_to_commits(&config, callback).await;
                }
            })
        };

        // Create a listener for pull requests
        let pull_request_listener = {
            let config = Arc::clone(&config);
            let repo_url = repo_url.clone();
            tokio::spawn(async move {
                if config.event == "pull_request" || config.event == "*" {
                    let callback = {
                        let config = Arc::clone(&config);
                        let repo_url = repo_url.clone();
                        move || {
                            let config = Arc::clone(&config);
                            let repo_url = repo_url.clone();
                            tokio::spawn(async move {
                                match send_to_controller("pipeline_name", &repo_url, Path::new(&config.actions_path)).await {
                                    Ok(_) => println!("Pipeline sent successfully"),
                                    Err(e) => eprintln!("Failed to send pipeline: {}", e),
                                }
                            });
                        }
                    };

                    listen_to_pull_requests(&config, callback).await;
                }
            })
        };

        // Stores the handles to wait for them to finish
        handles.push(commit_listener);
        handles.push(pull_request_listener);
    }

    // Wait for all listeners to finish
    futures::future::join_all(handles).await;
}
