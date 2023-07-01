use crate::{
    context::Context,
    disk_usage::file_size::{byte, line_count, word_count, DiskUsage, FileSize},
    fs::inode::Inode,
    icons,
    styles::get_ls_colors,
    tree::error::Error,
};
use ansi_term::Style;
use ignore::DirEntry;
use lscolors::Style as LS_Style;
use std::{
    borrow::Cow,
    convert::TryFrom,
    ffi::OsStr,
    fs::{FileType, Metadata},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[cfg(unix)]
use crate::{
    disk_usage::file_size::block,
    fs::permissions::{FileMode, SymbolicNotation},
};

/// Ordering and sorting rules for [Node].
pub mod cmp;

/// File attributes specific to Unix systems.
#[cfg(unix)]
pub mod unix;

/// A node of [`Tree`] that can be created from a [`DirEntry`]. Any filesystem I/O and
/// relevant system calls are expected to complete after initialization. A `Node` when `Display`ed
/// uses ANSI colors determined by the file-type and `LS_COLORS`.
///
/// [`Tree`]: super::Tree
pub struct Node {
    dir_entry: DirEntry,
    metadata: Metadata,
    file_size: Option<FileSize>,
    style: Option<Style>,
    symlink_target: Option<PathBuf>,
    inode: Option<Inode>,

    #[cfg(unix)]
    unix_attrs: unix::Attrs,
}

impl Node {
    /// Initializes a new [Node].
    pub const fn new(
        dir_entry: DirEntry,
        metadata: Metadata,
        file_size: Option<FileSize>,
        style: Option<Style>,
        symlink_target: Option<PathBuf>,
        inode: Option<Inode>,
        #[cfg(unix)] unix_attrs: unix::Attrs,
    ) -> Self {
        Self {
            dir_entry,
            metadata,
            file_size,
            style,
            symlink_target,
            inode,
            #[cfg(unix)]
            unix_attrs,
        }
    }

    /// Returns a reference to `file_name`. If file is a symlink then `file_name` is the name of
    /// the symlink not the target.
    pub fn file_name(&self) -> &OsStr {
        self.dir_entry.file_name()
    }

    pub const fn dir_entry(&self) -> &DirEntry {
        &self.dir_entry
    }

    /// Get depth level of [Node].
    pub fn depth(&self) -> usize {
        self.dir_entry.depth()
    }

    /// Gets the number of blocks used by the underlying [`DirEntry`]. Returns `None` in the case of
    /// no blocks allocated like in the case of directories.
    #[cfg(unix)]
    pub fn blocks(&self) -> Option<u64> {
        use std::os::unix::fs::MetadataExt;

        let blocks = self.metadata.blocks();

        if blocks == 0 {
            None
        } else {
            Some(blocks)
        }
    }

    /// Timestamp of when file was last modified.
    pub fn modified(&self) -> Option<SystemTime> {
        self.metadata.modified().ok()
    }

    /// Timestamp of when file was created.
    pub fn created(&self) -> Option<SystemTime> {
        self.metadata.created().ok()
    }

    /// Timestamp of when file was last accessed.
    pub fn accessed(&self) -> Option<SystemTime> {
        self.metadata.accessed().ok()
    }

    /// Gets the underlying [Inode] of the entry.
    pub const fn inode(&self) -> Option<Inode> {
        self.inode
    }

    /// Returns the underlying `ino` of the [`DirEntry`].
    #[cfg(unix)]
    pub const fn ino(&self) -> Option<u64> {
        if let Some(inode) = self.inode {
            Some(inode.ino)
        } else {
            None
        }
    }

    /// Returns the underlying `nlink` of the [`DirEntry`].
    #[cfg(unix)]
    pub const fn nlink(&self) -> Option<u64> {
        if let Some(inode) = self.inode {
            Some(inode.nlink)
        } else {
            None
        }
    }

    /// Returns `true` if node is a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type().map_or(false, |ft| ft.is_dir())
    }

    /// Is the Node a symlink.
    pub const fn is_symlink(&self) -> bool {
        self.symlink_target.is_some()
    }

    /// Path to symlink target.
    pub fn symlink_target_path(&self) -> Option<&Path> {
        self.symlink_target.as_deref()
    }

    /// Returns the file name of the symlink target if [Node] represents a symlink.
    pub fn symlink_target_file_name(&self) -> Option<&OsStr> {
        self.symlink_target_path().map(Path::as_os_str)
    }

    /// Returns reference to underlying [`FileType`].
    pub fn file_type(&self) -> Option<FileType> {
        self.dir_entry.file_type()
    }

    /// Returns the path to the [Node]'s parent, if any.
    pub fn parent_path(&self) -> Option<&Path> {
        self.path().parent()
    }

    /// Returns a reference to `path`. If the underlying [`DirEntry`] is a symlink then the path of
    /// the symlink shall be returned.
    pub fn path(&self) -> &Path {
        self.dir_entry.path()
    }

    /// Gets '`file_size`'.
    pub const fn file_size(&self) -> Option<&FileSize> {
        self.file_size.as_ref()
    }

    /// Sets `file_size`.
    pub fn set_file_size(&mut self, size: FileSize) {
        self.file_size = Some(size);
    }

    /// Attempts to return an instance of [`FileMode`] for the display of symbolic permissions.
    #[cfg(unix)]
    pub fn mode(&self) -> Result<FileMode, Error> {
        let permissions = self.metadata.permissions();
        let file_mode = permissions.try_mode_symbolic_notation()?;
        Ok(file_mode)
    }

    /// Whether or not [Node] has extended attributes.
    #[cfg(unix)]
    pub const fn has_xattrs(&self) -> bool {
        self.unix_attrs.has_xattrs
    }

    /// Returns the owner of the [`Node`].
    #[cfg(unix)]
    pub fn owner(&self) -> Option<&str> {
        self.unix_attrs.owner()
    }

    /// Returns the group of the [`Node`].
    #[cfg(unix)]
    pub fn group(&self) -> Option<&str> {
        self.unix_attrs.group()
    }

    /// Getter for [Node]'s style field.
    pub const fn style(&self) -> Option<Style> {
        self.style
    }

    /// See [`crate::icons::fs::compute`].
    pub fn compute_icon(&self, no_color: bool) -> Cow<'static, str> {
        if no_color {
            icons::fs::compute(self.dir_entry(), self.symlink_target_path())
        } else {
            icons::fs::compute_with_color(self.dir_entry(), self.symlink_target_path(), self.style)
        }
    }
}

