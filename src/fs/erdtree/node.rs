use super::get_ls_colors;
use crate::fs::file_size::FileSize;
use ansi_term::Style;
use ignore::DirEntry;
use lscolors::Style as LS_Style;
use std::{
    convert::From,
    fmt::{self, Display, Formatter},
    fs::FileType,
    path::{Path, PathBuf},
    slice::Iter,
};

/// A node of [super::tree::Tree] that can be created from a [DirEntry]. Any filesystem I/O and
/// relevant system calls are expected to complete after initialization. A `Node` when `Display`ed
/// uses ANSI colors determined by the file-type and `LS_COLORS`.
#[derive(Debug)]
pub struct Node {
    pub depth: usize,
    pub file_size: Option<u64>,
    children: Option<Vec<Node>>,
    file_name: String,
    file_type: Option<FileType>,
    path: PathBuf,
    style: Style,
}

impl Node {
    /// Initializes a new [Node].
    pub fn new(
        depth: usize,
        file_size: Option<u64>,
        children: Option<Vec<Node>>,
        file_name: String,
        file_type: Option<FileType>,
        path: PathBuf,
        style: Style,
    ) -> Self {
        Self {
            children,
            depth,
            file_name,
            file_size,
            file_type,
            path,
            style,
        }
    }

    /// Returns a mutable reference to `children` if any.
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        self.children.as_mut()
    }

    /// Returns an iter over a `children` slice if any.
    pub fn children(&self) -> Option<Iter<Node>> {
        self.children.as_ref().map(|children| children.iter())
    }

    /// Returns a reference to `file_name`.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns `true` if node is a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type
            .as_ref()
            .map(|ft| ft.is_dir())
            .unwrap_or(false)
    }

    /// Returns `true` if node is a symlink.
    pub fn is_symlink(&self) -> bool {
        self.file_type
            .as_ref()
            .map(|ft| ft.is_symlink())
            .unwrap_or(false)
    }

    /// Returns the path to the `Node`'s parent, if any. This is a pretty expensive operation used
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

    /// Set's `children`.
    pub fn set_children(&mut self, children: Vec<Node>) {
        self.children = Some(children);
    }

    /// Set's `file_size`.
    pub fn set_file_size(&mut self, size: u64) {
        self.file_size = Some(size);
    }

    /// Set's 'style'.
    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl From<DirEntry> for Node {
    fn from(dir_entry: DirEntry) -> Self {
        let children = None;

        let depth = dir_entry.depth();

        let file_name = dir_entry.file_name().to_string_lossy().into_owned();

        let file_type = dir_entry.file_type();

        let metadata = dir_entry.metadata().ok();

        let path = dir_entry.into_path();

        let style = get_ls_colors()
            .style_for_path_with_metadata(&path, metadata.as_ref())
            .map(LS_Style::to_ansi_term_style)
            .unwrap_or_default();

        let mut file_size = None;

        if let Some(ref ft) = file_type {
            if ft.is_file() {
                file_size = metadata.map(|md| md.len())
            }
        };

        Self::new(
            depth, file_size, children, file_name, file_type, path, style,
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let size = self
            .file_size
            .map(|size| format!("({})", FileSize::new(size)))
            .or_else(|| Some("".to_owned()))
            .unwrap();

        let output = self
            .style()
            .foreground
            .map(|fg| format!("{} {size}", fg.bold().paint(self.file_name())))
            .unwrap_or_else(|| format!("{} {size}", self.file_name()));

        write!(f, "{output}")
    }
}
