use ignore::DirEntry;
use std::{
    convert::From,
    fmt::{self, Display, Formatter},
    fs::FileType,
    rc::Rc,
};

/// Counterpart of `LocalNode` that can exist in a multi-threaded context. Nothing is inherently
/// special about `SharedNode` which makes it `Send` and `Sync`, other than the fact that it
/// doesn't have reference-counted children which exists in its counter-part. All filesystem I/O
/// needed to construct a `SharedNode` is done upon its initialization.
#[derive(Debug)]
pub struct SharedNode {
    pub(self) depth: usize,
    pub(self) file_name: String,
    pub(self) file_size: Option<u64>,
    pub(self) file_type: Option<FileType>,
}

/// Counterpart of `SharedNode` that is used strictly in a single-threaded context.
pub struct LocalNode {
    children: Vec<Rc<LocalNode>>,
    pub depth: usize,
    pub file_name: String,
    pub file_size: Option<u64>,
    pub file_type: Option<FileType>,
}

impl SharedNode {
    pub fn new(dir_entry: DirEntry) -> Self {
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
        
        Self {
            depth,
            file_name,
            file_size,
            file_type
        }
    }
}

impl From<SharedNode> for LocalNode {
    fn from(node: SharedNode) -> Self {
        let SharedNode {
            depth,
            file_name,
            file_size,
            file_type,
        } = node;

        let children = vec![];

        Self {
            children,
            depth,
            file_name,
            file_size,
            file_type,
        }
    }
}

impl LocalNode {
    pub fn push_child(&mut self, child: Rc<LocalNode>) {
        self.children.push(child);
    }
}

impl Display for LocalNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name)
    }
}
