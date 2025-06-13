use sequoia_openpgp::{
    armor::{self, Kind}, cert::{CertBuilder},crypto::{KeyPair, Password}, parse::Parse, policy::StandardPolicy, serialize::{
        stream::{Armorer, Message, Signer as OpenPgpSigner}, Serialize
    }, types::{KeyFlags}, Cert
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

    fn clean_release(&self, path: PathBuf) -> Result<(), ReleaseAgentError>;

    fn get_public_key(&self) -> Result<core::PublicKey, ReleaseAgentError>;
}

#[derive(Clone)]
pub struct SequoiaPGPManager {
    private_cert_path: PathBuf,
    public_cert_path: PathBuf,
    passphrase: String,
}

impl SequoiaPGPManager {
    pub fn new(cert_path: PathBuf, passphrase: String) -> Result<Self, ReleaseAgentError> {
        let private_key_path = cert_path.join("sealci-release-agent-private.asc");
        let public_key_path = cert_path.join("sealci-release-agent-public.asc");
        
        // Check if both keys already exist
        let (private_path, public_path) = if private_key_path.exists() && public_key_path.exists() {
            (private_key_path, public_key_path)
        } else {
            // Generate new keys - returns tuple (private_path, public_path)
            Self::generate_root_certificate(passphrase.clone(), cert_path)?
        };

        Ok(Self {
            private_cert_path: private_path, 
            public_cert_path: public_path,
            passphrase
        })
    }

    fn generate_root_certificate(
        passphrase: String,
        path: PathBuf,
    ) -> Result<(PathBuf, PathBuf), ReleaseAgentError> { // Change return type
        let now = std::time::SystemTime::now();
        // a year
        let expiration = std::time::Duration::new(60 * 60 * 24 * 365, 0);
        let (cert, _) = CertBuilder::new()
            .add_userid("SealCI Release Agent <release-agent@sealci.dev>")
            .add_subkey(KeyFlags::empty().set_signing().set_split_key(), expiration, None)
            .set_creation_time(now)
            .set_validity_period(expiration)
            .set_password(Some(passphrase.clone().into()))
            .generate()
            .map_err(|_| ReleaseAgentError::KeyGenerationError)?;

        info!("Generated root certificate for SealCI Release Agent");

        // Save private key (with secrets)
        let private_cert_path = path.join("sealci-release-agent-private.asc");
        let private_output = AscFile {
            path: private_cert_path.clone(),
        };
        let mut private_sink = private_output.create_pgp_safe(false, armor::Kind::SecretKey).map_err(|e| {
            error!("Error creating armored private key: {:?}", e);
            ReleaseAgentError::SigningError
        })?;
        cert.as_tsk().serialize(&mut private_sink).map_err(|e| {
            error!("Error serializing private key: {}", e);
            ReleaseAgentError::SigningError
        })?;
        private_sink.finalize().map_err(|e| {
            error!("Error finalizing private key: {}", e);
            ReleaseAgentError::SigningError
        })?;

        // Save public key (without secrets)
        let public_cert_path = path.join("sealci-release-agent-public.asc");
        let public_output = AscFile {
            path: public_cert_path.clone(),
        };
        let mut public_sink = public_output.create_pgp_safe(false, armor::Kind::PublicKey).map_err(|e| {
            error!("Error creating armored public key: {:?}", e);
            ReleaseAgentError::SigningError
        })?;
        cert.serialize(&mut public_sink).map_err(|e| {
            error!("Error serializing public key: {}", e);
            ReleaseAgentError::SigningError
        })?;
        public_sink.finalize().map_err(|e| {
            error!("Error finalizing public key: {}", e);
            ReleaseAgentError::SigningError
        })?;

        Ok((private_cert_path, public_cert_path)) // Return both paths
    }
    fn load_keypair(
        &self,
        cert_path: PathBuf,
    ) -> Result<(KeyPair, Cert, PathBuf), ReleaseAgentError> {

        let cert = Cert::from_file(cert_path.clone()).map_err(|_| ReleaseAgentError::KeyLoadingError)?;
        let policy = StandardPolicy::new();
        let passphrase = &self.passphrase;

        let subkeys = cert
            .keys()
            .subkeys()
            .with_policy(&policy, None)
            .alive()
            .revoked(false)
            .for_signing()
            .secret();

        let count = subkeys.count();
        info!("Nombre de sous-clés trouvées : {}", count);

        let key_pair = cert
            .keys()
            .subkeys()
            .with_policy(&policy, None)
            .alive()
            .revoked(false)
            .for_signing()
            .secret()
            .next()
            .ok_or(ReleaseAgentError::KeyNotFoundError)?
            .key()
            .clone()
            .decrypt_secret(&Password::from(passphrase.clone())) // Use the actual passphrase
            .map_err(|_| ReleaseAgentError::KeyDecryptionError)?
            .into_keypair()
            .map_err(|_| ReleaseAgentError::KeyDecryptionError)?;


        Ok((key_pair, cert, cert_path))
    }

    pub fn get_pub_cert(&self) -> Result<Cert, ReleaseAgentError> {
        let cert = Cert::from_file(self.public_cert_path.clone()).map_err(|_| ReleaseAgentError::KeyLoadingError)?;
        Ok(cert)
    }
}

#[async_trait]
impl ReleaseSigner for SequoiaPGPManager {
    fn sign_release(
        &self,
        file_path: PathBuf,
    ) -> Result<(core::PublicKey, PathBuf), ReleaseAgentError> {
        info!("Signing release at {}", file_path.display());
        info!("Using private certificate at {}", self.private_cert_path.display());
        
        // Load keypair from private key file
        let (key_pair, _, _) = Self::load_keypair(&self, self.private_cert_path.clone())?;
        let public_key = key_pair.public();
        let fingerprint = public_key.fingerprint().to_string();

        // Read the PUBLIC key file for returning
        let serialized_key_file = File::open(&self.public_cert_path).map_err(|e| {
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

        let mut compressed_archived = OpenOptions::new().read(true).open(file_path).map_err(|e| {
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

        Ok((
            core::PublicKey {
                key_data: serialized_key, // This is now the PUBLIC key only
                fingerprint,
            },
            signature_path,
        ))
    }


    fn get_public_key(&self) -> Result<core::PublicKey, ReleaseAgentError> {
        let (key_pair, _, _) = Self::load_keypair(&self, self.public_cert_path.clone())?;
        let public_key = key_pair.public();
        let fingerprint = public_key.fingerprint().to_string();
        let serialized_key_file = File::open(self.public_cert_path.clone()).map_err(|e| {
            error!("Error opening subkey public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;
        let serialized_key: String = std::io::read_to_string(serialized_key_file).map_err(|e| {
            error!("Error reading subkey public key file: {}", e);
            ReleaseAgentError::SigningError
        })?;
        info!("Public key fingerprint: {}", fingerprint);
        info!("Public key data: {}", serialized_key);
        if serialized_key.is_empty() {
            return Err(ReleaseAgentError::SigningError);
        }
        if fingerprint.is_empty() {
            return Err(ReleaseAgentError::SigningError);
        }

        Ok(core::PublicKey {
            key_data: serialized_key,
            fingerprint,
        })
    }

    fn clean_release(&self, path: PathBuf) -> Result<(), ReleaseAgentError> {
        std::fs::remove_file(path.clone()).map_err(|e| {
            error!("Error removing folder: {}, {}", e, path.display());
            ReleaseAgentError::GitRepositoryNotFound
        })?;
        Ok(())
    }
}
