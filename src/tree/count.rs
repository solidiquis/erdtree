use super::Node;
use std::{
    convert::From,
    fmt::{self, Display},
};

/// For keeping track of the number of various file-types of [Node]'s chlidren.
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct FileCount {
    pub num_dirs: usize,
    pub num_files: usize,
    pub num_links: usize,
}

impl FileCount {
    /// Update [Self] with information from [Node].
    pub fn update(&mut self, node: &Node) {
        if node.is_dir() {
            self.num_dirs += 1;
        } else if node.is_symlink() {
            self.num_links += 1;
        } else {
            self.num_files += 1;
        }
    }

    /// Update [Self] with information from [Self].
    pub fn update_from_count(
        &mut self,
        Self {
            num_dirs,
            num_files,
            num_links,
        }: Self,
    ) {
        self.num_dirs += num_dirs;
        self.num_links += num_links;
        self.num_files += num_files;
    }
}

impl From<Vec<Self>> for FileCount {
    fn from(data: Vec<Self>) -> Self {
        let mut agg = Self::default();

        for datum in data {
            agg.update_from_count(datum);
        }

        agg
    }
}

impl Display for FileCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut components = vec![];

        if self.num_dirs > 0 {
            let output = if self.num_dirs > 1 {
                format!("{} {}", self.num_dirs, "directories")
            } else {
                format!("{} {}", self.num_dirs, "directory")
            };

            components.push(output);
        }

        if self.num_files > 0 {
            let output = if self.num_files > 1 {
                format!("{} {}", self.num_files, "files")
            } else {
                format!("{} {}", self.num_files, "file")
            };

            components.push(output);
        }

        if self.num_links > 0 {
            let output = if self.num_links > 1 {
                format!("{} {}", self.num_links, "links")
            } else {
                format!("{} {}", self.num_links, "link")
            };

            components.push(output);
        }

        write!(f, "{}", components.join(", "))
    }
}
