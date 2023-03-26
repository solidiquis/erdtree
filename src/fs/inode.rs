use std::{convert::TryFrom, fs::Metadata};

/// Represents a file's underlying inode.
#[derive(Debug)]
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

    /// Returns a tuple fields of the [Inode] that mark is unique.
    pub const fn properties(&self) -> (u64, u64) {
        (self.ino, self.dev)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Insufficient information to compute inode")]
pub struct Error;

impl TryFrom<Metadata> for Inode {
    type Error = Error;

    #[cfg(unix)]
    fn try_from(md: Metadata) -> Result<Self, Self::Error> {
        use std::os::unix::fs::MetadataExt;

        Ok(Self::new(md.ino(), md.dev(), md.nlink()))
    }

    #[cfg(not(any(unix)))]
    fn try_from(md: Metadata) -> Result<Self, Self::Error> {
        Err(Error {})
    }
}
