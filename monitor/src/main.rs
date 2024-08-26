use std::sync::Arc;

mod config;
mod controller;
mod event_listener;

use crate::config::Config;
use crate::controller::send_to_controller;
use crate::event_listener::{get_github_repo_url, listen_to_commits, listen_to_pull_requests};
use clap::{Arg, Command};
use std::path::Path;

#[tokio::main]
async fn main() {
    // CLI arguments
    let matches = Command::new("GitHub Monitor")
        .version("1.0")
        .about("Monitors a GitHub repository for changes")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .required(false)
                .help("The path to the config file"),
        )
        .arg(
            Arg::new("event")
                .short('e')
                .long("event")
                .required(false)
                .help("The event to listen to (commit, pull_request, *)"),
        )
        .arg(
            Arg::new("repo_owner")
                .short('o')
                .long("repo_owner")
                .required(false)
                .help("The owner of the repo to watch"),
        )
        .arg(
            Arg::new("repo_name")
                .short('n')
                .long("repo_name")
                .required(false)
                .help("The name of the repo to watch"),
        )
        .arg(
            Arg::new("github_token")
                .short('t')
                .long("github_token")
                .required(false)
                .help("The GitHub token"),
        )
        .arg(
            Arg::new("actions_path")
                .short('a')
                .long("actions_path")
                .required(false)
                .help("The path to the actions file"),
        )
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
            actions_path: {
                let path = matches.get_one::<String>("actions_path").expect("--actions_path argument is required").to_string();
                Config::exists_actions_file(&Config { actions_path: path.clone(), ..Default::default() });
                path
            }
        }
    };

    println!("-- SealCI - Config loaded !");
    println!("{:#?}", config);

    let config: Arc<Config> = Arc::new(config);
    let mut tasks = vec![];

    if config.event == "commit" || config.event == "*" {
        let config_clone = Arc::clone(&config);
        let task = tokio::spawn(async move {
            listen_to_commits(&config_clone, move || {
                let repo_url = get_github_repo_url(&config_clone.repo_owner, &config_clone.repo_name);
                let config_inner_clone = Arc::clone(&config_clone); // Clone the Arc
                tokio::spawn(async move {
                    match send_to_controller(
                        "pipeline_name",
                        &repo_url,
                        Path::new(&config_inner_clone.actions_path),
                    )
                    .await
                    {
                        Ok(_) => println!("Pipeline sent successfully for commit"),
                        Err(e) => eprintln!("Failed to send pipeline for commit: {}", e),
                    }
                });
            });
        });
        tasks.push(task);
    }

    if config.event == "pull_request" || config.event == "*" {
        let config_clone = Arc::clone(&config);
        let task = tokio::spawn(async move {
            listen_to_pull_requests(&config_clone, move || {
                let repo_url = get_github_repo_url(&config_clone.repo_owner, &config_clone.repo_name);
                let config_inner_clone = Arc::clone(&config_clone); // Clone the Arc
                tokio::spawn(async move {
                    match send_to_controller(
                        "pipeline_name",
                        &repo_url,
                        Path::new(&config_inner_clone.actions_path),
                    )
                    .await
                    {
                        Ok(_) => println!("Pipeline sent successfully for pull request"),
                        Err(e) => eprintln!("Failed to send pipeline for pull request: {}", e),
                    }
                });
            });
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.expect("Task panicked");
    }
}
