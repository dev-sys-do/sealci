use std::pin::Pin;

use bollard::{container::LogOutput, errors::Error};
use futures_util::Stream;

use crate::container::{execute_commands, launch_container};

pub async fn launch_action(
    image_name: String,
    commands: &mut Vec<&str>,
) -> Result<Pin<Box<dyn Stream<Item = Result<LogOutput, Error>> + Send>>, bollard::errors::Error> {
    let id: String = launch_container(&image_name).await?;
    return execute_commands(commands, &id).await;
}
