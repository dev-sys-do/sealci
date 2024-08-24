use crate::config::{Config, SingleConfig};
use crate::event_listener::get_github_repo_url;
use crate::{create_commit_listener, create_pull_request_listener};
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

pub enum RequestType {
    Create,
    Update,
    Delete,
}

pub async fn manage_threads(
    request_type: &RequestType,
    thread_list: Data<RwLock<JoinSet<()>>>,
    configs: Data<RwLock<Config>>,
) {
    {
        let mut thread_list = thread_list.write().await;
        thread_list.shutdown().await;
    }

    match request_type {
        RequestType::Create | RequestType::Update | RequestType::Delete => {
            let configs_read = configs.read().await;
            for conf in configs_read.configurations.iter() {
                let mut thread_list = thread_list.write().await;
                create_thread(conf, &mut *thread_list).await;
            }
        }
    }
}



pub async fn create_thread(config: &SingleConfig, thread_list: &mut JoinSet<()>) {
    let repo_url = get_github_repo_url(&config.repo_owner, &config.repo_name);
    thread_list.spawn(create_commit_listener(Arc::new(config.clone()), repo_url.clone()));
    thread_list.spawn(create_pull_request_listener(Arc::new(config.clone()), repo_url));
}