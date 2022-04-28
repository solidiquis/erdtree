use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use std::slice::Iter;

pub type TreeNodeResult = Result<TreeNode, io::Error>;
pub type Children = Vec<TreeNode>;

pub struct TreeNode {
    location: PathBuf,
    file_type: FileType,
    file_name: String,
    metadata: fs::Metadata,
    generation: u64,
    children: Children
}

#[derive(PartialEq)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

impl TreeNode {
    pub fn new<S>(location: &S, file_type: FileType, file_name: String, generation: u64) -> Self
        where S: AsRef<Path> + ?Sized
    {
        let metadata = fs::metadata(location).unwrap();

        let mut node = Self {
            location: location.as_ref().to_path_buf(),
            file_type,
            file_name,
            metadata,
            generation,
            children: vec![]
        };

        node.construct_branches(generation).unwrap();

        node
    }

    pub fn get_location(&self) -> &Path {
        &self.location
    }

    pub fn get_metadata(&self) -> &fs::Metadata {
        &self.metadata
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Dir
    }

    pub fn is_not_dir(&self) -> bool {
        !self.is_dir()
    }

    pub fn get_file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn iter_children(&self) -> Iter<'_, TreeNode> {
        self.children.iter()
    }

    pub fn num_children(&self) -> u64 {
        self.children.len() as u64
    }

    pub fn len(&self) -> u64 {
        self.metadata.len()
    }

    fn ascertain_file_type(entry: &fs::DirEntry) -> io::Result<FileType> {
       let file_type = entry.file_type()?;

       if file_type.is_dir()  { return Ok(FileType::Dir) }
       if file_type.is_file() { return Ok(FileType::File) }

       Ok(FileType::Symlink)
    }

    fn construct_branches(&mut self, generation: u64) -> Result<(), io::Error> {
        if self.is_not_dir() { return Ok(()) }

        for possible_entry in fs::read_dir(self.get_location())? {
            if let Err(_) = possible_entry { continue }

            let entry = possible_entry.unwrap();
            let epath = entry.path();
            let fname = entry.file_name().into_string().unwrap();
            let ftype = match Self::ascertain_file_type(&entry) {
                Ok(file_type) => file_type,
                _ => continue
            };

            let new_node = Self::new(&epath, ftype, fname, generation + 1);

            self.add_child(new_node);
        }

        Ok(())
    }
}
