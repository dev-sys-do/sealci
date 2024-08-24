use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bollard::exec::{CreateExecResults, StartExecResults};
use futures_util::StreamExt;
use log::info;
use tokio::{spawn, sync::mpsc::UnboundedSender, time::sleep};
use tonic::Status;

use crate::{
    container::{create_exec, inspect_exec, launch_container, start_exec},
    proto::{ActionResponseStream, ActionResult},
};

// TODO: manage all unwrap on lock on log_input and action_id

pub async fn launch_action(
    image_name: String,
    commands: &mut Vec<String>,
    log_input: Arc<Mutex<UnboundedSender<Result<ActionResponseStream, Status>>>>,
    action_id: Arc<Mutex<String>>,
) -> Result<(), Status> {
    let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
        log: "Launching action".to_string(),
        action_id: action_id.lock().unwrap().to_string(),
        result: Some(ActionResult {
            completion: 1,
            exit_code: None,
        }),
    }));
    let id: String = match launch_container(&image_name).await {
        Ok(id) => {
            let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
                log: format!("Container launched using image: {}", image_name),
                action_id: action_id.lock().unwrap().to_string(),
                result: Some(ActionResult {
                    completion: 1,
                    exit_code: None,
                }),
            }));
            id
        }
        Err(e) => {
            return Err(Status::aborted(format!(
                "Launching error
         {}",
                e
            )))
        }
    };

    for command in &mut *commands {
        let log_input = Arc::clone(&log_input);
        let action_id = Arc::clone(&action_id);
        let exec_id;

        match create_exec(command, id.as_str()).await {
            Ok(CreateExecResults { id }) => {
                exec_id = id;
                let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
                    log: command.clone(),
                    action_id: action_id.lock().unwrap().to_string(),
                    result: Some(ActionResult {
                        completion: 2,
                        exit_code: None,
                    }),
                }));
            }
            Err(_) => return Err(Status::aborted("Error happened when creating exec")),
        }

        match start_exec(&exec_id).await {
            Ok(StartExecResults::Attached {
                mut output,
                input: _,
            }) => {
                spawn(async move {
                    while let Some(log) = output.next().await {
                        match log {
                            Ok(log_output) => {
                                let _ = &log_input.lock().unwrap().send(Ok(ActionResponseStream {
                                    log: log_output.to_string(),
                                    action_id: action_id.lock().unwrap().to_string(),
                                    result: Some(ActionResult {
                                        completion: 2,
                                        exit_code: None,
                                    }),
                                }));
                            }
                            Err(e) => {
                                let _ = log_input.lock().unwrap().send(Ok(ActionResponseStream {
                                    log: "Step exited with an error".to_string(),
                                    action_id: action_id.lock().unwrap().to_string(),
                                    result: Some(ActionResult {
                                        completion: 3,
                                        exit_code: Some(1),
                                    }),
                                }));
                                return Err(Status::aborted(format!("Execution error: {}", e)));
                            }
                        }
                    }
                    Ok(())
                });
            }
            Ok(StartExecResults::Detached) => {
                return Err(Status::aborted("Can't attach to container"))
            }
            Err(_) => return Err(Status::aborted("Error happened when launching action")),
        }

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
    }
    Ok(())
}
