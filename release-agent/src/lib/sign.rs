use sequoia_openpgp::{
    armor::{self, Kind}, cert::CertBuilder, crypto::{KeyPair, Password}, packet::prelude::Packet, parse::Parse, policy::StandardPolicy, serialize::{
        stream::{Armorer, Message, Signer as OpenPgpSigner}, Serialize
    }, Cert
};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use tonic::async_trait;
use tracing::{error, info};

use crate::{core::{self, ReleaseAgentError}, file::AscFile};

#[async_trait]
pub trait ReleaseSigner: Clone + Send + Sync {
    // file_path is the path to the file to be signed
    fn sign_release(
        &self,
        file_path: PathBuf,
    ) -> Result<(core::PublicKey, PathBuf), ReleaseAgentError>;

    fn get_public_key(&self) -> Result<core::PublicKey, ReleaseAgentError>;
}

#[derive(Clone)]
pub struct SequoiaPGPManager {
    key_pair: KeyPair,
    cert: Cert,
    root_public_key: PathBuf,
}

impl SequoiaPGPManager {
    pub fn new(cert_path: PathBuf, passphrase: String) -> Result<Self, ReleaseAgentError> {
        info!("Generating root key pair");
        let (key_pair, cert, root_public_key) = Self::generate_keypair(passphrase, cert_path)?;
        info!("Root key pair generated");
        Ok(Self { key_pair, cert, root_public_key })
    }

    fn generate_keypair(
        passphrase: String,
        path: PathBuf,
    ) -> Result<(KeyPair, Cert, PathBuf), ReleaseAgentError> {
        let now = std::time::SystemTime::now();
        // a year
        let expiration = std::time::Duration::new(60 * 60 * 24 * 365, 0);
        let (cert, _) = CertBuilder::new()
            .add_userid("SealCI Release Agent <release-agent@sealci.dev>")
            .add_signing_subkey()
            .set_creation_time(now)
            .set_validity_period(expiration)
            .set_password(Some(passphrase.clone().into()))
            .generate()
            .map_err(|_| ReleaseAgentError::KeyGenerationError)?;
        info!("Generated key pair");


        let key_pair = cert
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
            .map_err(|_| ReleaseAgentError::KeyDecryptionError)?;
        let cert_path = path.join("sealci-release-agent.asc");

        let output = AscFile{
            path: cert_path.clone(),
        };
        let mut sink = output.create_pgp_safe(false, armor::Kind::PublicKey).map_err(|e| {
            error!("Error creating armored message: {:?}", e);
            ReleaseAgentError::SigningError
        })?;
        cert.serialize(&mut sink).map_err(|e| {
            error!("Error serializing public key: {}", e);
            ReleaseAgentError::SigningError
        })?;

        sink.finalize().map_err(|e| {
            error!("Error finalizing public key: {}", e);
            ReleaseAgentError::SigningError
        })?;


        Ok((key_pair, cert, cert_path))
    }

    fn cert_file_to_keypair(
        cert_path: PathBuf,
        passphrase: String,
    ) -> Result<KeyPair, ReleaseAgentError> {
        let cert = Cert::from_file(cert_path).map_err(|_| ReleaseAgentError::KeyLoadingError)?;
        cert.keys()
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
impl ReleaseSigner for SequoiaPGPManager {
    fn sign_release(
        &self,
        file_path: PathBuf,
    ) -> Result<(core::PublicKey, PathBuf), ReleaseAgentError> {
        let key_pair = self.key_pair.clone();
        let public_key = self.cert.primary_key().key();
        let fingerprint = public_key.fingerprint().to_string();

        let serialized_key_file = File::open(self.root_public_key.clone()).map_err(|e| {
            error!("Error opening public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;
        let serialized_key: String = std::io::read_to_string(serialized_key_file).map_err(|e| {
            error!("Error reading public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;
        let signature_path = PathBuf::from(format!("{}.sig", file_path.display()));
        let signature_file = File::create(signature_path.clone()).map_err(|e| {
            error!("Error creating signature file: {}", e);
            ReleaseAgentError::SigningError
        })?;

        // testing something here

        let signature_message = Message::new(signature_file);
        let armored_message = Armorer::new(signature_message)
            .kind(Kind::Signature)
            .build()
            .map_err(|e| {
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

        let mut compressed_archived =
            OpenOptions::new().read(true).open(file_path).map_err(|e| {
                error!("Error opening file to sign: {}", e);
                ReleaseAgentError::SigningError
            })?;

        std::io::copy(&mut compressed_archived, &mut detached_signer_writer).map_err(|e| {
            error!(
                "Error copying compressed archive to detached signer writer {}",
                e
            );
            ReleaseAgentError::SigningError
        })?;

        detached_signer_writer.finalize().map_err(|_| {
            error!("Error finishing detached signer writer");
            ReleaseAgentError::SigningError
        })?;

        Ok((
            core::PublicKey {
                key_data: serialized_key,
                fingerprint,
            },
            signature_path,
        ))
    }

    fn get_public_key(&self) -> Result<core::PublicKey, ReleaseAgentError> {
        let public_key = self.cert.primary_key().key();
        let fingerprint = public_key.fingerprint().to_string();

        let serialized_key_file = File::open(self.root_public_key.clone()).map_err(|e| {
            error!("Error opening public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;
        let serialized_key: String = std::io::read_to_string(serialized_key_file).map_err(|e| {
            error!("Error reading public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;

        Ok(core::PublicKey {
            key_data: serialized_key,
            fingerprint,
        })
    }
}
