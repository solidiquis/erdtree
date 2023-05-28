use super::Node;
use std::{
    convert::From,
    fmt::{self, Display},
    ops::{Add, AddAssign},
};

/// For keeping track of the number of various file-types of [Node]'s chlidren.
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct FileCount {
    pub num_dirs: usize,
    pub num_files: usize,
    pub num_links: usize,
}

impl AddAssign<&Node> for FileCount {
    /// Update [Self] with information from [Node].
    fn add_assign(&mut self, rhs: &Node) {
        if rhs.is_dir() {
            self.num_dirs += 1;
        } else if rhs.is_symlink() {
            self.num_links += 1;
        } else {
            self.num_files += 1;
        }
    }
}
impl Add<&Node> for FileCount {
    type Output = Self;
    /// Update [Self] with information from [Node].
    fn add(self, rhs: &Node) -> Self::Output {
        if rhs.is_dir() {
            Self {
                num_dirs: self.num_dirs + 1,
                ..self
            }
        } else if rhs.is_symlink() {
            Self {
                num_links: self.num_links + 1,
                ..self
            }
        } else {
            Self {
                num_files: self.num_files + 1,
                ..self
            }
        }
    }
}

impl AddAssign for FileCount {
    /// Update [Self] with information from [Self].
    fn add_assign(&mut self, rhs: Self) {
        self.num_dirs += rhs.num_dirs;
        self.num_links += rhs.num_links;
        self.num_files += rhs.num_files;
    }
}
impl Add for FileCount {
    type Output = Self;
    /// Add [Self] with information from another [Self].
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num_dirs: self.num_dirs + rhs.num_dirs,
            num_links: self.num_links + rhs.num_links,
            num_files: self.num_files + rhs.num_files,
        }
    }
}

impl From<Vec<Self>> for FileCount {
    fn from(data: Vec<Self>) -> Self {
        data.into_iter()
            .fold(Self::default(), |acc, datum| acc + datum)
    }
}

impl Display for FileCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut components = vec![];

        if self.num_dirs > 0 {
            let output = format!(
                "{} {}",
                self.num_dirs,
                if self.num_dirs > 1 {
                    "directories"
                } else {
                    "directory"
                }
            );

            components.push(output);
        }

        if self.num_files > 0 {
            let output = format!(
                "{} {}",
                self.num_files,
                if self.num_files > 1 { "files" } else { "file" }
            );

            components.push(output);
        }

        if self.num_links > 0 {
            let output = format!(
                "{} {}",
                self.num_links,
                if self.num_links > 1 { "links" } else { "link" }
            );

            components.push(output);
        }

        write!(f, "{}", components.join(", "))
    }
}
