use crate::{
    fs::inode::Inode,
    icons::{self, icon_from_ext, icon_from_file_name, icon_from_file_type},
    render::{
        context::Context,
        disk_usage::file_size::{DiskUsage, FileSize},
        styles::get_ls_colors,
        tree::error::Error,
    },
};
use ansi_term::Color;
use ansi_term::Style;
use ignore::DirEntry;
use layout::SizeLocation;
use lscolors::Style as LS_Style;
use std::{
    borrow::Cow,
    convert::TryFrom,
    ffi::OsStr,
    fmt::{self, Formatter},
    fs::{FileType, Metadata},
    path::{Path, PathBuf},
};

/// Ordering and sorting rules for [Node].
pub mod cmp;

/// For determining orientation of disk usage information for [Node].
mod layout;

/// A node of [`Tree`] that can be created from a [DirEntry]. Any filesystem I/O and
/// relevant system calls are expected to complete after initialization. A `Node` when `Display`ed
/// uses ANSI colors determined by the file-type and [`LS_COLORS`].
///
/// [`Tree`]: super::Tree
/// [`LS_COLORS`]: crate::render::styles::LS_COLORS
#[derive(Debug)]
pub struct Node {
    pub file_size: Option<FileSize>,
    icon: String,
    style: Style,
    symlink_target: Option<PathBuf>,
    dir_entry: DirEntry,
    metadata: Metadata,
}

impl Node {
    /// Initializes a new [Node].
    pub const fn new(
        file_size: Option<FileSize>,
        icon: String,
        style: Style,
        symlink_target: Option<PathBuf>,
        dir_entry: DirEntry,
        metadata: Metadata,
    ) -> Self {
        Self {
            file_size,
            icon,
            style,
            symlink_target,
            dir_entry,
            metadata,
        }
    }

    /// Returns a reference to `file_name`. If file is a symlink then `file_name` is the name of
    /// the symlink not the target.
    pub fn file_name(&self) -> &OsStr {
        &self.dir_entry.file_name()
    }

    /// Get depth level of [Node].
    pub fn depth(&self) -> usize {
        self.dir_entry.depth()
    }

    /// Gets the underlying [Inode] of the entry.
    pub fn inode(&self) -> Option<Inode> {
        Inode::try_from(&self.metadata).ok()
    }

    /// Converts `OsStr` to `String`; if fails does a lossy conversion replacing non-Unicode
    /// sequences with Unicode replacement scalar values.
    pub fn file_name_lossy(&self) -> Cow<'_, str> {
        self.file_name()
            .to_str()
            .map_or_else(|| self.file_name().to_string_lossy(), Cow::from)
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

    /// Returns reference to underlying [FileType].
    pub fn file_type(&self) -> Option<FileType> {
        self.dir_entry.file_type()
    }

    /// Returns the path to the [Node]'s parent, if any.
    pub fn parent_path(&self) -> Option<&Path> {
        self.path().parent()
    }

    /// Returns a reference to `path`.
    pub fn path(&self) -> &Path {
        &self.dir_entry.path()
    }

    /// Gets 'file_size'.
    pub const fn file_size(&self) -> Option<&FileSize> {
        self.file_size.as_ref()
    }

    /// Sets `file_size`.
    pub fn set_file_size(&mut self, size: FileSize) {
        self.file_size = Some(size);
    }

    /// Sets 'style'.
    pub const fn style(&self) -> &Style {
        &self.style
    }

    /// Grabs a reference to `icon`.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Stylizes input, `entity` based on [`LS_COLORS`]
    ///
    /// [`LS_COLORS`]: crate::render::styles::LS_COLORS
    fn stylize(&self, entity: &str) -> String {
        self.style().foreground.map_or_else(
            || entity.to_string(),
            |fg| fg.bold().paint(entity).to_string(),
        )
    }

    /// Stylizes symlink name for display.
    fn stylize_link_name(&self) -> Option<String> {
        self.symlink_target_file_name().map(|name| {
            let file_name = self.file_name_lossy();
            let styled_name = self.stylize(&file_name);
            let target_name = Color::Red.paint(format!("\u{2192} {}", name.to_string_lossy()));
            format!("{styled_name} {target_name}")
        })
    }

