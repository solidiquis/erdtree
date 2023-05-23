use ignore::DirEntry;
use std::{fs, path::PathBuf};

/// Operations pertaining to underlying inodes of files.
pub mod inode;

/// Unix file permissions.
#[cfg(unix)]
pub mod permissions;

/// Determining whether or not a file has extended attributes.
#[cfg(unix)]
pub mod xattr;

/// Concerned with determining group and owner of file.
pub mod ug;

/// Returns the path to the target of the soft link. Returns `None` if provided `dir_entry` isn't a
/// symlink.
pub fn symlink_target(dir_entry: &DirEntry) -> Option<PathBuf> {
    dir_entry
        .path_is_symlink()
        .then(|| fs::read_link(dir_entry.path()))
        .transpose()
        .ok()
        .flatten()
}
