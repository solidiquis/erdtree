use super::super::get_ls_colors;
use crate::{
    fs::inode::Inode,
    icons::{self, icon_from_ext, icon_from_file_name, icon_from_file_type},
    render::{
        context::Context,
        disk_usage::{DiskUsage, FileSize},
    },
};
use ansi_term::Color;
use ansi_term::Style;
use indextree::{Arena, Node as NodeWrapper, NodeId};
use ignore::DirEntry;
use lscolors::Style as LS_Style;
use std::{
    borrow::Cow,
    convert::From,
    ffi::{OsStr, OsString},
    fmt::{self, Formatter},
    fs::{self, FileType},
    path::{Path, PathBuf},
};

/// A node of [`Tree`] that can be created from a [DirEntry]. Any filesystem I/O and
/// relevant system calls are expected to complete after initialization. A `Node` when `Display`ed
/// uses ANSI colors determined by the file-type and [`LS_COLORS`].
///
/// [`Tree`]: super::Tree
/// [`LS_COLORS`]: super::ui::LS_COLORS
#[derive(Debug)]
pub struct Node {
    pub depth: usize,
    pub file_size: Option<FileSize>,
    file_name: OsString,
    file_type: Option<FileType>,
    inode: Option<Inode>,
    path: PathBuf,
    show_icon: bool,
    style: Style,
    symlink_target: Option<PathBuf>,
}

impl Node {
    /// Initializes a new [Node].
    pub fn new(
        depth: usize,
        file_size: Option<FileSize>,
        file_name: OsString,
        file_type: Option<FileType>,
        inode: Option<Inode>,
        path: PathBuf,
        show_icon: bool,
        style: Style,
        symlink_target: Option<PathBuf>,
    ) -> Self {
        Self {
            depth,
            file_name,
            file_size,
            file_type,
            inode,
            path,
            show_icon,
            style,
            symlink_target,
        }
    }

    /// Recursively traverse [Node]s, removing any [Node]s that have no children.
    pub fn prune_directories(&mut self) {
        todo!();
    }

    /// Returns a reference to `file_name`. If file is a symlink then `file_name` is the name of
    /// the symlink not the target.
    pub fn file_name(&self) -> &OsStr {
        &self.file_name
    }

    /// Converts `OsStr` to `String`; if fails does a lossy conversion replacing non-Unicode
    /// sequences with Unicode replacement scalar value.
    pub fn file_name_lossy(&self) -> Cow<'_, str> {
        self.file_name()
            .to_str()
            .map_or_else(|| self.file_name().to_string_lossy(), Cow::from)
    }

    /// Returns `true` if node is a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
    }

    /// Is the Node a symlink.
    pub fn is_symlink(&self) -> bool {
        self.symlink_target.is_some()
    }

    /// Path to symlink target.
    pub fn symlink_target_path(&self) -> Option<&Path> {
        self.symlink_target.as_deref()
    }

    /// Returns the file name of the symlink target if [Node] represents a symlink.
    pub fn symlink_target_file_name(&self) -> Option<&OsStr> {
        self.symlink_target_path().and_then(|path| path.file_name())
    }

    /// Returns reference to underlying [FileType].
    pub fn file_type(&self) -> Option<&FileType> {
        self.file_type.as_ref()
    }

    /// Returns the path to the [Node]'s parent, if any.
    pub fn parent_path(&self) -> Option<&Path> {
        self.path.parent()
    }

    /// Returns a reference to `path`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets 'file_size'.
    pub fn file_size(&self) -> Option<&FileSize> {
        self.file_size.as_ref()
    }

    /// Sets `file_size`.
    pub fn set_file_size(&mut self, size: FileSize) {
        self.file_size = Some(size);
    }

    /// Sets 'style'.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Returns reference to underlying [Inode] if any.
    pub fn inode(&self) -> Option<&Inode> {
        self.inode.as_ref()
    }

    /// Get a reference to [Node] from arena.
    pub fn get(node_id: NodeId, tree: &Arena<Node>) -> Option<&Self> {
        tree.get(node_id).map(NodeWrapper::get)
    }

    /// Get a mutable reference to [Node] from arena.
    pub fn get_mut(node_id: NodeId, tree: &mut Arena<Node>) -> Option<&mut Self> {
        tree.get_mut(node_id).map(NodeWrapper::get_mut)
    }

    /// Gets stylized icon for node if enabled. Icons without extensions are styled based on the
    /// [`LS_COLORS`] foreground configuration of the associated file name.
    ///
    /// [`LS_COLORS`]: super::ui::LS_COLORS
    fn get_icon(&self) -> Option<String> {
        if !self.show_icon {
            return None;
        }

        let path = self.symlink_target_path().unwrap_or_else(|| self.path());

        if let Some(icon) = self.file_type().and_then(icon_from_file_type) {
            return Some(self.stylize(icon));
        }

        if let Some(icon) = path.extension().and_then(icon_from_ext) {
            return Some(self.stylize(icon));
        }

        let file_name = self
            .symlink_target_file_name()
            .unwrap_or_else(|| self.file_name());

        if let Some(icon) = icon_from_file_name(file_name) {
            return Some(self.stylize(icon));
        }

        Some(icons::get_default_icon().to_owned())
    }

    /// Stylizes input, `entity` based on [`LS_COLORS`]
    ///
    /// [`LS_COLORS`]: super::ui::LS_COLORS
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
}

