use crate::error::prelude::*;
use clap::Parser;
use std::{env, fs, path::PathBuf};

/// Enum definitions for enumerated command-line arguments.
pub mod enums;

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

    /// Show hidden files
    #[arg(short = '.', long)]
    pub hidden: bool,

    /// Report byte size in either binary or SI units
    #[arg(short, long, value_enum, default_value_t)]
    pub byte_units: enums::BytePresentation,

    /// Disable traversal of .git directory when traversing hidden files
    #[arg(long, requires = "hidden")]
    pub no_git: bool,

    /// Follow symlinks
    #[arg(short = 'f', long)]
    pub follow: bool,

    #[arg(short, long, value_enum, default_value_t)]
    pub metric: enums::Metric,

    /// Do not respect .gitignore files
    #[arg(short = 'i', long)]
    pub no_ignore: bool,

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

    fn default_num_threads() -> usize {
        std::thread::available_parallelism().map_or(3, usize::from)
    }
}
