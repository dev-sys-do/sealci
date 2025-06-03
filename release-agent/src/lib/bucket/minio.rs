use std::path::{Path, PathBuf};

// here implement the minio client for sealci
use minio::s3::{builders::ObjectContent, creds::StaticProvider, Client, ClientBuilder};
use tonic::async_trait;
use tracing::info;

use crate::core::ReleaseAgentError;

use super::BucketClient;

#[derive(Debug, Clone)]
pub struct MinioClient {
    client: Client,
    bucket_name: String,
}

impl MinioClient {
    pub async fn new(
        endpoint: String,
        access_key: String,
        secret_key: String,
        bucket_name: String,
    ) -> Result<Self, ReleaseAgentError> {
        let static_provider = StaticProvider::new(access_key.as_str(), secret_key.as_str(), None);
        let client = ClientBuilder::new(
            endpoint
                .parse()
                .map_err(|_| ReleaseAgentError::InvalidBucketEndpoint)?,
        )
        .provider(Some(Box::new(static_provider)))
        .build()
        .map_err(|_| ReleaseAgentError::InvalidBucketEndpoint)?;

        Ok(MinioClient {
            client,
            bucket_name
        })
    }

    pub async fn put_object(&self, object_name: String, path_buth: PathBuf) -> Result<(), ReleaseAgentError> {
        let path: &Path = path_buth.as_path();
        let content = ObjectContent::from(path);
        self.client
            .put_object_content(&self.bucket_name, &object_name, content)
            .send()
            .await
            .map_err(|_| ReleaseAgentError::BucketNotAvailable)?;
        info!(
            "file is successfully uploaded as object '{object_name}' to bucket '{bucket_name}'.",
            object_name = object_name,
            bucket_name = self.bucket_name
        );
        Ok(())
    }
}

#[async_trait]
impl BucketClient for MinioClient {
    async fn put_release(&self, release: String ,release_path: PathBuf, sig_path: PathBuf) -> Result<(), ReleaseAgentError> {
        let release_object_name = format!("releases/{}/{}", release, release_path.clone().file_name().unwrap().to_str().unwrap());
        let sig_object_name = format!("releases/{}/{}", release, sig_path.clone().file_name().unwrap().to_str().unwrap());
        self.put_object(release_object_name, release_path).await?;
        self.put_object(sig_object_name, sig_path).await?;
        Ok(())
    }
}
