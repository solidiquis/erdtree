use crate::fs::erdtree::order::Order;
use clap::Parser;
use ignore::{
    overrides::{Override, OverrideBuilder},
    WalkBuilder, WalkParallel,
};
use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    usize,
};

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
    pub show_hidden: bool,

    /// Include or exclude files using glob patterns
    #[arg(short, long)]
    glob: Vec<String>,

    /// Include or exclude files using glob patterns; case insensitive
    #[arg(long)]
    iglob: Vec<String>,

    /// Process all glob patterns case insensitively
    #[arg(long)]
    glob_case_insensitive: bool,
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
        self.order
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth
    }

    /// Ignore file overrides.
    pub fn overrides(&self) -> Result<Override, ignore::Error> {
        if self.glob.is_empty() && self.iglob.is_empty() {
            return Ok(Override::empty());
        }

        let mut builder = OverrideBuilder::new(self.dir());
        if self.glob_case_insensitive {
            builder.case_insensitive(true).unwrap();
        }

        for glob in self.glob.iter() {
            builder.add(glob)?;
        }

        // all subsequent patterns are case insensitive
        builder.case_insensitive(true).unwrap();
        for glob in self.iglob.iter() {
            builder.add(glob)?;
        }

        builder.build()
    }
}

impl TryFrom<&Clargs> for WalkParallel {
    type Error = Error;

    fn try_from(clargs: &Clargs) -> Result<Self, Self::Error> {
        Ok(WalkBuilder::new(clargs.dir())
            .follow_links(false)
            .overrides(clargs.overrides()?)
            .git_ignore(!clargs.ignore_git_ignore)
            .hidden(!clargs.show_hidden)
            .threads(clargs.num_threads)
            .build_parallel())
    }
}

/// Errors which may occur during command-line argument parsing.
#[derive(Debug)]
pub enum Error {
    InvalidGlobPatterns(ignore::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidGlobPatterns(e) => write!(f, "Invalid glob patterns: {e}"),
        }
    }
}

impl StdError for Error {}

impl From<ignore::Error> for Error {
    fn from(value: ignore::Error) -> Self {
        Self::InvalidGlobPatterns(value)
    }
}
