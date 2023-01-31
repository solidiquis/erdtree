use crate::fs::file_size::FileSize;
use ignore::DirEntry;
use std::{
    convert::From,
    fmt::{self, Display, Formatter},
    fs::FileType,
    path::{Path, PathBuf},
    slice::Iter,
};

#[derive(Debug)]
pub struct Node {
    pub depth: usize,
    pub file_size: Option<u64>,
    children: Option<Vec<Node>>,
    file_name: String,
    file_type: Option<FileType>,
    path: PathBuf,
}

impl Node {
    pub fn set_children(&mut self, children: Vec<Node>) {
        self.children = Some(children);
    }

    pub fn set_file_size(&mut self, size: u64) {
        self.file_size = Some(size);
    }
    
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        self.children.as_mut()
    }

    pub fn children(&self) -> Option<Iter<Node>> {
        self.children.as_ref().map(|children| children.iter())
    }

    pub fn is_dir(&self) -> bool {
        self.file_type
            .as_ref()
            .map(|ft| ft.is_dir())
            .unwrap_or(false)
    }

    /// Hmm... this is kind of expensive.
    pub fn parent_path_buf(&self) -> Option<PathBuf> {
        let mut path_buf = self.path.clone();

        if path_buf.pop() {
            Some(path_buf)
        } else {
            None
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl From<DirEntry> for Node {
    fn from(dir_entry: DirEntry) -> Self {
        let children = None;
        let depth = dir_entry.depth();
        let file_name = dir_entry.file_name()
            .to_string_lossy()
            .into_owned();
        let file_type = dir_entry.file_type();

        let file_size = if let Some(ref ft) = file_type {
            if ft.is_file() {
                dir_entry.metadata().ok().map(|md| md.len())
            } else {
                None
            }
        } else {
            None
        };

        let path = dir_entry.into_path()
            .to_owned();
        
        Self {
            children,
            depth,
            file_name,
            file_size,
            file_type,
            path
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let size = self.file_size
            .or(Some(0))
            .map(|size| FileSize::new(size) )
            .unwrap();

        write!(f, "{} ({})", self.file_name, size)
    }
}
