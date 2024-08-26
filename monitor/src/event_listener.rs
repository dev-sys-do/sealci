use reqwest::blocking::{Client, Response};
use serde_json::Value;

use tokio::time::{sleep, Duration};

use crate::config::Config;

pub fn get_github_api_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://api.github.com/repos/{}/{}", repo_owner, repo_name)
}

pub fn get_github_repo_url(repo_owner: &str, repo_name: &str) -> String {
    format!("https://github.com/{}/{}", repo_owner, repo_name)
}

fn request_github_api(url: &str, token: &str) -> Option<Value> {
    let client: Client = Client::new();
    let response: Response = client
        .get(url)
        .header("User-Agent", "rust-reqwest")
        .header("Authorization", format!("token {}", token))
        .send()
        .ok()?;

    response.json().ok()
}

fn get_latest_commit(config: &Config) -> Option<String> {
    let url: String = format!("{}/commits", get_github_api_url(&config.repo_owner, &config.repo_name));

    let commits: Value = request_github_api(&url, &config.github_token)?;
    commits.get(0)?["sha"].as_str().map(String::from)
}

fn get_latest_pull_request(config: &Config) -> Option<u64> {
    let url: String = format!("{}/pulls", get_github_api_url(&config.repo_owner, &config.repo_name));

    let pull_requests: Value = request_github_api(&url, &config.github_token)?;
    pull_requests.get(0)?["id"].as_u64()
}

pub async fn listen_to_commits(
    config: &Config,
    callback: impl Fn() + Send + 'static
) {
    let mut last_commit: Option<String> = get_latest_commit(config);
    println!("-- SealCI - Last commit found: {:?}", last_commit);

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new commits...");
        if let Some(current_commit) = get_latest_commit(config) {
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
    let mut last_pull_request: Option<u64> = get_latest_pull_request(config);
    println!("-- SealCI - Last pull request found: {:?}", last_pull_request);

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        println!("-- SealCI - Checking for new pull requests...");
        if let Some(current_pull_request) = get_latest_pull_request(config) {
            if Some(&current_pull_request) != last_pull_request.as_ref() { // If there is a new commit
                println!("-- SealCI - New pull request found: {:?}", current_pull_request);
                last_pull_request = Some(current_pull_request);
                callback();
            }
        }
    }
}