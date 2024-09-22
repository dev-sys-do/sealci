use reqwest::Client;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use log::info;

use crate::controller::send_to_controller;
use crate::config::{ SingleConfig};
use std::sync::Arc;
use std::future::Future;
use std::path::Path;



pub fn get_github_api_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://api.github.com/repos/{}/{}", repo_owner, repo_name)
}

pub fn get_github_repo_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://github.com/{}/{}", repo_owner, repo_name)
}

async fn request_github_api(url: &str, token: &str) -> Option<Value> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "rust-reqwest")
        .header("Authorization", format!("token {}", token))
        .send()
        .await
        .ok()?;

    info!("-- SealCI - GitHub API response: {:?}", response.status());
    response.json().await.ok()
}

async fn get_latest_commit(config: &SingleConfig) -> Option<(String, String)> {
    let url = format!("{}/commits", get_github_api_url(&config.repo_owner, &config.repo_name));
    let commits = request_github_api(&url, &config.github_token).await?;
    let commit_sha = commits.get(0)?["sha"].as_str()?.to_string();
    let commit_message = commits.get(0)?["commit"]["message"].as_str()?.to_string();
    Some((commit_sha, commit_message))
}

async fn get_latest_pull_request(config: &SingleConfig) -> Option<(u64, String)> {
    let url = format!("{}/pulls", get_github_api_url(&config.repo_owner, &config.repo_name));
    let pull_requests = request_github_api(&url, &config.github_token).await?;
    let pull_request_id = pull_requests.get(0)?["id"].as_u64()?;
    let pull_request_title = pull_requests.get(0)?["title"].as_str()?.to_string();
    Some((pull_request_id, pull_request_title))
}

pub async fn listen_to_commits(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static
) {
    let mut last_commit = get_latest_commit(config).await;
    if let Some((sha, message)) = &last_commit {
        println!("-- SealCI - Last commit found: {} - {}", sha, message);
    }

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new commits...");
        if let Some((current_commit, current_message)) = get_latest_commit(config).await {
            if Some(&(current_commit.clone(), current_message.clone())) != last_commit.as_ref() { // If there is a new commit
                println!("-- SealCI - New commit found: {} - {}", current_commit, current_message);
                last_commit = Some((current_commit, current_message));
                callback();
            }
        }
    }
}

pub async fn listen_to_pull_requests(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static
) {
    let mut last_pull_request = get_latest_pull_request(config).await;
    if let Some((id, title)) = &last_pull_request {
        println!("-- SealCI - Last pull request found: {} - {}", id, title);
    }

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new pull requests...");
        if let Some((current_pull_request, current_title)) = get_latest_pull_request(config).await {
            if Some(&(current_pull_request, current_title.clone())) != last_pull_request.as_ref() { // If there is a new pull request
                println!("-- SealCI - New pull request found: {} - {}", current_pull_request, current_title);
                last_pull_request = Some((current_pull_request, current_title));
                callback();
            }
        }
    }
}

pub fn create_commit_listener(config: Arc<SingleConfig>, repo_url: String) -> impl Future<Output = ()> {
    async move {
        if config.event == "commit" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone());
            listen_to_commits(&config, callback).await;
        }
    }
}

pub fn create_pull_request_listener(config: Arc<SingleConfig>, repo_url: String) -> impl Future<Output = ()> {
    async move {
        if config.event == "pull_request" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone());
            listen_to_pull_requests(&config, callback).await;
        }
    }
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