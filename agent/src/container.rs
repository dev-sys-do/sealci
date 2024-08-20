use bollard::container::{AttachContainerOptions, AttachContainerResults, Config, LogOutput};
use bollard::errors::Error;
use bollard::image::CreateImageOptions;
use bollard::secret::{ContainerCreateResponse, CreateImageInfo};
use futures_util::{Stream, TryStreamExt};
use std::pin::Pin;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::docker::dockerLocal;

pub async fn launch_container(image_name: &str) -> Result<String, bollard::errors::Error> {
    create_image(image_name).await?;
    let alpine_config = create_config(image_name);
    let ContainerCreateResponse { id, warnings } = create_container(alpine_config).await?;
    start_container(&id).await?;
    return Ok(id);
}

pub async fn execute_commands(
    commands: &mut Vec<&str>,
    container_id: &str,
) -> Result<Pin<Box<dyn Stream<Item = Result<LogOutput, Error>> + Send>>, bollard::errors::Error> {
    let exit_command = "exit";
    commands.push(&exit_command);
    let AttachContainerResults { output, input } = attach_container(&container_id).await?;
    write_commands(commands, input).await;
    return Ok(output);
}

pub async fn create_image(
    image_name: &str,
) -> Result<Vec<CreateImageInfo>, bollard::errors::Error> {
    return dockerLocal
        .create_image(
            Some(CreateImageOptions {
                from_image: image_name,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await;
}

pub fn create_config(image_name: &str) -> bollard::container::Config<&str> {
    return Config {
        image: Some(image_name),
        tty: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        open_stdin: Some(true),
        ..Default::default()
    };
}

pub async fn create_container(
    config: bollard::container::Config<&str>,
) -> Result<ContainerCreateResponse, Error> {
    return dockerLocal
        .create_container::<&str, &str>(None, config)
        .await;
}

pub async fn start_container(id: &str) -> Result<(), bollard::errors::Error> {
    return dockerLocal.start_container::<String>(&id, None).await;
}
pub async fn attach_container(id: &str) -> Result<AttachContainerResults, bollard::errors::Error> {
    return dockerLocal
        .attach_container(
            &id,
            Some(AttachContainerOptions::<String> {
                stdout: Some(true),
                stderr: Some(true),
                stdin: Some(true),
                stream: Some(true),
                logs: Some(true),
                ..Default::default()
            }),
        )
        .await;
}

pub async fn stop_container(container_name: &str) -> Result<(), bollard::errors::Error> {
    return dockerLocal.stop_container(container_name, None).await;
}

pub async fn remove_container(container_id: &str) -> Result<(), bollard::errors::Error> {
    return dockerLocal.remove_container(container_id, None).await;
}

pub async fn write_commands(commands: &mut Vec<&str>, mut input: Pin<Box<dyn AsyncWrite + Send>>) {
    for command in commands {
        input.write(command.as_bytes()).await.ok();
        input.write_all(b"\n").await.ok();
    }
}
