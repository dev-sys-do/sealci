mod config;
mod event_listener;
mod controller;

use crate::config::Config;
use crate::event_listener::{get_github_repo_url, listen_to_commits, listen_to_pull_requests};
use crate::controller::send_to_controller;
use clap::{Arg, Command};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Runtime;


fn main() {
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
        Config::from_file(config_path)
    } else {
        println!("-- SealCI - Loading config from CLI arguments");
        Config {
            event: matches.get_one::<String>("event").expect("--event argument is required").to_string(),
            repo_owner: matches.get_one::<String>("repo_owner").expect("--repo_owner argument is required").to_string(),
            repo_name: matches.get_one::<String>("repo_name").expect("--repo_name argument is required").to_string(),
            github_token: matches.get_one::<String>("github_token").expect("--github_token argument is required").to_string(),
            actions_path: matches.get_one::<String>("actions_path").expect("--actions_path argument is required").to_string()
        }
    };

    println!("-- SealCI - Config loaded !");
    println!("{:#?}", config);

    // Borrowing the config by reference
    let config: Arc<Config> = Arc::new(config);

    if config.event == "commit" || config.event == "*" {
        let rt: Runtime = Runtime::new().expect("Failed to create runtime");

        listen_to_commits(&config, {
            let config: Arc<Config> = Arc::clone(&config);
            let repo_url: String = get_github_repo_url(&config.repo_owner, &config.repo_name);
            
            move || {
                rt.block_on(async {
                    match send_to_controller("pipeline_name", &repo_url, Path::new(&config.actions_path)).await {
                        Ok(_) => println!("Pipeline sent successfully"),
                        Err(e) => eprintln!("Failed to send pipeline: {}", e),
                    }
                });
            }
        });
    }
    if config.event == "pull_request" || config.event == "*" {
        let rt: Runtime = Runtime::new().expect("Failed to create runtime");

        listen_to_pull_requests(&config, {
            let config: Arc<Config> = Arc::clone(&config);
            let repo_url: String = get_github_repo_url(&config.repo_owner, &config.repo_name);
            
            move || {
                rt.block_on(async {
                    match send_to_controller("pipeline_name", &repo_url, Path::new(&config.actions_path)).await {
                        Ok(_) => println!("Pipeline sent successfully"),
                        Err(e) => eprintln!("Failed to send pipeline: {}", e),
                    }
                });
            }
        });
    }
}
