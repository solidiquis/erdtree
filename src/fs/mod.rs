use ignore::DirEntry;
use std::{fs, path::PathBuf};

/// Operations pertaining to underlying inodes of files.
pub mod inode;

/// Unix file permissions.
#[cfg(unix)]
pub mod permissions;

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
