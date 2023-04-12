use crate::{
    fs::inode::Inode,
    icons::{self, get_default_icon, icon_from_ext, icon_from_file_name, icon_from_file_type},
    render::{
        context::Context,
        disk_usage::file_size::{DiskUsage, FileSize},
        styles::get_ls_colors,
        tree::error::Error,
    },
    tty,
};
use ansi_term::{ANSIGenericString, Color, Style};
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
/// uses ANSI colors determined by the file-type and `LS_COLORS`.
///
/// [`Tree`]: super::Tree
#[derive(Debug)]
pub struct Node {
    dir_entry: DirEntry,
    metadata: Metadata,
    file_size: Option<FileSize>,
    style: Option<Style>,
    icon: Option<Cow<'static, str>>,
    symlink_target: Option<PathBuf>,
}

impl Node {
    /// Initializes a new [Node].
    pub const fn new(
        dir_entry: DirEntry,
        metadata: Metadata,
        file_size: Option<FileSize>,
        style: Option<Style>,
        icon: Option<Cow<'static, str>>,
        symlink_target: Option<PathBuf>,
    ) -> Self {
        Self {
            dir_entry,
            metadata,
            file_size,
            style,
            icon,
            symlink_target,
        }
    }

    /// Returns a reference to `file_name`. If file is a symlink then `file_name` is the name of
    /// the symlink not the target.
    pub fn file_name(&self) -> &OsStr {
        self.dir_entry.file_name()
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
        self.dir_entry.path()
    }

    /// Gets 'file_size'.
    pub const fn file_size(&self) -> Option<&FileSize> {
        self.file_size.as_ref()
    }

    /// Sets `file_size`.
    pub fn set_file_size(&mut self, size: FileSize) {
        self.file_size = Some(size);
    }

    /// Grabs a reference to `icon`.
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Stylizes input, `entity` based on `LS_COLORS`. If `style` is `None` then the entity is
    /// returned unmodified.
    fn stylize<'a>(&self, entity: Cow<'a, str>) -> Cow<'a, str> {
        if let Some(Style {
            foreground: Some(ref fg),
            ..
        }) = self.style
        {
            Cow::from(fg.bold().paint(entity).to_string())
        } else {
            entity
        }
    }

    /// Stylizes symlink name for display.
    fn stylize_link_name(&self) -> Option<Cow<'_, str>> {
        self.symlink_target_file_name().map(|name| {
            let file_name = self.file_name_lossy();
            let styled_name = self.stylize(file_name);
            let target_name = Color::Red.paint(format!("\u{2192} {}", name.to_string_lossy()));
            Cow::from(format!("{styled_name} {target_name}"))
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

        let size_padding = if size.is_empty() { "" } else { " " };

        let icon = self.icon().unwrap_or("");

        let icon_padding = if icon.len() > 1 { icon.len() - 1 } else { 0 };

        let file_name = if ctx.no_color() {
            self.file_name().to_string_lossy()
        } else {
            self.stylize_link_name().unwrap_or_else(|| {
                let file_name = self.file_name_lossy();
                self.stylize(file_name)
            })
        };

        match size_loc {
            SizeLocation::Right => {
                write!(
                    f,
                    "{prefix}{icon:<icon_padding$}{file_name}{size_padding}{size}"
                )
            }
            SizeLocation::Left => {
                write!(f, "{size} {prefix}{icon:<icon_padding$}{file_name}")
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

    /// Attempts to compute icon with given parameters. Icon will be colorless if stdout isn't a
    /// tty or if `no_color` is true.
    ///
    /// The precedent from highest to lowest in terms of which parameters determine the icon used
    /// is as followed: file-type, file-extension, and then file-name. If an icon cannot be
    /// computed the fall-back default icon is used.
    ///
    /// If a directory entry is a link and the link target is provided, the link target will be
    /// used to determine the icon.
    fn compute_icon(
        entry: &DirEntry,
        link_target: Option<&PathBuf>,
        style: Option<Style>,
        no_color: bool,
    ) -> Option<Cow<'static, str>> {
        let icon = entry
            .file_type()
            .and_then(icon_from_file_type)
            .map(Cow::from);

        let paint_icon = |icon| match style {
            Some(Style {
                foreground: Some(fg),
                ..
            }) if tty::stdout_is_tty() && !no_color => {
                let ansi_string: ANSIGenericString<str> = fg.bold().paint(icon);
                let styled_icon = ansi_string.to_string();
                Some(Cow::from(styled_icon))
            }

            _ => Some(icon),
        };

        if let Some(icon) = icon {
            return paint_icon(icon);
        }

        let ext = match link_target {
            Some(target) if entry.path_is_symlink() => target.extension(),
            _ => entry.path().extension(),
        };

        let icon = ext.and_then(icon_from_ext).map(|attrs| {
            if no_color || !tty::stdout_is_tty() {
                Cow::from(attrs.1)
            } else {
                Cow::from(icons::col(attrs.0, attrs.1))
            }
        });

        if icon.is_some() {
            return icon;
        }

        let icon = icon_from_file_name(entry.file_name())
            .map(Cow::from)
            .and_then(paint_icon);

        if icon.is_some() {
            return icon;
        }

        if no_color {
            Some(Cow::from(get_default_icon().1))
        } else {
            let (code, icon) = get_default_icon();
            Some(Cow::from(icons::col(code, icon)))
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

        let file_size = match file_type {
            Some(ref ft) if ft.is_file() && !ctx.suppress_size => match ctx.disk_usage {
                DiskUsage::Logical => Some(FileSize::logical(&metadata, ctx.prefix, ctx.scale)),
                DiskUsage::Physical => FileSize::physical(path, &metadata, ctx.prefix, ctx.scale),
            },
            _ => None,
        };

        let icon = if ctx.icons {
            let no_color = ctx.no_color();
            Self::compute_icon(&dir_entry, link_target.as_ref(), style, no_color)
        } else {
            None
        };

        Ok(Self::new(
            dir_entry,
            metadata,
            file_size,
            style,
            icon,
            link_target,
        ))
    }
}
