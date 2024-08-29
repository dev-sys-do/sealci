use crate::config::{Config, SingleConfig};
use actix_web::{web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub(crate) configs: Arc<RwLock<Config>>,
}

#[derive(Serialize)]
struct ConfigWithId {
    id: usize,
    event: String,
    repo_owner: String,
    repo_name: String,
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
) -> Result<impl Responder> {
    let id = path.into_inner();
    let configs = data.configs.read().await;

    if id >= 1 && id <= configs.configurations.len() {
        let config = &configs.configurations[id - 1];
        Ok(HttpResponse::Ok().json(config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}

pub async fn get_actions_file(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let configs = data.configs.read().await;

    if id >= 1 && id <= configs.configurations.len() {
        let config = &configs.configurations[id - 1];
        let actions_file_content = std::fs::read_to_string(&config.actions_path)
            .unwrap_or_else(|_| "Actions file not found".to_string());
        Ok(HttpResponse::Ok().body(actions_file_content))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}

#[derive(Deserialize)]
pub struct NewConfig {
    event: String,
    repo_owner: String,
    repo_name: String,
    github_token: String,
    actions_path: String,
}

pub async fn add_configuration(data: web::Data<AppState>, new_config: web::Json<NewConfig>) -> impl Responder {
    let mut configs = data.configs.write().await;
    let config = SingleConfig {
        event: new_config.event.clone(),
        repo_owner: new_config.repo_owner.clone(),
        repo_name: new_config.repo_name.clone(),
        github_token: new_config.github_token.clone(),
        actions_path: new_config.actions_path.clone(),
    };
    configs.configurations.push(config.clone());
    if let Err(e) = configs.save_to_file() {
        return HttpResponse::InternalServerError().body(format!("Failed to save config: {}", e));
    }
    HttpResponse::Ok().json(config)
}

pub async fn update_configuration(
    data: web::Data<AppState>,
    path: web::Path<usize>,
    new_config: web::Json<NewConfig>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut configs = data.configs.write().await;

    if id >= 1 && id <= configs.configurations.len() {
        let config = &mut configs.configurations[id - 1];
        config.event = new_config.event.clone();
        config.repo_owner = new_config.repo_owner.clone();
        config.repo_name = new_config.repo_name.clone();
        config.github_token = new_config.github_token.clone();
        config.actions_path = new_config.actions_path.clone();
        let updated_config = config.clone();
        if let Err(e) = configs.save_to_file() {
            return Ok(HttpResponse::InternalServerError().body(format!("Failed to save config: {}", e)));
        }
        Ok(HttpResponse::Ok().json(updated_config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}

pub async fn delete_configuration(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut configs = data.configs.write().await;

    if id >= 1 && id <= configs.configurations.len() {
        let removed_config = configs.configurations.remove(id - 1);
        if let Err(e) = configs.save_to_file() {
            return Ok(HttpResponse::InternalServerError().body(format!("Failed to save config: {}", e)));
        }
        Ok(HttpResponse::Ok().json(removed_config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}
