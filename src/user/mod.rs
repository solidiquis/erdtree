use crate::error::prelude::*;
use clap::Parser;
use std::{env, fs, path::PathBuf};

/// Enum definitions for enumerated command-line arguments.
pub mod args;

/// Defines the CLI whose purpose is to capture user arguments and reconcile them with arguments
/// found with a config file if relevant.
#[derive(Parser, Debug)]
#[command(name = "erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "4.0.0")]
#[command(
    about = "erdtree (erd) is a cross-platform, multi-threaded, and general purpose filesystem and disk usage utility.",
    long_about = None,
)]
pub struct Context {
    /// Directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Ignore hidden files
    #[arg(short = '.', long)]
    pub no_hidden: bool,

    /// Ignore .git directory
    #[arg(long)]
    pub no_git: bool,

    /// Ignore files in .gitignore
    #[arg(short = 'i', long)]
    pub gitignore: bool,

    /// Report byte size in either binary or SI units
    #[arg(short, long, value_enum, default_value_t)]
    pub byte_units: args::BytePresentation,

    /// Follow symlinks
    #[arg(short = 'f', long)]
    pub follow: bool,

    /// Show extended metadata and attributes
    #[cfg(unix)]
    #[arg(short, long)]
    pub long: bool,

    /// Show file's groups
    #[cfg(unix)]
    #[arg(long)]
    pub group: bool,

    /// Show each file's ino
    #[cfg(unix)]
    #[arg(long)]
    pub ino: bool,

    /// Show the total number of hardlinks to the underlying inode
    #[cfg(unix)]
    #[arg(long)]
    pub nlink: bool,

    /// Show permissions in numeric octal format instead of symbolic
    #[cfg(unix)]
    #[arg(long, requires = "long")]
    pub octal: bool,

    /// Which kind of timestamp to use; modified by default
    #[cfg(unix)]
    #[arg(long, value_enum, requires = "long")]
    pub time: Option<args::TimeStamp>,

    /// Which format to use for the timestamp; default by default
    #[cfg(unix)]
    #[arg(long = "time-format", value_enum, requires = "long")]
    pub time_format: Option<args::TimeFormat>,

    /// Maximum depth to display
    #[arg(short = 'L', long, value_name = "NUM")]
    level: Option<usize>,

    #[arg(short, long, value_enum, default_value_t)]
    pub metric: args::Metric,

    /// Which kind of layout to use when rendering the output
    #[arg(short = 'y', long, value_enum, default_value_t)]
    pub layout: args::Layout,

    /// Number of threads to use for disk reads
    #[arg(short = 'T', long, default_value_t = Context::default_num_threads())]
    pub threads: usize,

    /// Prevent traversal into directories that are on different filesystems
    #[arg(short = 'x', long = "one-file-system")]
    pub same_fs: bool,

    /// Prints logs at the end of the output
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

impl Context {
    pub fn init() -> Result<Self> {
        let mut clargs = Self::parse();

        if clargs.dir.is_none() {
            let current_dir = Self::get_current_dir()?;
            clargs.dir = Some(current_dir);
        }

        Ok(clargs)
    }

    pub fn dir(&self) -> Option<&PathBuf> {
        self.dir.as_ref()
    }

    pub fn dir_canonical(&self) -> Result<PathBuf> {
        match self.dir() {
            Some(root) => fs::canonicalize(root).into_report(ErrorCategory::Internal),
            None => Self::get_current_dir(),
        }
    }

    pub fn get_current_dir() -> Result<PathBuf> {
        env::current_dir()
            .and_then(fs::canonicalize)
            .into_report(ErrorCategory::System)
            .context("Failed to access current working directory")
            .set_help("Ensure current directory exists and sufficient permissions are granted")
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn level(&self) -> usize {
        self.level.unwrap_or(usize::MAX)
    }

    fn default_num_threads() -> usize {
        std::thread::available_parallelism().map_or(3, usize::from)
    }
}
