use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt::{self, Display},
    fs::Metadata,
};

/// Represents a file's underlying inode.
#[derive(Debug)]
pub struct Inode {
    ino: u64,
    dev: u64,
    nlink: u64
}

impl Inode {
    /// Initializer for an inode given all the properties that make it unique.
    pub fn new(ino: u64, dev: u64, nlink: u64) -> Self {
        Self { ino, dev, nlink }
    }

    /// Returns a tuple of all the fields of the [Inode].
    pub fn properties(&self) -> (u64, u64, u64) {
        (self.ino, self.dev, self.nlink)
    }
}

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Insufficient information to compute inode")
    }
}

impl StdError for Error {}

impl TryFrom<Metadata> for Inode {
    type Error = Error;

    #[cfg(unix)]
    fn try_from(md: Metadata) -> Result<Self, Self::Error> {
        use std::os::unix::fs::MetadataExt;

        Ok(Self::new(md.ino(), md.dev(), md.nlink()))
    }

    #[cfg(windows)]
    fn try_from(md: Metadata) -> Result<Self, Self::Error> {
        use std::os::windows::fs::MetadataExt;

        if let (Some(ino), Some(dev), Some(nlinks)) = (
            metadata.file_index(),
            metadata.volume_serial_number(),
            metadata.number_of_links(),
        ) {
            return Ok(Self::new(md, dev, nlink));
        }

        Err(Error {})
    }

    #[cfg(not(any(unix, windows)))]
    fn try_from(md: Metadata) -> Result<Self, Self::Error> {
        Err(Error {})
    }
}
