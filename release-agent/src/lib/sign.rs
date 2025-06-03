use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::PathBuf};
use std::sync::RwLock;
use sequoia_openpgp::{
    Cert,
    armor::Kind,
    crypto::{KeyPair, Password},
    parse::Parse,
    policy::StandardPolicy,
    serialize::stream::{Armorer, Message, Signer as OpenPgpSigner},
};

use tracing::error;
use minisign::SecretKey;
use tonic::async_trait;

use tracing::info;
use crate::core::ReleaseAgentError;

///! A trait for signing releases.
/// This trait is used to sign a release archive, such as a tarball or zip file,
/// The return file is the signed archive, which can be used for distribution.
/// The private key is used to sign the archive.
#[async_trait]
pub trait ReleaseSigner: Clone + Send + Sync {
    // file_path is the path to the file to be signed
    fn sign_release(&self, file_path: PathBuf) -> Result<PathBuf, ReleaseAgentError>;
}

#[derive(Clone)]
pub struct SequoiaPGPSigner {
    key_pair: KeyPair,
}

impl SequoiaPGPSigner {
    pub fn new(cert_path: PathBuf, passphrase: String) -> Result<Self, ReleaseAgentError> {
        let key_pair_guard = Self::cert_file_to_keypair(cert_path, passphrase)?;
        Ok(Self {
            key_pair: key_pair_guard.clone(),
        })
    }

    fn cert_file_to_keypair(cert_path: PathBuf, passphrase: String) -> Result<KeyPair, ReleaseAgentError> {
        let cert = Cert::from_file(cert_path).map_err(|_| ReleaseAgentError::KeyLoadingError)?;
        cert
            .keys()
            .with_policy(&StandardPolicy::new(), None)
            .alive()
            .revoked(false)
            .for_signing()
            .secret()
            .next()
            .ok_or(ReleaseAgentError::KeyNotFoundError)?
            .key()
            .clone()
            .decrypt_secret(&Password::from(passphrase))
            .map_err(|_| ReleaseAgentError::KeyDecryptionError)?
            .into_keypair()
            .map_err(|_| ReleaseAgentError::KeyDecryptionError)
    }

}

#[async_trait]
impl ReleaseSigner for SequoiaPGPSigner {
    fn sign_release(&self,file_path: PathBuf) -> Result<PathBuf, ReleaseAgentError> {
        let key_pair = self.key_pair.clone();
        let signature_path = PathBuf::from(format!("{}.sig", file_path.display()));
        let signature_file = File::create(signature_path.clone()).map_err(|e| {
            error!("Error creating signature file: {}", e);
            ReleaseAgentError::SigningError
        })?;

        let signature_message = Message::new(signature_file);
        let armored_message = Armorer::new(signature_message).kind(Kind::Signature).build().map_err(|e| {
            error!("Error creating armored message: {}", e);
            ReleaseAgentError::SigningError
        })?;

        let signer = OpenPgpSigner::new(armored_message, key_pair.clone()).map_err(|_| {
            error!("Error creating OpenPGP signer");
            ReleaseAgentError::SigningError
        })?;

        let mut detached_signer_writer = signer.detached().build().map_err(|_| {
            error!("Error creating detached signer writer");
            ReleaseAgentError::SigningError
        })?;

        let mut compressed_archived = OpenOptions::new()
            .read(true)
            .open(file_path)
            .map_err(|e| {
                error!("Error opening file to sign: {}", e);
                ReleaseAgentError::SigningError
            })?;

        std::io::copy(&mut compressed_archived, &mut detached_signer_writer).map_err(|e| {
            error!("Error copying compressed archive to detached signer writer {}", e);
            ReleaseAgentError::SigningError
        })?;

        detached_signer_writer.finalize().map_err(|_| {
            error!("Error finishing detached signer writer");
            ReleaseAgentError::SigningError
        })?;

        Ok(signature_path)
    }
}
