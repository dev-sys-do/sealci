use crate::config::{Config, SingleConfig};
use crate::constants::{CONFIG_NOT_FOUND, INVALID_EVENT_ERROR, MISSING_CONFIG, VALID_EVENTS};
use crate::file_utils::process_multipart_form;
use crate::thread_utils::{manage_threads, RequestType};
use actix_multipart::Multipart;
use actix_web::web::Data;
use actix_web::{web, Error, HttpResponse, Responder, Result};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

pub struct AppState {
    pub(crate) configs: Arc<RwLock<Config>>,
}

#[derive(Serialize)]
pub struct ConfigWithId {
    id: usize,
    event: String,
    pub(crate) repo_owner: String,
    pub(crate) repo_name: String,
    github_token: String,
    actions_path: String,
}

pub async fn get_configurations(data: web::Data<AppState>) -> impl Responder {
    let configs = data.configs.read().await;

    let configs_with_id: Vec<ConfigWithId> = configs
        .configurations
        .iter()
        .enumerate()
        .map(|(index, config)| ConfigWithId {
            id: (index + 1),
            event: config.event.clone(),
            repo_owner: config.repo_owner.clone(),
            repo_name: config.repo_name.clone(),
            github_token: config.github_token.clone(),
            actions_path: config.actions_path.clone(),
        })
        .collect();

    HttpResponse::Ok().json(configs_with_id)
}

pub async fn get_configuration_by_id(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let configs = data.configs.read().await;

    if id >= 1 && id <= configs.configurations.len() {
        let config = &configs.configurations[id - 1];
        Ok(HttpResponse::Ok().json(config))
    } else {
        Ok(HttpResponse::NotFound().body(CONFIG_NOT_FOUND))
    }
}

pub async fn add_configuration(
    payload: Multipart,
    data: Data<AppState>,
    thread_list: Data<RwLock<JoinSet<()>>>,
) -> Result<HttpResponse, Error> {
    let result = process_multipart_form(payload).await?;

    if let Some(actions_path) = result.actions_file_path {
        let single_config = SingleConfig {
            event: result.new_config.event,
            repo_owner: result.new_config.repo_owner,
            repo_name: result.new_config.repo_name,
            github_token: result.new_config.github_token,
            actions_path,
        };

        {
            let configs = data.configs.read().await;
            let existing_config = configs.configurations.iter().any(|config| {
                config.repo_owner == single_config.repo_owner &&
                    config.repo_name == single_config.repo_name &&
                    (config.event == single_config.event || config.event == "*" || single_config.event == "*")
            });

            if existing_config {
                return Err(actix_web::error::ErrorBadRequest(
                    "Configuration for this repository and event already exists",
                ));
            }
        }

        {
            let mut configs = data.configs.write().await;
            configs.configurations.push(single_config.clone());
            configs.save_to_file().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to save config: {}", e))
            })?;
        }

        manage_threads(&RequestType::Create, thread_list.clone(), Data::from(Arc::clone(&data.configs))).await;

        Ok(HttpResponse::Ok().json(single_config))
    } else {
        Err(actix_web::error::ErrorBadRequest(MISSING_CONFIG))
    }
}

pub async fn update_configuration(
    data: Data<AppState>,
    payload: Multipart,
    path: web::Path<usize>,
    thread_list: Data<RwLock<JoinSet<()>>>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    // Process the form payload before updating the configuration
    let result = process_multipart_form(payload).await?;

    // Lock the configs, update, and save changes
    let updated_config = {
        let mut configs = data.configs.write().await;

        // Check if the configuration exists
        if id < 1 || id > configs.configurations.len() {
            return Ok(HttpResponse::NotFound().body(CONFIG_NOT_FOUND));
        }

        // Borrow the config mutably
        let config = &mut configs.configurations[id - 1];
        config.event = result.new_config.event;
        config.repo_owner = result.new_config.repo_owner;
        config.repo_name = result.new_config.repo_name;
        config.github_token = result.new_config.github_token;

        // If a new actions file path was provided, update the config with the new path
        if let Some(actions_path) = result.actions_file_path {
            config.actions_path = actions_path;
        }

        let updated_config = config.clone();
        configs.save_to_file().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to save config: {}", e))
        })?;

        updated_config
    };

    // Restart threads with the updated configuration after lock release
    manage_threads(&RequestType::Update, thread_list.clone(), Data::from(Arc::clone(&data.configs))).await;

    // Return the updated config
    Ok(HttpResponse::Ok().json(updated_config))
}

pub async fn delete_configuration(
    data: Data<AppState>,
    path: web::Path<usize>,
    thread_list: Data<RwLock<JoinSet<()>>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();

    let removed_config = {
        let mut configs = data.configs.write().await;

        // Check if the configuration exists
        if id < 1 || id > configs.configurations.len() {
            return Ok(HttpResponse::NotFound().body(CONFIG_NOT_FOUND));
        }

        // Remove the configuration and save changes
        let removed_config = configs.configurations.remove(id - 1);
        configs.save_to_file().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to save config: {}", e))
        })?;

        removed_config
    };

    // Restart threads after deleting the configuration
    manage_threads(&RequestType::Delete, thread_list.clone(), Data::from(Arc::clone(&data.configs))).await;

    Ok(HttpResponse::Ok().json(removed_config))
}

pub async fn get_actions_file(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let configs = data.configs.read().await;

    if id >= 1 && id <= configs.configurations.len() {
        let config = &configs.configurations[id - 1];
        let actions_file_content = std::fs::read_to_string(&config.actions_path)
            .unwrap_or_else(|_| "Actions file not found".to_string());
        Ok(HttpResponse::Ok().body(actions_file_content))
    } else {
        Ok(HttpResponse::NotFound().body(CONFIG_NOT_FOUND))
    }
}

// Helper function to validate event
pub(crate) fn validate_event(event: &str) -> Result<(), Error> {
    if VALID_EVENTS.contains(&event) {
        Ok(())
    } else {
        Err(actix_web::error::ErrorBadRequest(format!(
            "{}: {} - valid events are: commit, pull_request, *",
            INVALID_EVENT_ERROR, event
        )))
    }
}
