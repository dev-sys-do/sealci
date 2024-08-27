use reqwest::Client;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use crate::config::Config;

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

    response.json().await.ok()
}

async fn get_latest_commit(config: &Config) -> Option<String> {
    let url = format!("{}/commits", get_github_api_url(&config.repo_owner, &config.repo_name));
    let commits = request_github_api(&url, &config.github_token).await?;
    commits.get(0)?["sha"].as_str().map(String::from)
}

async fn get_latest_pull_request(config: &Config) -> Option<u64> {
    let url = format!("{}/pulls", get_github_api_url(&config.repo_owner, &config.repo_name));
    let pull_requests = request_github_api(&url, &config.github_token).await?;
    pull_requests.get(0)?["id"].as_u64()
}

pub async fn listen_to_commits(
    config: &Config,
    callback: impl Fn() + Send + 'static
) {
    let mut last_commit = get_latest_commit(config).await;
    println!("-- SealCI - Last commit found: {:?}", last_commit);

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new commits...");
        if let Some(current_commit) = get_latest_commit(config).await {
            if Some(&current_commit) != last_commit.as_ref() { // If there is a new commit
                println!("-- SealCI - New commit found: {:?}", current_commit);
                last_commit = Some(current_commit);
                callback();
            }
        }
    }
}

pub async fn listen_to_pull_requests(
    config: &Config,
    callback: impl Fn() + Send + 'static
) {
    let mut last_pull_request = get_latest_pull_request(config).await;
    println!("-- SealCI - Last pull request found: {:?}", last_pull_request);

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new pull requests...");
        if let Some(current_pull_request) = get_latest_pull_request(config).await {
            if Some(&current_pull_request) != last_pull_request.as_ref() { // If there is a new pull request
                println!("-- SealCI - New pull request found: {:?}", current_pull_request);
                last_pull_request = Some(current_pull_request);
                callback();
            }
        }
    }
}