impl TryFrom<(DirEntry, &Context)> for Node {
    type Error = Error;

    fn try_from(data: (DirEntry, &Context)) -> Result<Self, Error> {
        let (dir_entry, ctx) = data;

        let path = dir_entry.path();

        let link_target = crate::fs::symlink_target(&dir_entry);

        let metadata = dir_entry.metadata()?;

        let style = get_ls_colors().ok().map(|ls_colors| {
            ls_colors
                .style_for_path_with_metadata(path, Some(&metadata))
                .map_or_else(Style::default, LS_Style::to_ansi_term_style)
        });

        let file_type = dir_entry.file_type();

        let file_size = match file_type {
            Some(ref ft)
                if !ctx.suppress_size && (ft.is_file() || ft.is_symlink() && !ctx.follow) =>
            {
                match ctx.disk_usage {
                    DiskUsage::Logical => {
                        let metric = byte::Metric::init_logical(&metadata, ctx.unit, ctx.human);
                        Some(FileSize::Byte(metric))
                    },
                    DiskUsage::Physical => {
                        let metric =
                            byte::Metric::init_physical(path, &metadata, ctx.unit, ctx.human);
                        Some(FileSize::Byte(metric))
                    },
                    DiskUsage::Line => {
                        let metric = line_count::Metric::init(path);
                        metric.map(FileSize::Line)
                    },
                    DiskUsage::Word => {
                        let metric = word_count::Metric::init(path);
                        metric.map(FileSize::Word)
                    },

                    #[cfg(unix)]
                    DiskUsage::Block => {
                        let metric = block::Metric::init(&metadata);
                        Some(FileSize::Block(metric))
                    },
                }
            },
            _ => None,
        };

        let inode = Inode::try_from(&metadata).ok();

        #[cfg(unix)]
        let unix_attrs = if ctx.long {
            unix::Attrs::from((&metadata, &dir_entry))
        } else {
            unix::Attrs::default()
        };

        Ok(Self::new(
            dir_entry,
            metadata,
            file_size,
            style,
            link_target,
            inode,
            #[cfg(unix)]
            unix_attrs,
        ))
    }
}
