use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tonic::Status;

use crate::{
    container::{execute_commands, launch_container},
    proto::{ActionResponseStream, ActionResult},
};

pub async fn launch_action(
    image_name: String,
    commands: &mut Vec<String>,
    log_input: UnboundedSender<Result<ActionResponseStream, Status>>,
    action_id: String,
) -> Result<(), Status> {
    let id: String = match launch_container(&image_name).await {
        Ok(id) => id,
        Err(e) => {
            return Err(Status::aborted(format!(
                "Launching error
         {}",
                e
            )))
        }
    };
    match execute_commands(commands, &id).await {
        Ok(mut output_stream) => {
            tokio::spawn(async move {
                while let Some(log) = output_stream.next().await {
                    match log {
                        Ok(log_output) => {
                            let _ = &log_input.send(Ok(ActionResponseStream {
                                log: log_output.to_string(),
                                action_id: action_id.to_string(),
                                result: Some(ActionResult {
                                    completion: 0,
                                    exit_code: Some(0),
                                }),
                            }));
                        }
                        Err(e) => {
                            let _ = log_input.send(Ok(ActionResponseStream {
                                log: "Action exited with an error".to_string(),
                                action_id: action_id.to_string(),
                                result: Some(ActionResult {
                                    completion: 3,
                                    exit_code: Some(1),
                                }),
                            }));
                            let _ = log_input
                                .send(Err(Status::cancelled(format!("Error happened {}", e))));
                        }
                    }
                }
            });
        }
        Err(_) => return Err(Status::aborted("Error happened when launching action")),
    };
    Ok(())
}
