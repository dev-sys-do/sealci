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
}

#[async_trait]
impl BucketClient for MinioClient {
    async fn put_release(&self, release: String ,path_buf: PathBuf) -> Result<(), ReleaseAgentError> {
        let object_name = format!("releases/{}/{}", release, path_buf.clone().file_name().unwrap().to_str().unwrap());
        let path: &Path = path_buf.as_path();
        let content = ObjectContent::from(path);
        self.client
            .put_object_content(&self.bucket_name, &object_name,content)
            .send()
            .await.map_err(|_| ReleaseAgentError::BucketNotAvailable)?;
        info!(
            "file is successfully uploaded as object '{object_name}' to bucket '{bucket_name}'.",
            object_name = object_name,
            bucket_name = self.bucket_name
        );
        Ok(())
    }
}
