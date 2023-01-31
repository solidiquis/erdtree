use clap::{Parser, ValueEnum};
use std::{
    convert::From,
    path::{Path, PathBuf},
    usize
};
use ignore::{WalkBuilder, WalkParallel};

#[derive(Parser, Debug)]
#[command(name = "Erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "File tree visualizer with disk usage.", long_about = None)]
pub struct Clargs {
    /// Root directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Ignore .gitignore; disabled by default
    #[arg(short, long)]
    pub ignore_git_ignore: bool,
    
    /// Maximum depth to traverse; no limit by default
    #[arg(short, long, value_name = "NUM")]
    pub max_depth: Option<usize>,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    pub num_threads: usize,

    /// Sort order to display directory content
    #[arg(short, long, value_enum, default_value_t = Order::None)]
    order: Order,

    /// Whether to show hidden files; disabled by default
    #[arg(short, long)]
    pub show_hidden: bool
}

impl Clargs {
    pub fn dir(&self) -> &Path {
        if let Some(ref path) = self.dir {
            path.as_path()
        } else {
            Path::new(".")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Order {
    /// Sort entries by file name
    Filename,
    
    /// Sort entries by size
    Size,

    /// Devault value. Unspecified order
    None
}

impl From<&Clargs> for WalkParallel {
    fn from(clargs: &Clargs) -> WalkParallel {
        WalkBuilder::new(clargs.dir())
            .follow_links(false)
            .git_ignore(!clargs.ignore_git_ignore)
            .hidden(!clargs.show_hidden)
            .max_depth(clargs.max_depth)
            .threads(clargs.num_threads)
            .build_parallel()
    }
}
