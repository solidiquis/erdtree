use ignore::DirEntry;
use std::{io, path::Path};
use xattr::XAttrs;

/// Allow extended attributes to be queried directly from the directory entry.
impl ExtendedAttr for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
}

/// Simple trait that allows files to query extended attributes if it exists.
pub trait ExtendedAttr {
    fn path(&self) -> &Path;

    /// Query the extended attribute and return an error if something goes wrong.
    fn try_get_xattrs(&self) -> io::Result<XAttrs> {
        xattr::list(self.path())
    }

    /// Query the extended attribute and return `None` if something goes.
    fn get_xattrs(&self) -> Option<XAttrs> {
        xattr::list(self.path()).ok()
    }
}
