mod config;
mod controller;
mod external_api;
mod event_listener;
mod constants;
mod file_utils;
mod thread_utils;

use crate::config::{Config, SingleConfig};
use crate::constants::SERVER_ADDRESS;
use crate::external_api::{add_configuration, delete_configuration, get_actions_file, get_configuration_by_id, get_configurations, update_configuration, AppState};
use crate::thread_utils::create_thread;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::{Arg, Command};
use std::sync::Arc;
use tokio;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = match get_config().await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Config load error"));
        }
    };
    let configs = Arc::new(RwLock::new(config));
    

    let thread_listeners_handles = Arc::new(RwLock::new(JoinSet::new()));

    println!("Launching API and listening to events on GitHub repository...");

    // Spawn the GitHub listeners in the background
    tokio::spawn({
        let configs = Arc::clone(&configs);
        let thread_listeners_handles = Arc::clone(&thread_listeners_handles);
        async move {
            let mut thread_list = thread_listeners_handles.write().await;
            launch_github_listeners(configs, &mut *thread_list).await;
        }
    });

    // Launch the API server as the main task
    launch_api_server(Arc::clone(&configs), Arc::clone(&thread_listeners_handles)).await?;

    Ok(())
}

async fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
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

    let config = if let Some(config_path) = matches.get_one::<String>("config") {
        println!("-- SealCI - Loading config from file: {:?}", config_path);
        Config::from_file(config_path).await?
    } else {
        println!("-- SealCI - Loading config from CLI arguments");

        let repo_name = matches
            .get_one::<String>("repo_name")
            .ok_or("--repo_name argument is required")?;

        Config {
            configurations: vec![SingleConfig {
                event: matches
                    .get_one::<String>("event")
                    .ok_or("--event argument is required")?
                    .to_string(),
                repo_owner: matches
                    .get_one::<String>("repo_owner")
                    .ok_or("--repo_owner argument is required")?
                    .to_string(),
                repo_name: repo_name.to_string(),
                github_token: matches
                    .get_one::<String>("github_token")
                    .ok_or("--github_token argument is required")?
                    .to_string(),
                actions_path: {
                    let path = matches
                        .get_one::<String>("actions_path")
                        .ok_or("--actions_path argument is required")?
                        .to_string();
                    
                    Config::exists_actions_file(&path, repo_name)?;
                    path
                },
            }],
            file_path: None,
        }
    };

    println!("-- SealCI - Config loaded !");
    println!("{:#?}", config);
    Ok(config)
}


async fn launch_api_server(configs: Arc<RwLock<Config>>,
                           thread_listeners_handles: Arc<RwLock<JoinSet<()>>>,
) -> std::io::Result<()> {
    let data = web::Data::new(AppState { configs: Arc::clone(&configs) });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(Data::from(Arc::clone(&thread_listeners_handles)))
            .route("/configurations", web::get().to(get_configurations))
            .route("/configurations", web::post().to(add_configuration))
            .route("/configurations/{id}", web::get().to(get_configuration_by_id))
            .route("/configurations/{id}", web::put().to(update_configuration))
            .route("/configurations/{id}", web::delete().to(delete_configuration))
            .route("/configurations/{id}/actions-file", web::get().to(get_actions_file))
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await
}

async fn launch_github_listeners(
    configs: Arc<RwLock<Config>>,
    thread_list: &mut JoinSet<()>,
) {
    let configurations = {
        let configs_read = configs.read().await;
        configs_read.configurations.clone()
    };

    for single_config in configurations {
        create_thread(&single_config, thread_list).await;
    }
}




