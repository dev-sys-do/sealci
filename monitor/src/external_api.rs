use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::config::{Config, SingleConfig};

struct AppState {
    configs: Arc<RwLock<Config>>,
}

#[actix_web::main]
pub async fn launch_external_api(configs: Arc<RwLock<Config>>) -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        configs: Arc::clone(&configs),
    });
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

#[derive(Serialize)]
struct ConfigWithId {
    id: usize,
    event: String,
    repo_owner: String,
    repo_name: String,
    github_token: String,
    actions_path: String,
}

async fn get_configurations(data: web::Data<AppState>) -> impl Responder {
    let configs = data.configs.read().unwrap();

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

async fn get_configuration_by_id(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let configs = data.configs.read().unwrap();

    if id >= 1 && id <= configs.configurations.len() {
        let config = &configs.configurations[id - 1];
        Ok(HttpResponse::Ok().json(config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}

async fn get_actions_file(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let configs = data.configs.read().unwrap();

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
struct NewConfig {
    event: String,
    repo_owner: String,
    repo_name: String,
    github_token: String,
    actions_path: String,
}

async fn add_configuration(data: web::Data<AppState>, new_config: web::Json<NewConfig>) -> impl Responder {
    let mut configs = data.configs.write().unwrap();
    let config = SingleConfig {
        event: new_config.event.clone(),
        repo_owner: new_config.repo_owner.clone(),
        repo_name: new_config.repo_name.clone(),
        github_token: new_config.github_token.clone(),
        actions_path: new_config.actions_path.clone(),
    };
    configs.configurations.push(config.clone());
    HttpResponse::Ok().json(config)
}

async fn update_configuration(
    data: web::Data<AppState>,
    path: web::Path<usize>,
    new_config: web::Json<NewConfig>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut configs = data.configs.write().unwrap();

    if id >= 1 && id <= configs.configurations.len() {
        let config = &mut configs.configurations[id - 1];
        config.event = new_config.event.clone();
        config.repo_owner = new_config.repo_owner.clone();
        config.repo_name = new_config.repo_name.clone();
        config.github_token = new_config.github_token.clone();
        config.actions_path = new_config.actions_path.clone();
        Ok(HttpResponse::Ok().json(config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}

async fn delete_configuration(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut configs = data.configs.write().unwrap();

    if id >= 1 && id <= configs.configurations.len() {
        let removed_config = configs.configurations.remove(id - 1);
        Ok(HttpResponse::Ok().json(removed_config))
    } else {
        Ok(HttpResponse::NotFound().body("Configuration not found"))
    }
}
