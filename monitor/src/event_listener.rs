use crate::config::SingleConfig;
use reqwest::{Client, Response};
use serde_json::Value;
use tracing::{info, error};
use std::error::Error;
use tokio::time::{sleep, Duration};

use crate::controller::send_to_controller;
use std::future::Future;
use std::path::Path;
use std::sync::Arc;


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

    if !response.status().is_success() {
        error!("GitHub API request failed: {:?}", response.status());
    }

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

// async fn get_latest_pull_request(config: &SingleConfig) -> Result<u64, Box<dyn Error>> {
//     let url = format!(
//         "{}/pulls",
//         get_github_api_url(&config.repo_owner, &config.repo_name)
//     );
//     let pull_requests = request_github_api(&url, &config.github_token).await?;
//     let latest_pr = match pull_requests.get(0) {
//         Some(pr) => pr["id"].as_u64(),
//         None => return Err("No pull requests found".into()),
//     };
//     let last_pr_id = match latest_pr {
//         Some(pr) => pr,
//         None => return Err("Id of latest pull request not found".into()),
//     };
//     Ok(last_pr_id)
// }

pub async fn listen_to_commits(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static,
) -> Result<(), Box<dyn Error>> {
    // Get the latest commit and unwrap the result properly
    let mut last_commit = get_latest_commit(config).await?;
    info!("Last commit found: {}", last_commit);

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        info!("{}/{} - Checking for new commits...", config.repo_owner, config.repo_name);

        // Handle the Result from `get_latest_commit`
        match get_latest_commit(config).await {
            Ok(current_commit) => {
                // Compare the latest commit with the current one
                if last_commit != current_commit {
                    info!("{}/{} - New commit found: {}", config.repo_owner, config.repo_name, current_commit);
                    last_commit = current_commit;
                    callback();
                }
            }
            Err(e) => {
                // Handle errors (such as network issues or API problems)
                error!("Error fetching the latest commit: {}", e);
            }
        }
    }
}

// async fn get_pull_request_details(
//     config: &SingleConfig,
//     pr_number: u64,
// ) -> Result<Value, Box<dyn Error>> {
//     let url = format!(
//         "{}/pulls/{}",
//         get_github_api_url(&config.repo_owner, &config.repo_name),
//         pr_number
//     );
//     let pull_request = request_github_api(&url, &config.github_token).await?;
//     Ok(pull_request)
// }

// async fn get_open_pull_requests(config: &SingleConfig) -> Result<Vec<Value>, Box<dyn Error>> {
//     let url = format!(
//         "{}/pulls?state=open",
//         get_github_api_url(&config.repo_owner, &config.repo_name)
//     );
//     let pull_requests = request_github_api(&url, &config.github_token).await?;
//     Ok(pull_requests.as_array().unwrap().to_vec())
// }

async fn get_latest_pull_request(config: &SingleConfig) -> Option<(u64, String)> {
    let url = format!("{}/pulls", get_github_api_url(&config.repo_owner, &config.repo_name));
    
    // Manually handle the Result
    let pull_requests = match request_github_api(&url, &config.github_token).await {
        Ok(data) => data,
        Err(e) => {
            error!("Error fetching pull requests: {}", e);
            return None; // Return None in case of error
        }
    };
    
    let pull_request_id = pull_requests.get(0)?.get("id")?.as_u64()?;
    let pull_request_title = pull_requests.get(0)?.get("title")?.as_str()?.to_string();
    Some((pull_request_id, pull_request_title))
}


pub async fn listen_to_pull_requests(
    config: &SingleConfig,
    callback: impl Fn() + Send + 'static
) {
    let mut last_pull_request = get_latest_pull_request(config).await;
    if let Some((id, title)) = &last_pull_request {
        info!("Last pull request found: {} - {}", id, title);
    }

    loop {
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
        info!("{}/{} - Checking for new pull requests...", config.repo_owner, config.repo_name);
        if let Some((current_pull_request, current_title)) = get_latest_pull_request(config).await {
            if Some(&(current_pull_request, current_title.clone())) != last_pull_request.as_ref() { // If there is a new pull request
                info!("{}/{} - New pull request found: {} - {}", config.repo_owner, config.repo_name, current_pull_request, current_title);
                last_pull_request = Some((current_pull_request, current_title));
                callback();
            }
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
            let callback = create_callback(
                Arc::clone(&config),
                repo_url.clone(),
                Arc::clone(&controller_endpoint),
            );
            let _ = listen_to_commits(&config, callback).await;
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
            let callback = create_callback(
                Arc::clone(&config),
                repo_url.clone(),
                Arc::clone(&controller_endpoint),
            );
            let _ = listen_to_pull_requests(&config, callback).await;
        }
    }
}

pub fn create_callback(
    config: Arc<SingleConfig>,
    repo_url: String,
    controller_endpoint: Arc<String>,
) -> impl Fn() {
    move || {
        info!("Callback triggered");
        let config = Arc::clone(&config);
        let repo_url = repo_url.clone();
        let controller_endpoint_clone = controller_endpoint.clone();
        tokio::spawn(async move {
            info!("Sending pipeline to controller...");
            match send_to_controller(
                &repo_url,
                Path::new(&config.actions_path),
                controller_endpoint_clone,
            )
            .await
            {
                Ok(_) => info!("Pipeline sent successfully"),
                Err(e) => error!("Failed to send pipeline: {}", e),
            }
        });
    }
}
