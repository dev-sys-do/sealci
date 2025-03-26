use std::fmt;

#[derive(Debug)]
pub enum DumpletError {
    DockerError(bollard::errors::Error),
    IoError(std::io::Error),
}

impl fmt::Display for DumpletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DumpletError::DockerError(err) => write!(f, "Docker error: {}", err),
            DumpletError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

// Implement the `From` trait for the `DumpletError` type
impl From<bollard::errors::Error> for DumpletError {
    fn from(err: bollard::errors::Error) -> Self {
        DumpletError::DockerError(err)
    }
}

impl From<std::io::Error> for DumpletError {
    fn from(err: std::io::Error) -> Self {
        DumpletError::IoError(err)
    }
}
