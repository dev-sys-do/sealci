use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bollard::exec::{CreateExecResults, StartExecResults};
use futures_util::StreamExt;
use log::info;
use tokio::{spawn, sync::mpsc::UnboundedSender, time::sleep};
use tonic::Status;
use url::Url;

use crate::{
    container::{
        create_exec, inspect_exec, launch_container, remove_container, start_exec, stop_container,
    },
    proto::{ActionResponseStream, ActionResult},
};

pub async fn launch_action(
    image_name: String,
    commands: &mut Vec<String>,
    log_input: Arc<Mutex<UnboundedSender<Result<ActionResponseStream, Status>>>>,
    action_id: Arc<Mutex<u32>>,
    repo_url: String,
) -> Result<(), Status> {
    let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
        log: "Launching action".to_string(),
        action_id: *action_id.lock().unwrap(),
        result: Some(ActionResult {
            completion: 1,
            exit_code: None,
        }),
    }));

    let container_id: String = match launch_container(&image_name).await {
        Ok(id) => id,
        Err(e) => return Err(Status::aborted(format!("Launching error: {}", e))),
    };

    let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
        log: format!("Container launched using image: {}", image_name),
        action_id: *action_id.lock().unwrap(),
        result: Some(ActionResult {
            completion: 1,
            exit_code: None,
        }),
    }));

    let repo_name = setup_repository(repo_url, container_id.as_str()).await?;

    let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
        log: format!("Repository {} cloned", repo_name),
        action_id: *action_id.lock().unwrap(),
        result: Some(ActionResult {
            completion: 1,
            exit_code: None,
        }),
    }));

    for command in &mut *commands {
        let log_input = Arc::clone(&log_input);
        let action_id = Arc::clone(&action_id);
        let absolute_path = format!("/{}", repo_name);
        let exec_id = start_command(
            command,
            &container_id,
            log_input,
            action_id,
            Some(absolute_path),
        )
        .await?;
        wait_for_command(exec_id, &container_id).await?;
    }
    clean_action(container_id.as_str()).await?;
    Ok(())
}

pub async fn setup_repository(repo_url: String, container_id: &str) -> Result<String, Status> {
    let setup_command = format!("git clone {}", repo_url);
    let exec_id = match create_exec(&setup_command, container_id, None).await {
        Ok(CreateExecResults { id }) => id,
        Err(_) => return Err(Status::aborted("Error happened when creating exec")),
    };
    let _ = start_exec(&exec_id).await;
    wait_for_command(exec_id, &container_id.to_string()).await?;
    let repo_name = match get_repo_name(&repo_url) {
        Some(repo_name) => Ok(repo_name),
        None => Err(Status::aborted("Error happened when getting repo name")),
    };
    repo_name
}

pub async fn start_command(
    command: &mut String,
    container_id: &str,
    log_input: Arc<Mutex<UnboundedSender<Result<ActionResponseStream, Status>>>>,
    action_id: Arc<Mutex<u32>>,
    repo_name: Option<String>,
) -> Result<String, Status> {
    let exec_id = match create_exec(&command.to_string(), container_id, repo_name).await {
        Ok(CreateExecResults { id }) => id,
        Err(_) => return Err(Status::aborted("Error happened when creating exec")),
    };
    let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
        log: command.clone(),
        action_id: *action_id.lock().unwrap(),
        result: Some(ActionResult {
            completion: 2,
            exit_code: None,
        }),
    }));
    let mut container_ouput = match start_exec(&exec_id).await {
        Ok(StartExecResults::Attached { output, input: _ }) => output,
        Ok(StartExecResults::Detached) => return Err(Status::aborted("Can't attach to container")),
        Err(_) => return Err(Status::aborted("Error happened when launching action")),
    };
    spawn(async move {
        while let Some(log) = container_ouput.next().await {
            let container_log_output = match log {
                Ok(log_output) => log_output,
                Err(e) => return Err(Status::aborted(format!("Execution error: {}", e))),
            };
            let _ = &log_input.lock().unwrap().send(Ok(ActionResponseStream {
                log: container_log_output.to_string(),
                action_id: *action_id.lock().unwrap(),
                result: Some(ActionResult {
                    completion: 2,
                    exit_code: None,
                }),
            }));
        }
        Ok(())
    });
    Ok(exec_id)
}

pub async fn wait_for_command(exec_id: String, container_id: &String) -> Result<(), Status> {
    loop {
        let exec_state = match inspect_exec(&exec_id).await {
            Ok(exec_state) => exec_state,
            Err(_) => return Err(Status::aborted("Error happened checking state of a step")),
        };
        match exec_state.exit_code {
            Some(0) => {
                break;
            }
            Some(exit_code) => {
                info!("Step exited with an error: {}", exit_code);
                clean_action(container_id).await?;
                return Err(Status::aborted("Step exited with an error"));
            }
            None => {}
        }
        match exec_state.running {
            Some(true) => {}
            Some(false) => {
                break;
            }
            None => {
                return Err(Status::aborted("Error happened checking state of a step"));
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}

pub async fn clean_action(container_id: &str) -> Result<(), Status> {
    match stop_container(container_id).await {
        Ok(_) => {
            info!("Container stopped");
        }
        Err(_) => return Err(Status::aborted("Error happened when stopping container")),
    };
    match remove_container(container_id).await {
        Ok(_) => {
            info!("Container stopped");
        }
        Err(_) => return Err(Status::aborted("Error happened when stopping container")),
    };
    Ok(())
}

fn get_repo_name(github_url: &str) -> Option<String> {
    let url = Url::parse(github_url).ok()?;
    let segments: Vec<&str> = url.path_segments()?.collect();
    segments.last().map(|s| s.to_string())
}