    /// General method for printing a `Node`. The `Display` (and `ToString`) traits are not used,
    /// to give more control over the output.
    ///
    /// Format a node for display with size on the right.
    ///
    /// Example:
    /// `| Some Directory (12.3 KiB)`
    ///
    ///
    /// Format a node for display with size on the left.
    ///
    /// Example:
    /// `  1.23 MiB | Some File`
    ///
    /// Note the two spaces to the left of the first character of the number -- even if never used,
    /// numbers are padded to 3 digits to the left of the decimal (and ctx.scale digits after)
    pub fn display(&self, f: &mut Formatter, prefix: &str, ctx: &Context) -> fmt::Result {
        let size_loc = SizeLocation::from(ctx);

        let size = self.file_size().map_or_else(
            || size_loc.default_string(ctx),
            |size| size_loc.format(size),
        );

        let size_padding = if size.is_empty() {
            String::new()
        } else {
            String::from(" ")
        };

        let icon = self.icon();

        let icon_padding = if icon.len() > 1 { icon.len() - 1 } else { 0 };

        let styled_name = self.stylize_link_name().unwrap_or_else(|| {
            let file_name = self.file_name_lossy();
            self.stylize(&file_name)
        });

        match size_loc {
            SizeLocation::Right => {
                write!(
                    f,
                    "{prefix}{icon:<icon_padding$}{styled_name}{size_padding}{size}"
                )
            }
            SizeLocation::Left => {
                write!(f, "{size} {prefix}{icon:<icon_padding$}{styled_name}")
            }
        }
    }

    /// Unix file identifiers that you'd find in the `ls -l` command.
    #[cfg(unix)]
    pub fn file_type_identifier(&self) -> Option<&str> {
        use std::os::unix::fs::FileTypeExt;

        let file_type = self.file_type()?;

        let iden = if file_type.is_dir() {
            "d"
        } else if file_type.is_file() {
            "-"
        } else if file_type.is_symlink() {
            "l"
        } else if file_type.is_fifo() {
            "p"
        } else if file_type.is_socket() {
            "s"
        } else if file_type.is_char_device() {
            "c"
        } else if file_type.is_block_device() {
            "b"
        } else {
            return None;
        };

        Some(iden)
    }

    /// File identifiers.
    #[cfg(not(unix))]
    pub fn file_type_identifier(&self) -> Option<&str> {
        let file_type = self.file_type()?;

        let iden = if file_type.is_dir() {
            "d"
        } else if file_type.is_file() {
            "-"
        } else if file_type.is_symlink() {
            "l"
        } else {
            return None;
        };

        Some(iden)
    }

    /// Tries to compute which icon to use from [FileType]. Directories and links for example have
    /// special icons based on file-type as opposed to extension.
    fn icon_from_file_type(file_type: &FileType) -> Option<&str> {
        icon_from_file_type(file_type)
    }

    /// Tries to compute which icon to use from file-extension provided the path.
    fn icon_from_path(path: &Path) -> Option<&str> {
        path.extension().and_then(icon_from_ext)
    }

    /// Tries to compute which icon to use from the provided file-name. This is relevant to special
    /// files such as `.gitignore`, `LICENSE`, and so on.
    fn icon_from_file_name(file_name: &OsStr) -> Option<&str> {
        icon_from_file_name(file_name)
    }
}

impl TryFrom<(DirEntry, &Context)> for Node {
    type Error = Error;

    fn try_from(data: (DirEntry, &Context)) -> Result<Self, Error> {
        let (dir_entry, ctx) = data;

        let path = dir_entry.path();

        let symlink_target = crate::fs::symlink_target(&dir_entry);

        let metadata = dir_entry.metadata()?;

        let style = get_ls_colors()
            .style_for_path_with_metadata(path, Some(&metadata))
            .map(LS_Style::to_ansi_term_style)
            .unwrap_or_default();

        let file_type = dir_entry.file_type();

        let file_size = match file_type {
            Some(ref ft) if ft.is_file() && !ctx.suppress_size => match ctx.disk_usage {
                DiskUsage::Logical => Some(FileSize::logical(&metadata, ctx.prefix, ctx.scale)),
                DiskUsage::Physical => FileSize::physical(path, &metadata, ctx.prefix, ctx.scale),
            },
            _ => None,
        };

        let icon = if ctx.icons {
            let plain_icon = file_type
                .as_ref()
                .and_then(Self::icon_from_file_type)
                .or_else(|| {
                    symlink_target.as_ref().map_or_else(
                        || Self::icon_from_path(path),
                        |target| Self::icon_from_path(target),
                    )
                })
                .or_else(|| Self::icon_from_file_name(dir_entry.file_name()))
                .unwrap_or_else(icons::get_default_icon);

            style.foreground.map_or_else(
                || String::from(plain_icon),
                |fg| fg.bold().paint(plain_icon).to_string(),
            )
        } else {
            String::new()
        };

        Ok(Self::new(
            file_size,
            icon,
            style,
            symlink_target,
            dir_entry,
            metadata,
        ))
    }
}
