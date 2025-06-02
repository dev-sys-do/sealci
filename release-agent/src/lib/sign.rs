use std::{fs::File, io::Write, path::PathBuf};
use minisign::SecretKey;
use tonic::async_trait;

use crate::core::ReleaseAgentError;

///! A trait for signing releases.
/// This trait is used to sign a release archive, such as a tarball or zip file,
/// The return file is the signed archive, which can be used for distribution.
/// The private key is used to sign the archive.
#[async_trait]
pub trait ReleaseSigner: Clone + Send + Sync {
    fn sign_release(&self, compressed_archived: File, file_path: PathBuf) -> Result<File, ReleaseAgentError>;
}

#[derive(Debug, Clone)]
pub struct MiniSigner {
    secret_key: String,
    password: String,
}

impl MiniSigner {
    pub fn new(secret_key: String, password: String) -> Self {
        Self {
            secret_key,
            password,
        }
    }
}

#[async_trait]
impl ReleaseSigner for MiniSigner {
    fn sign_release(&self, compressed_archived: File, file_path: PathBuf) -> Result<File, ReleaseAgentError> {
        let password = self.password.clone();
        let private_key = SecretKey::from_file(self.secret_key.as_str(), Some(password)).map_err(|_| ReleaseAgentError::SigningError)?;

        let signature = minisign::sign(None, &private_key, compressed_archived, None, None).map_err(|_| ReleaseAgentError::SigningError)?;
        let mut file = File::create(file_path.with_extension("sig")).map_err(|_| ReleaseAgentError::SigningError)?;
        let signature_serialized = signature.to_string();
        file.write_all(signature_serialized.as_bytes()).map_err(|_| ReleaseAgentError::SigningError)?;
        Ok(file)
    }
}
