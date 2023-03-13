use super::super::get_ls_colors;
use crate::{
    fs::inode::Inode,
    icons::{self, icon_from_ext, icon_from_file_name, icon_from_file_type},
    render::{
        context::Context,
        disk_usage::{DiskUsage, FileSize},
        order::NodeComparator,
    },
};
use ansi_term::Color;
use ansi_term::Style;
use ignore::DirEntry;
use lscolors::Style as LS_Style;
use std::{
    borrow::Cow,
    convert::From,
    ffi::{OsStr, OsString},
    fmt::{self, Display, Formatter},
    fs::{self, FileType},
    path::{Path, PathBuf},
    slice::{Iter, IterMut},
};

/// A node of [`Tree`] that can be created from a [DirEntry]. Any filesystem I/O and
/// relevant system calls are expected to complete after initialization. A `Node` when `Display`ed
/// uses ANSI colors determined by the file-type and [`LS_COLORS`].
///
/// [`Tree`]: super::tree::Tree
/// [`LS_COLORS`]: super::tree::ui::LS_COLORS
#[derive(Clone, Debug)]
pub struct Node {
    pub depth: usize,
    pub file_size: Option<FileSize>,
    children: Option<Vec<Node>>,
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
        children: Option<Vec<Node>>,
        file_name: OsString,
        file_type: Option<FileType>,
        inode: Option<Inode>,
        path: PathBuf,
        show_icon: bool,
        style: Style,
        symlink_target: Option<PathBuf>,
    ) -> Self {
        Self {
            children,
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

    /// Returns a mutable reference to `children` if any.
    pub fn children_mut(&mut self) -> Option<IterMut<Node>> {
        self.children.as_mut().map(|children| children.iter_mut())
    }

    /// Returns an iter over a `children` slice if any.
    pub fn children(&self) -> Option<Iter<Node>> {
        self.children.as_ref().map(|children| children.iter())
    }

    /// Sorts `children` given comparator.
    pub fn sort_children(&mut self, comparator: Box<NodeComparator<'_>>) {
        self.children
            .as_mut()
            .map(|nodes| nodes.sort_by(comparator));
    }

    /// Recursively traverse [Node]s, removing any [Node]s that have no children.
    pub fn prune_directories(&mut self) {
        self.children.as_mut().map(|nodes| {
            nodes.retain_mut(|node| {
                if node.is_dir() {
                    if node.children.is_some() {
                        node.prune_directories();
                        return true;
                    } else {
                        return false;
                    }
                }
                true
            })
        });
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
            .map_or_else(|| self.file_name().to_string_lossy(), |s| Cow::from(s))
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
        self.symlink_target.as_ref().map(PathBuf::as_path)
    }

    /// Returns the file name of the symlink target if [Node] represents a symlink.
    pub fn symlink_target_file_name(&self) -> Option<&OsStr> {
        self.symlink_target_path()
            .map(|path| path.file_name())
            .flatten()
    }

    /// Returns reference to underlying [FileType].
    pub fn file_type(&self) -> Option<&FileType> {
        self.file_type.as_ref()
    }

    /// Returns the path to the [Node]'s parent, if any. This is a pretty expensive operation used
    /// during parallel traversal. Perhaps an area for optimization.
    pub fn parent_path_buf(&self) -> Option<PathBuf> {
        let mut path_buf = self.path.clone();

        if path_buf.pop() {
            Some(path_buf)
        } else {
            None
        }
    }

    /// Returns a reference to `path`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Sets `children`.
    pub fn set_children(&mut self, children: Vec<Node>) {
        self.children = Some(children);
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

    /// Gets stylized icon for node if enabled. Icons without extensions are styled based on the
    /// [`LS_COLORS`] foreground configuration of the associated file name.
    ///
    /// [`LS_COLORS`]: super::tree::ui::LS_COLORS
    fn get_icon(&self) -> Option<String> {
        if !self.show_icon {
            return None;
        }

        let path = self.symlink_target_path().unwrap_or_else(|| self.path());

        if let Some(icon) = path.extension().map(icon_from_ext).flatten() {
            return Some(self.stylize(icon));
        }

        if let Some(icon) = self.file_type().map(icon_from_file_type).flatten() {
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
    /// [`LS_COLORS`]: super::tree::ui::LS_COLORS
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
            format!("{} {}", styled_name, target_name)
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

        let scale = scale.clone();
        let prefix = prefix.clone();
        let icons = icons.clone();

        let children = None;

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
            children,
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

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let size = self
            .file_size()
            .map(|size| format!("({})", size))
            .or_else(|| Some("".to_owned()))
            .unwrap();

        let icon = self
            .show_icon
            .then(|| self.get_icon())
            .flatten()
            .unwrap_or("".to_owned());

        let icon_padding = (icon.len() > 1).then(|| icon.len() - 1).unwrap_or(0);

        let (styled_name, name_padding) = self
            .stylize_link_name()
            .map(|name| {
                let padding = name.len() - 1;
                (name, padding)
            })
            .or_else(|| {
                let file_name = self.file_name_lossy();
                let name = self.stylize(&file_name);
                let padding = name.len() + 1;

                Some((name, padding))
            })
            .unwrap();

        let output = format!(
            "{:<icon_padding$}{:<name_padding$}{size}",
            icon,
            styled_name,
            icon_padding = icon_padding,
            name_padding = name_padding
        );

        write!(f, "{output}")
    }
}
