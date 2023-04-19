use ignore::DirEntry;
use std::{os::unix::ffi::OsStrExt, path::Path, ptr};

/// Allow extended attributes to be queried directly from the directory entry.
impl ExtendedAttr for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
}

/// Simple trait that allows files to query extended attributes if it exists.
pub trait ExtendedAttr {
    fn path(&self) -> &Path;

    /// Queries the filesystem to check if there exists extended attributes for the implementor's
    /// path.
    fn has_xattrs(&self) -> bool {
        unsafe { has_xattrs(self.path()) }
    }
}

/// Checks to see if a directory entry referred to by `path` has extended attributes.
unsafe fn has_xattrs(path: &Path) -> bool {
    use libc::{c_char, listxattr};

    let path_ptr = {
        let slice = path.as_os_str().as_bytes();
        let slice_ptr = slice.as_ptr();
        slice_ptr.cast::<c_char>()
    };

    #[cfg(target_os = "linux")]
    return 0 < listxattr(path_ptr, ptr::null_mut::<c_char>(), 0);

    #[cfg(target_os = "macos")]
    return 0 < listxattr(path_ptr, ptr::null_mut::<c_char>(), 0, 0);
}
