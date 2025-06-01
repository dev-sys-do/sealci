use std::error::Error;
use std::fs::File;

///! A trait for signing releases.
/// This trait is used to sign a release archive, such as a tarball or zip file,
/// The return file is the signed archive, which can be used for distribution.
/// The private key is used to sign the archive.
pub trait ReleaseSigner: Clone + Send + Sync {
    fn sign_release(&self, compressed_archived: File) -> Result<File, Box<dyn Error>>;
}
