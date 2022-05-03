use crate::utils::bytes::{self, UnitPrefix};
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
    len: u64,
    generation: u64,
    children: Children,
}

#[derive(PartialEq)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

impl TreeNode {
    pub fn new<S>(location: &S, file_type: FileType, file_name: String, ignore_patterns: &Option<Vec<&str>>, generation: u64) -> Self
        where S: AsRef<Path> + ?Sized
    {
        let mut node = Self {
            location: location.as_ref().to_path_buf(),
            file_type,
            file_name,
            len: 0,
            generation,
            children: vec![],
        };

        if node.is_dir() {
            node.construct_branches(generation, ignore_patterns).unwrap();
        } else {
            let file_len = fs::metadata(location).unwrap().len();
            node.len = file_len;
        }

        node
    }

    pub fn get_location(&self) -> &Path {
        &self.location
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
        self.len
    }

    pub fn sprintf_file_name(&self) -> String {
        if let FileType::Dir = self.get_file_type() {
            format!("\x1B[1;33m{}\x1B[0m", self.get_file_name())
        } else {
            self.get_file_name().to_string()
        }
    }

    pub fn sprintf_len(&self) -> String {
        let len_in_bytes = self.len();
        let presentable_unit = bytes::pretty_unit(len_in_bytes);
        let presentable_len = bytes::convert(len_in_bytes, UnitPrefix::None, presentable_unit.clone());

        match presentable_unit {
            UnitPrefix::None => format!("\x1B[1;31m{}\x1B[0m \x1B[31m{:?}\x1B[0m", presentable_len, presentable_unit),
            _ => format!("\x1B[1;31m{:.*}\x1B[0m \x1B[31m{:?}\x1B[0m", 2, presentable_len, presentable_unit)
        }
    }

    fn ascertain_file_type(entry: &fs::DirEntry) -> io::Result<FileType> {
       let file_type = entry.file_type()?;

       if file_type.is_dir()  { return Ok(FileType::Dir) }
       if file_type.is_file() { return Ok(FileType::File) }

       Ok(FileType::Symlink)
    }

    fn construct_branches(&mut self, generation: u64, ignore_patterns: &Option<Vec<&str>>) -> Result<(), io::Error> {
        'entries: for possible_entry in fs::read_dir(self.get_location())? {
            if let Err(_) = possible_entry { continue }

            let entry = possible_entry.unwrap();
            let fname = entry.file_name().into_string().unwrap();

            match ignore_patterns {
                Some(ref patterns) => {
                    for i in patterns.iter() {
                        if fname.starts_with(i) { continue 'entries }
                    }
                },
                _ => ()
            }

            let epath = entry.path();
            let ftype = match Self::ascertain_file_type(&entry) {
                Ok(file_type) => file_type,
                _ => continue
            };

            let new_node = Self::new(&epath, ftype, fname, &None, generation + 1);

            self.len += new_node.len();

            self.add_child(new_node);
        }

        Ok(())
    }
}
