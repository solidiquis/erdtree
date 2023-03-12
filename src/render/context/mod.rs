use super::{disk_usage::DiskUsage, order::SortType};
use clap::{CommandFactory, Error as ClapError, FromArgMatches, Parser};
use ignore::overrides::{Override, OverrideBuilder};
use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display},
    path::{Path, PathBuf},
    usize,
};

/// Operations to load in `.erdtree.toml` defaults.
pub mod config;

/// Unit tests for [Context]
#[cfg(test)]
mod test;

/// Defines the CLI.
#[derive(Parser, Debug)]
#[command(name = "erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "1.3.0")]
#[command(about = "erdtree (et) is a multi-threaded filetree visualizer and disk usage analyzer.", long_about = None)]
pub struct Context {
    /// Root directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Print physical or logical file size
    #[arg(short, long, value_enum, default_value_t = DiskUsage::Logical)]
    pub disk_usage: DiskUsage,

    /// Include or exclude files using glob patterns
    #[arg(short, long)]
    glob: Vec<String>,

    /// Include or exclude files using glob patterns; case insensitive
    #[arg(long)]
    iglob: Vec<String>,

    /// Process all glob patterns case insensitively
    #[arg(long)]
    glob_case_insensitive: bool,

    /// Show hidden files; disabled by default
    #[arg(short = 'H', long)]
    pub hidden: bool,

    /// Disable traversal of .git directory when traversing hidden files; disabled by default
    #[arg(long)]
    ignore_git: bool,

    /// Display file icons; disabled by default
    #[arg(short = 'I', long)]
    pub icons: bool,

    /// Ignore .gitignore; disabled by default
    #[arg(short, long)]
    pub ignore_git_ignore: bool,

    /// Maximum depth to display
    #[arg(short, long, value_name = "NUM")]
    pub level: Option<usize>,

    /// Total number of digits after the decimal to display for disk usage
    #[arg(short = 'n', long, default_value_t = 2, value_name = "NUM")]
    pub scale: usize,

    /// Sort-order to display directory content
    #[arg(short, long, value_enum)]
    sort: Option<SortType>,

    /// Always sorts directories above files
    #[arg(long)]
    dirs_first: bool,

    /// Traverse symlink directories and consider their disk usage; disabled by default
    #[arg(short = 'S', long)]
    pub follow_links: bool,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,

    /// Omit disk usage from output; disabled by default
    #[arg(long)]
    pub suppress_size: bool,

    /// Don't read configuration file
    #[arg(long)]
    pub no_config: bool,
}

impl Context {
    /// Initializes [Context], optionally reading in the configuration file to override defaults.
    /// Arguments provided will take precedence over config.
    pub fn init() -> Result<Self, Error> {
        let clargs = Context::command().args_override_self(true).get_matches();

        let no_config = clargs
            .get_one("no_config")
            .map(bool::clone)
            .unwrap_or(false);

        let context = {
            if no_config {
                Context::from_arg_matches(&clargs).map_err(|e| Error::ArgParse(e))?
            } else {
                if let Some(ref config) = config::read_config_to_string::<&str>(None) {
                    let raw_config_args = config::parse_config(config);
                    let config_args = Context::command().get_matches_from(raw_config_args);

                    let mut ctx =
                        Context::from_arg_matches(&config_args).map_err(|e| Error::Config(e))?;

                    ctx.update_from_arg_matches(&clargs)
                        .map_err(|e| Error::ArgParse(e))?;

                    ctx
                } else {
                    Context::from_arg_matches(&clargs).map_err(|e| Error::ArgParse(e))?
                }
            }
        };

        Ok(context)
    }

    /// Returns reference to the path of the root directory to be traversed.
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }

    /// The sort-order used for printing.
    pub fn sort(&self) -> Option<SortType> {
        self.sort
    }

    /// Getter for `dirs_first` field.
    pub fn dirs_first(&self) -> bool {
        self.dirs_first
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn level(&self) -> Option<usize> {
        self.level
    }

    /// Ignore file overrides.
    pub fn overrides(&self) -> Result<Override, ignore::Error> {
        let mut builder = OverrideBuilder::new(self.dir());

        if self.ignore_git {
            builder.add("!.git/**/*")?;
        }

        if self.glob.is_empty() && self.iglob.is_empty() {
            return builder.build();
        }

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

#[derive(Debug)]
pub enum Error {
    ArgParse(ClapError),
    Config(ClapError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgParse(e) => write!(f, "{e}"),
            Self::Config(e) => write!(f, "A configuration file was found but failed to parse: {e}"),
        }
    }
}

impl StdError for Error {}
