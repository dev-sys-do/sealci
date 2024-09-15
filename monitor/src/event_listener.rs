use crate::config::SingleConfig;
use log::info;
use reqwest::{Client, Response};
use serde_json::Value;
use std::error::Error;
use tokio::time::{sleep, Duration};
use log::{info, error};

use crate::controller::send_to_controller;
use crate::config::SingleConfig;
use std::sync::Arc;
use std::future::Future;
use std::path::Path;



pub fn get_github_api_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://api.github.com/repos/{}/{}", repo_owner, repo_name)
}

pub fn get_github_repo_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://github.com/{}/{}", repo_owner, repo_name)
}

async fn request_github_api(url: &str, token: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let response: Response = client
        .get(url)
        .header("User-Agent", "rust-reqwest")
        .header("Authorization", format!("token {}", token))
        .send()
        .await?;

    info!("-- SealCI - GitHub API response: {:?}", response.status());
    let res = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(res)
}

async fn get_latest_commit(config: &SingleConfig) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "{}/commits",
        get_github_api_url(&config.repo_owner, &config.repo_name)
    );
    let commits = request_github_api(&url, &config.github_token).await?;
    let latest_commit = match commits.get(0) {
        Some(commit) => commit["sha"].as_str().map(String::from),
        None => return Err("No commits found".into()),
    };
    let last_commit_sha = match latest_commit {
        Some(commit) => commit,
        None => return Err("Sha of latest commit not found".into()),
    };
    Ok(last_commit_sha)
}

async fn get_latest_pull_request(config: &SingleConfig) -> Result<u64, Box<dyn Error>> {
    let url = format!(
        "{}/pulls",
        get_github_api_url(&config.repo_owner, &config.repo_name)
    );
    let pull_requests = request_github_api(&url, &config.github_token).await?;
    let latest_pr = match pull_requests.get(0) {
        Some(pr) => pr["id"].as_u64(),
        None => return Err("No pull requests found".into()),
    };
    let last_pr_id = match latest_pr {
        Some(pr) => pr,
        None => return Err("Id of latest pull request not found".into()),
    };
    Ok(last_pr_id)
}

pub async fn listen_to_commits(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static,
) -> Result<(), Box<dyn Error>> {
    let mut last_commit = get_latest_commit(config).await?;
    println!("-- SealCI - Last commit found: {:?}", last_commit);
    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        info!("-- SealCI - Checking for new commits...");
        let curent_commit = match get_latest_commit(config).await {
            Ok(current_commit) => {
                info!("-- SealCI - New commit found: {:?}", current_commit);
                current_commit
            }
            Err(_) => continue,
        };
        if last_commit != curent_commit {
            last_commit = curent_commit;
            callback();
        }
    }
}

pub async fn listen_to_pull_requests(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static,
) -> Result<(), Box<dyn Error>> {
    let mut last_pull_request = get_latest_pull_request(config).await?;
    info!(
        "-- SealCI - Last pull request found: {:?}",
        last_pull_request
    );

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        info!("-- SealCI - Checking for new pull requests...");
        let current_pull_request = match get_latest_pull_request(config).await {
            Ok(current_pull_request) => current_pull_request,
            Err(_) => continue,
        };
        if current_pull_request != last_pull_request {
            info!(
                "-- SealCI - New pull request found: {:?}",
                current_pull_request
            );
            last_pull_request = current_pull_request;
            callback();
        }
    }
}

pub fn create_commit_listener(
    config: Arc<SingleConfig>,
    repo_url: String,
    controller_endpoint: Arc<String>,
) -> impl Future<Output = ()> {
    async move {
        if config.event == "commit" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone(), controller_endpoint);
            listen_to_commits(&config, callback).await;
        }
    }
}

pub fn create_pull_request_listener(
    config: Arc<SingleConfig>,
    repo_url: String,
    controller_endpoint: Arc<String>,
) -> impl Future<Output = ()> {
    async move {
        if config.event == "pull_request" || config.event == "*" {
            let callback = create_callback(Arc::clone(&config), repo_url.clone(), controller_endpoint);
            listen_to_pull_requests(&config, callback).await;
        }
    }
}

fn create_callback(
    config: Arc<SingleConfig>,
    repo_url: String,
    controller_endpoint: Arc<String>
) -> impl Fn() {
    move || {
        let config = Arc::clone(&config);
        let repo_url = repo_url.clone();
        let controller_endpoint_clone = controller_endpoint.clone();
        tokio::spawn(async move {
            match send_to_controller(&repo_url, Path::new(&config.actions_path), controller_endpoint_clone).await {
                Ok(_) => info!("Pipeline sent successfully"),
                Err(e) => error!("Failed to send pipeline: {}", e),
            }
        });
    }
}