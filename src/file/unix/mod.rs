use ignore::DirEntry;
use std::{convert::From, fs::Metadata};

/// Unix file permissions in symbolic and octal notation for presentation.
#[cfg(unix)]
pub mod permissions;

/// Concerned with determining group and owner of file.
#[cfg(unix)]
pub mod ug;
use ug::UserGroupInfo;

/// Determining whether or not a file has extended attributes.
#[cfg(unix)]
pub mod xattr;
use xattr::ExtendedAttr;

/// File attributes that are optionally computed and specific to Unix-like systems.
#[derive(Debug, Default)]
pub struct Attrs {
    pub has_xattrs: bool,
    owner: Option<String>,
    group: Option<String>,
}

impl Attrs {
    /// Constructor for [`Attrs`].
    pub const fn new(has_xattrs: bool, owner: Option<String>, group: Option<String>) -> Self {
        Self {
            has_xattrs,
            owner,
            group,
        }
    }

    /// Returns the file owner.
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }

    /// Returns the file's group.
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
}

/// Initializes a [`Attrs`] from a [`DirEntry`].
impl From<(&Metadata, &DirEntry)> for Attrs {
    fn from((md, entry): (&Metadata, &DirEntry)) -> Self {
        let has_xattrs = entry.has_xattrs();

        if let Ok((o, g)) = md.try_get_owner_and_group() {
            return Self::new(has_xattrs, Some(o), Some(g));
        }

        Self::new(has_xattrs, None, None)
    }
}
