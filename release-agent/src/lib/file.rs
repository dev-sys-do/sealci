use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use tempfile::NamedTempFile;

use sequoia_openpgp::{
    self as openpgp, armor,
    serialize::stream::{Armorer, Message},
};
use tracing::error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AscFile{
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum FileError {
    Io(io::Error),
    OpenPgp,
    FileError,
}

impl AscFile {
    /// Opens the file (or stdout) for writing data that is safe for
    /// non-interactive use because it is an OpenPGP data stream.
    ///
    /// Emitting armored data with the label `armor::Kind::SecretKey`
    /// implicitly configures this output to emit secret keys.
    pub fn create_pgp_safe<'a>(
        &self,
        binary: bool,
        kind: armor::Kind,
    ) -> Result<Message<'a>, FileError> {
        // Allow secrets to be emitted if the armor label says secret
        // key.
        let o = self.clone();
        let sink = o.create()?;

        let mut message = Message::new(sink);
        if !binary {
            message = Armorer::new(message)
                .kind(kind)
                .build()
                .map_err(|_| FileError::OpenPgp)?;
        }
        Ok(message)
    }

    /// Helper function, do not use directly. Instead, use create_or_stdout_safe
    /// or create_or_stdout_unsafe.
    fn create(&self) -> Result<Box<dyn Write + Sync + Send>, FileError> {
        let sink = self._create_sink()?;
        if !cfg!(debug_assertions) {
            // We either expect secrets, or we are in release mode.
            Ok(sink)
        } else {
            // In debug mode, if we don't expect secrets, scan the
            // output for inadvertently leaked secret keys.
            Ok(Box::new(SecretLeakDetector::new(sink)))
        }
    }
    fn _create_sink(&self) -> Result<Box<dyn Write + Sync + Send>, FileError> {
        let path = self.path.clone();
        Ok(Box::new(
            PartFileWriter::create(path).expect("Failed to create output file"),
        ))
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// A writer that writes to a temporary file first, then persists the
/// file under the desired name.
///
/// This has two benefits.  First, consumers only see the file once we
/// are done writing to it, i.e. they don't see a partial file.
///
/// Second, we guarantee not to overwrite the file until the operation
/// is finished.  Therefore, it is safe to use the same file as input
/// and output.
struct PartFileWriter {
    path: PathBuf,
    sink: Option<NamedTempFile>,
}

impl io::Write for PartFileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sink().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sink().unwrap().flush()
    }
}

impl Drop for PartFileWriter {
    fn drop(&mut self) {
        if let Err(e) = self.persist() {
            error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

impl PartFileWriter {
    /// Opens a file for writing.
    ///
    /// The file will be created under a different name in the target
    /// directory, and will only be renamed to `path` once
    /// [`PartFileWriter::persist`] is called or the object is
    /// dropped.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<PartFileWriter, FileError> {
        let path = path.as_ref().to_path_buf();
        let parent = path.parent().ok_or(FileError::FileError)?;
        let file_name = path.file_name().ok_or(FileError::FileError)?;

        let mut sink = tempfile::Builder::new();

        // By default, temporary files are 0x600 on Unix.  But, we
        // rather want created files to respect umask.
        use std::os::unix::fs::PermissionsExt;
        let all_read_write = std::fs::Permissions::from_mode(0o666);

        // The permissions will be masked by the user's umask.
        sink.permissions(all_read_write);

        let sink = sink.prefix(file_name).suffix(".part").tempfile_in(parent).map_err(|_| FileError::FileError)?;

        Ok(PartFileWriter {
            path,
            sink: Some(sink),
        })
    }

    /// Returns a mutable reference to the file, or an error.
    fn sink(&mut self) -> Result<&mut NamedTempFile, FileError> {
        self.sink.as_mut().ok_or(FileError::FileError)
    }

    /// Persists the file under its final name.
    pub fn persist(&mut self) -> Result<(), FileError> {
        if let Some(file) = self.sink.take() {
            file.persist(&self.path).map_err(|_| FileError::FileError)?;
        }
        Ok(())
    }
}

/// A writer that buffers all data, and scans for secret keys on drop.
///
/// This is used to assert that we only write secret keys in places
/// where we expect that.  As this buffers all data, and has a
/// performance impact, we only do this in debug builds.
struct SecretLeakDetector<W: io::Write + Send + Sync> {
    sink: W,
    data: Vec<u8>,
}

impl<W: io::Write + Send + Sync> io::Write for SecretLeakDetector<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.sink.write(buf)?;
        self.data.extend_from_slice(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sink.flush()
    }
}

impl<W: io::Write + Send + Sync> Drop for SecretLeakDetector<W> {
    fn drop(&mut self) {
        let _ = self.detect_leaks();
    }
}

impl<W: io::Write + Send + Sync> SecretLeakDetector<W> {
    /// Creates a shim around `sink` that scans for inadvertently
    /// leaked secret keys.
    fn new(sink: W) -> Self {
        SecretLeakDetector {
            sink,
            data: Vec::with_capacity(4096),
        }
    }

    /// Scans the buffered data for secret keys, panic'ing if one is
    /// found.
    fn detect_leaks(&self) -> Result<(), FileError> {
        use openpgp::Packet;
        use openpgp::parse::{PacketParser, PacketParserResult, Parse};

        let mut ppr = PacketParser::from_bytes(&self.data).map_err(|_| FileError::OpenPgp)?;
        while let PacketParserResult::Some(pp) = ppr {
            match &pp.packet {
                Packet::SecretKey(_) | Packet::SecretSubkey(_) => {
                    panic!("Leaked secret key: {:?}", pp.packet)
                }
                _ => (),
            }
            let (_, next_ppr) = pp.recurse().map_err(|_| FileError::OpenPgp)?;
            ppr = next_ppr;
        }

        Ok(())
    }
}
