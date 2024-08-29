mod config;
mod event_listener;
mod controller;
mod external_api;

use crate::config::{Config, SingleConfig};
use crate::controller::send_to_controller;
use crate::event_listener::{get_github_repo_url, listen_to_commits, listen_to_pull_requests};
use crate::external_api::{add_configuration, delete_configuration, get_actions_file, get_configuration_by_id, get_configurations, update_configuration, AppState};
use actix_web::{web, App, HttpServer};
use clap::{Arg, Command};
use std::path::Path;
use std::sync::Arc;
use tokio;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();
    let configs = Arc::new(RwLock::new(config));

    println!("Launching API and listening to events on Github repository...");

    tokio::select! {
        _ = launch_api_server(Arc::clone(&configs)) => {},
        _ = launch_github_listeners(Arc::clone(&configs)) => {},
    }

    Ok(())
}

fn get_config() -> Config {
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
        let repo_name = matches.get_one::<String>("repo_name").expect("--repo_name argument is required");
        Config {
            configurations: vec![SingleConfig {
                event: matches.get_one::<String>("event").expect("--event argument is required").to_string(),
                repo_owner: matches.get_one::<String>("repo_owner").expect("--repo_owner argument is required").to_string(),
                repo_name: repo_name.to_string(),
                github_token: matches.get_one::<String>("github_token").expect("--github_token argument is required").to_string(),
                actions_path: {
                    let path = matches.get_one::<String>("actions_path").expect("--actions_path argument is required").to_string();
                    Config::exists_actions_file(&path.clone(), &repo_name);
                    path
                }
            }],
            file_path: None,
        }
    };

    println!("-- SealCI - Config loaded !");
    println!("{:#?}", config);
    config
}

async fn launch_api_server(configs: Arc<RwLock<Config>>) -> std::io::Result<()> {
    let data = web::Data::new(AppState { configs: Arc::clone(&configs) });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/configurations", web::get().to(get_configurations))
            .route("/configurations", web::post().to(add_configuration))
            .route("/configurations/{id}", web::get().to(get_configuration_by_id))
            .route("/configurations/{id}", web::put().to(update_configuration))
            .route("/configurations/{id}", web::delete().to(delete_configuration))
            .route("/configurations/{id}/actions-file", web::get().to(get_actions_file))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn launch_github_listeners(configs: Arc<RwLock<Config>>) {
    let mut handles = vec![];
    let configurations = {
        let configs_read = configs.read().await;
        configs_read.configurations.clone()
    };
    for single_config in configurations {
        let config = Arc::new(single_config);
        let repo_url = get_github_repo_url(&config.repo_owner, &config.repo_name);

        let commit_listener = create_commit_listener(Arc::clone(&config), repo_url.clone());
        let pull_request_listener = create_pull_request_listener(Arc::clone(&config), repo_url.clone());

        handles.push(commit_listener);
        handles.push(pull_request_listener);
    }

    futures::future::join_all(handles).await;
}

fn create_commit_listener(config: Arc<SingleConfig>, repo_url: String) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if config.event == "commit" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone());
            listen_to_commits(&config, callback).await;
        }
    })
}

fn create_pull_request_listener(config: Arc<SingleConfig>, repo_url: String) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if config.event == "pull_request" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone());
            listen_to_pull_requests(&config, callback).await;
        }
    })
}

fn create_callback(config: Arc<SingleConfig>, repo_url: String) -> impl Fn() {
    move || {
        let config = Arc::clone(&config);
        let repo_url = repo_url.clone();
        tokio::spawn(async move {
            match send_to_controller(&repo_url, Path::new(&config.actions_path)).await {
                Ok(_) => println!("Pipeline sent successfully"),
                Err(e) => eprintln!("Failed to send pipeline: {}", e),
            }
        });
    }
}

