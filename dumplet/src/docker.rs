use bollard::Docker;
use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use futures_util::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::errors::DumpletError;

pub async fn export_docker_image(image_name: &str, output_path: &str) -> Result<(), DumpletError> {
    let docker = Docker::connect_with_local_defaults()?;

    // Pull the image
    println!("Pulling image {}...", image_name);
    let options = Some(CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);
    while let Some(_) = stream.next().await {}

    // Create a temporary container
    let container_config = bollard::container::Config {
        image: Some(image_name),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let container = docker.create_container::<String, _>(None, container_config).await?;
    let container_id = container.id;

    // Export filesystem to TAR
    let export_stream = docker.export_container(&container_id);

    let path = Path::new(output_path);
    let mut file = File::create(&path)?;
    let mut stream = export_stream;

    while let Some(Ok(chunk)) = stream.next().await {
        file.write_all(&chunk)?;
    }

    println!("Export completed: {}", output_path);
    Ok(())
}
