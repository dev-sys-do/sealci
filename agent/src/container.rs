use bollard::container::Config;
use bollard::errors::Error;
use bollard::exec::{self, CreateExecResults, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::secret::{ContainerCreateResponse, CreateImageInfo, ExecInspectResponse};
use futures_util::TryStreamExt;
use tracing::info;

use crate::dockerLocal;

pub async fn launch_container(image_name: &str) -> Result<String, bollard::errors::Error> {
    create_image(image_name).await?;
    info!("Image {} created", image_name);
    let config = create_config(image_name);
    info!("Config created");
    let ContainerCreateResponse { id, warnings: _ } = create_container(config).await?;
    info!("Container created");
    start_container(&id).await?;
    info!("Container started");
    return Ok(id);
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
        entrypoint: Some(vec!["/bin/sh"]),
        image: Some(image_name),
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

pub async fn stop_container(container_name: &str) -> Result<(), bollard::errors::Error> {
    return dockerLocal.stop_container(container_name, None).await;
}

pub async fn remove_container(container_id: &str) -> Result<(), bollard::errors::Error> {
    return dockerLocal.remove_container(container_id, None).await;
}

pub async fn create_exec(
    command: &str,
    container_id: &str,
    workdir: Option<String>,
) -> Result<CreateExecResults, bollard::errors::Error> {
    dockerLocal
        .create_exec(
            &container_id,
            exec::CreateExecOptions {
                cmd: Some(command.split(' ').map(String::from).collect()),
                tty: Some(true),
                attach_stdin: Some(true),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                working_dir: workdir,
                ..Default::default()
            },
        )
        .await
}
pub async fn start_exec(exec_id: &str) -> Result<StartExecResults, bollard::errors::Error> {
    Ok(dockerLocal.start_exec(&exec_id, None).await?)
}

pub async fn inspect_exec(exec_id: &str) -> Result<ExecInspectResponse, bollard::errors::Error> {
    dockerLocal.inspect_exec(&exec_id).await
}
