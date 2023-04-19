use std::{convert::TryFrom, fs::Metadata};

/// Represents a file's underlying inode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Inode {
    pub ino: u64,
    pub dev: u64,
    pub nlink: u64,
}

impl Inode {
    /// Initializer for an inode given all the properties that make it unique.
    pub const fn new(ino: u64, dev: u64, nlink: u64) -> Self {
        Self { ino, dev, nlink }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Insufficient information to compute inode")]
pub struct Error;

impl TryFrom<&Metadata> for Inode {
    type Error = Error;

    #[cfg(unix)]
    fn try_from(md: &Metadata) -> Result<Self, Self::Error> {
        use std::os::unix::fs::MetadataExt;

        Ok(Self::new(md.ino(), md.dev(), md.nlink()))
    }

    #[cfg(windows)]
    fn try_from(md: &Metadata) -> Result<Self, Self::Error> {
        use std::os::windows::fs::MetadataExt;

        if let (Some(dev), Some(ino), Some(nlink)) = (
            md.volume_serial_number(),
            md.file_index(),
            md.number_of_links(),
        ) {
            return Ok(Self::new(ino, dev.into(), nlink.into()));
        }

        Err(Error {})
    }

    #[cfg(not(any(unix, windows)))]
    fn try_from(md: &Metadata) -> Result<Self, Self::Error> {
        Err(Error {})
    }
}
