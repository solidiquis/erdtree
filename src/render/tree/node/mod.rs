use crate::{
    fs::inode::Inode,
    icons,
    render::{
        context::Context,
        disk_usage::file_size::{DiskUsage, FileSize},
        styles::get_ls_colors,
        tree::error::Error,
    },
};
use ansi_term::Style;
use ignore::DirEntry;
use lscolors::Style as LS_Style;
use std::{
    borrow::Cow,
    convert::TryFrom,
    ffi::OsStr,
    fmt::{self, Formatter},
    fs::{FileType, Metadata},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[cfg(unix)]
use crate::fs::{
    permissions::{FileMode, SymbolicNotation},
    xattr::ExtendedAttr,
};

/// Ordering and sorting rules for [Node].
pub mod cmp;

/// Concerned with formating [Node]s for display variants.
pub mod display;

/// Styling utilities for a [Node].
pub mod style;

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
    has_xattrs: bool,
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

        #[cfg(unix)] has_xattrs: bool,
    ) -> Self {
        Self {
            dir_entry,
            metadata,
            file_size,
            style,
            symlink_target,
            inode,
            #[cfg(unix)]
            has_xattrs,
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
    pub fn ino(&self) -> Option<u64> {
        self.inode.map(|inode| inode.ino)
    }

    /// Returns the underlying `nlink` of the [`DirEntry`].
    pub fn nlink(&self) -> Option<u64> {
        self.inode.map(|inode| inode.nlink)
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
        self.symlink_target_path().and_then(Path::file_name)
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
    const fn has_xattrs(&self) -> bool {
        self.has_xattrs
    }

    /// Formats the [Node] for the tree presentation.
    pub fn tree_display(&self, f: &mut Formatter, prefix: &str, ctx: &Context) -> fmt::Result {
        self.tree(f, Some(prefix), ctx)
    }

    /// Formats the [Node] for the [`Flat`] presentation.
    ///
    /// [`Flat`]: crate::render::tree::display::Flat
    pub fn flat_display(&self, f: &mut Formatter, ctx: &Context) -> fmt::Result {
        self.flat(f, ctx)
    }

    /// See [`icons::compute`].
    fn compute_icon(&self, no_color: bool) -> Cow<'static, str> {
        if no_color {
            icons::compute(self.dir_entry(), self.symlink_target_path())
        } else {
            icons::compute_with_color(self.dir_entry(), self.symlink_target_path(), self.style)
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

        let style = get_ls_colors().ok().and_then(|ls_colors| {
            ls_colors
                .style_for_path_with_metadata(path, Some(&metadata))
                .map(LS_Style::to_ansi_term_style)
                .or_else(|| Some(Style::default()))
        });

        let file_type = dir_entry.file_type();

        let mut file_size = match file_type {
            Some(ref ft) if ft.is_file() && !ctx.suppress_size => match ctx.disk_usage {
                DiskUsage::Logical => Some(FileSize::logical(&metadata, ctx.unit, ctx.human)),
                DiskUsage::Physical => FileSize::physical(path, &metadata, ctx.unit, ctx.human),
            },
            _ => None,
        };

        if let Some(ref mut fs) = file_size {
            fs.precompute_unpadded_display();
        }

        let inode = Inode::try_from(&metadata).ok();

        #[cfg(unix)]
        let has_xattrs = if ctx.long {
            dir_entry.has_xattrs()
        } else {
            false
        };

        Ok(Self::new(
            dir_entry,
            metadata,
            file_size,
            style,
            link_target,
            inode,
            #[cfg(unix)]
            has_xattrs,
        ))
    }
}
