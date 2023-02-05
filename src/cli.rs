use clap::Parser;
use crate::fs::erdtree::order::Order;
use std::{
    convert::From,
    path::{Path, PathBuf},
    usize
};
use ignore::{WalkBuilder, WalkParallel};

/// Defines the CLI.
#[derive(Parser, Debug)]
#[command(name = "Erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "File tree visualizer and disk usage analyzer.", long_about = None)]
pub struct Clargs {
    /// Root directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Ignore .gitignore; disabled by default
    #[arg(short, long)]
    pub ignore_git_ignore: bool,
    
    /// Maximum depth to display
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
    /// Returns reference to the path of the root directory to be traversed.
    pub fn dir(&self) -> &Path {
        if let Some(ref path) = self.dir {
            path.as_path()
        } else {
            Path::new(".")
        }
    }

    /// The order used for printing.
    pub fn order(&self) -> Order {
        self.order.clone()
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth
    }
}

impl From<&Clargs> for WalkParallel {
    fn from(clargs: &Clargs) -> WalkParallel {
        WalkBuilder::new(clargs.dir())
            .follow_links(false)
            .git_ignore(!clargs.ignore_git_ignore)
            .hidden(!clargs.show_hidden)
            .threads(clargs.num_threads)
            .build_parallel()
    }
}
