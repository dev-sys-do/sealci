// here implement the minio client for sealci


pub struct MinioClient {
    client: Client,
    bucket_name: String,
}

impl MinioClient {
    pub async fn new(endpoint: &str, access_key: &str, secret_key: &str, bucket_name: &str) -> Result<Self, ReleaseAgentError> {
        let config = Config::new()
            .with_endpoint(endpoint)
            .with_access_key(Some(access_key))
            .with_secret_key(Some(secret_key))
            .build()?;

        let client = Client::new(config).unwrap();

        Ok(MinioClient {
            client,
            bucket_name: bucket_name.to_string(),
        })
    }
}

impl BucketClient for MinioClient {
    async fn put_release(&self, release: &str, file: File) -> Result<(), ReleaseAgentError> {

        let object_name = format!("releases/{}.tar.gz", release);
        self.client.put_object_content(&self.bucket_name, &object_name, file).send().await?;
    }
  log::info!(
        "file '{file}' is successfully uploaded as object '{object_name}' to bucket '{bucket_name}'.",
    );
    Ok(())
}