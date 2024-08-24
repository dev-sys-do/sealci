use reqwest::blocking::Client;
use serde_json::Value;
use std::{thread, time::Duration};

fn get_latest_commit(repo_owner: &str, repo_name: &str, token: &str) -> Option<String> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/commits", repo_owner, repo_name);
    
    let response = client
        .get(&url)
        .header("User-Agent", "rust-reqwest")
        .header("Authorization", format!("token {}", token))
        .send()
        .ok()?;

    let commits: Value = response.json().ok()?;
    commits.get(0)?["sha"].as_str().map(String::from)
}

pub fn listen_to_commits(
    repo_owner: &str, 
    repo_name: &str, 
    token: &str, 
    callback: impl Fn() + Send + 'static
) {
    let mut last_commit = get_latest_commit(repo_owner, repo_name, token);
    println!("-- SealCI - Last commit found: {:?}", last_commit);

    loop {
        thread::sleep(Duration::from_secs(10)); // Attendre 60 secondes avant la prochaine vérification
        println!("-- SealCI - Checking for new commits...");
        if let Some(current_commit) = get_latest_commit(repo_owner, repo_name, token) {
            if Some(&current_commit) != last_commit.as_ref() {
                println!("-- SealCI - New commit found: {:?}", current_commit);
                last_commit = Some(current_commit);
                callback();
            }
        }
    }
}
