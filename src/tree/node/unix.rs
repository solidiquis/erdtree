use crate::fs::{ug::UserGroupInfo, xattr::ExtendedAttr};
use ignore::DirEntry;
use std::convert::From;

/// File attributes that are optionally computed and specific to Unix-like systems.
#[derive(Default)]
pub struct Attrs {
    pub has_xattrs: bool,
    owner: Option<String>,
    group: Option<String>,
}

impl Attrs {
    /// Constructor for [`Attrs`].
    pub fn new(has_xattrs: bool, owner: Option<String>, group: Option<String>) -> Self {
        Self {
            has_xattrs,
            owner,
            group,
        }
    }
}

/// Initializes a [`Attrs`] from a [`DirEntry`].
impl From<&DirEntry> for Attrs {
    fn from(entry: &DirEntry) -> Self {
        let has_xattrs = entry.has_xattrs();

        if let Ok((o, g)) = entry.try_get_owner_and_group() {
            return Self::new(has_xattrs, Some(o), Some(g));
        }

        Self::new(has_xattrs, None, None)
    }
}