impl From<(&DirEntry, &Context)> for Node {
    fn from(data: (&DirEntry, &Context)) -> Self {
        let (dir_entry, ctx) = data;
        let Context {
            disk_usage,
            icons,
            scale,
            suppress_size,
            prefix,
            ..
        } = ctx;

        let scale = *scale;
        let prefix = *prefix;
        let icons = *icons;

        let depth = dir_entry.depth();

        let file_type = dir_entry.file_type();

        let path = dir_entry.path();

        let symlink_target = dir_entry
            .path_is_symlink()
            .then(|| fs::read_link(path))
            .transpose()
            .ok()
            .flatten();

        let file_name = path.file_name().map_or_else(
            || OsString::from(path.display().to_string()),
            |os_str| os_str.to_owned(),
        );

        let metadata = dir_entry.metadata().ok();

        let style = get_ls_colors()
            .style_for_path_with_metadata(path, metadata.as_ref())
            .map(LS_Style::to_ansi_term_style)
            .unwrap_or_default();

        let mut file_size = None;

        if !suppress_size {
            if let Some(ref ft) = file_type {
                if ft.is_file() {
                    if let Some(ref md) = metadata {
                        file_size = match disk_usage {
                            DiskUsage::Logical => Some(FileSize::logical(md, prefix, scale)),
                            DiskUsage::Physical => FileSize::physical(path, md, prefix, scale),
                        }
                    }
                }
            }
        };

        let inode = metadata.map(Inode::try_from).transpose().ok().flatten();

        Self::new(
            depth,
            file_size,
            file_name,
            file_type,
            inode,
            path.into(),
            icons,
            style,
            symlink_target,
        )
    }
}

impl From<(NodeId, &mut Arena<Self>)> for &Node {
    fn from((node_id, tree): (NodeId, &mut Arena<Self>)) -> Self {
        tree.get(node_id).map(NodeWrapper::get).unwrap()
    }
}

/// Simple struct to define location to put the `FileSize` while printing a `Node`
#[derive(Copy, Clone, Default)]
enum SizeLocation {
    #[default]
    Right,
    Left,
}

impl SizeLocation {
    /// Returns a string to use when a node has no filesize, such as empty directories
    fn default_string(self, ctx: &Context) -> String {
        use SizeLocation::*;
        match self {
            Right => "".to_owned(),
            Left => FileSize::empty_string(ctx),
        }
    }

    /// Given a [`FileSize`], style it in the expected way for its printing location
    fn format(self, size: &FileSize) -> String {
        use SizeLocation::*;
        match self {
            Right => format!("({})", size.format(false)),
            Left => size.format(true),
        }
    }
}

impl Node {
    /// General method for printing a `Node`. The `Display` (and `ToString`) traits are not used,
    /// to give more control over the output.
    ///
    /// See [`Node::display_size_left`] and [`Node::display_size_right`] for examples of formatted output.
    fn display(
        &self,
        f: &mut Formatter,
        size_loc: SizeLocation,
        prefix: &str,
        ctx: &Context,
    ) -> fmt::Result {
        let size = self.file_size().map_or_else(
            || size_loc.default_string(ctx),
            |size| size_loc.format(size),
        );

        let icon = if self.show_icon {
            self.get_icon().unwrap()
        } else {
            "".to_owned()
        };

        let icon_padding = if icon.len() > 1 { icon.len() - 1 } else { 0 };

        let styled_name = self.stylize_link_name().unwrap_or_else(|| {
            let file_name = self.file_name_lossy();
            self.stylize(&file_name)
        });

        match size_loc {
            SizeLocation::Right => {
                write!(f, "{prefix}{icon:<icon_padding$}{styled_name} {size}",)
            }
            SizeLocation::Left => {
                write!(f, "{size} {prefix}{icon:<icon_padding$}{styled_name}",)
            }
        }
    }

    /// Format a node for display with size on the right.
    ///
    /// Example:
    /// `| Some Directory (12.3 KiB)`
    pub fn display_size_right(
        &self,
        f: &mut Formatter,
        prefix: &str,
        ctx: &Context,
    ) -> fmt::Result {
        self.display(f, SizeLocation::Right, prefix, ctx)
    }

    /// Format a node for display with size on the left.
    ///
    /// Example:
    /// `  1.23 MiB | Some File`
    ///
    /// Note the two spaces to the left of the first character of the number -- even if never used,
    /// numbers are padded to 3 digits to the left of the decimal (and ctx.scale digits after)
    pub fn display_size_left(&self, f: &mut Formatter, prefix: &str, ctx: &Context) -> fmt::Result {
        self.display(f, SizeLocation::Left, prefix, ctx)
    }
}
