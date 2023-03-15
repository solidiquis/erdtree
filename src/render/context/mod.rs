use super::{
    disk_usage::{DiskUsage, PrefixKind},
    order::SortType,
};
use clap::{
    parser::ValueSource, ArgMatches, CommandFactory, Error as ClapError, FromArgMatches, Parser,
};
use ignore::overrides::{Override, OverrideBuilder};
use std::{
    convert::From,
    error::Error as StdError,
    ffi::{OsStr, OsString},
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
#[command(version = "1.5.1")]
#[command(about = "erdtree (et) is a multi-threaded filetree visualizer and disk usage analyzer.", long_about = None)]
pub struct Context {
    /// Root directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Print physical or logical file size
    #[arg(short, long, value_enum, default_value_t = DiskUsage::default())]
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

    /// Display disk usage as binary or SI units
    #[arg(short, long, value_enum, default_value_t = PrefixKind::default())]
    pub prefix: PrefixKind,

    /// Disable printing of empty branches
    #[arg(short = 'P', long)]
    pub prune: bool,

    /// Sort-order to display directory content
    #[arg(short, long, value_enum, default_value_t = SortType::default())]
    sort: SortType,

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

    /// Show the size on the left, decimal aligned
    #[arg(long)]
    pub size_left: bool,

    /// Don't read configuration file
    #[arg(long)]
    pub no_config: bool,
}

impl Context {
    /// Initializes [Context], optionally reading in the configuration file to override defaults.
    /// Arguments provided will take precedence over config.
    pub fn init() -> Result<Self, Error> {
        let user_args = Context::command().args_override_self(true).get_matches();

        let no_config = user_args
            .get_one("no_config")
            .map(bool::clone)
            .unwrap_or(false);

        if no_config {
            return Context::from_arg_matches(&user_args).map_err(|e| Error::ArgParse(e));
        }

        if let Some(ref config) = config::read_config_to_string::<&str>(None) {
            let raw_config_args = config::parse_config(config);
            let config_args = Context::command().get_matches_from(raw_config_args);

            // If the user did not provide any arguments just read from config.
            if !user_args.args_present() {
                return Context::from_arg_matches(&config_args).map_err(|e| Error::Config(e));
            }

            // If the user did provide arguments we need to reconcile between config and
            // user arguments.
            let mut args = vec![OsString::from("--")];

            // Used to pick either from config or user args.
            let mut pick_args_from = |id: &str, matches: &ArgMatches| {
                if let Ok(Some(raw)) = matches.try_get_raw(id) {
                    let kebap = id.replace("_", "-");

                    let raw_args = raw
                        .map(OsStr::to_owned)
                        .map(|s| vec![OsString::from(format!("--{}", kebap)), s])
                        .filter(|pair| pair[1] != "false")
                        .flatten()
                        .filter(|s| s != "true")
                        .collect::<Vec<OsString>>();

                    args.extend(raw_args);
                }
            };

            for id in user_args.ids() {
                let id_str = id.as_str();

                // Don't look at me... my shame..
                if id_str == "Context" {
                    continue;
                }

                if let Some(user_arg) = user_args.value_source(id_str) {
                    match user_arg {
                        // prioritize the user arg if user provided a command line argument
                        ValueSource::CommandLine => pick_args_from(id_str, &user_args),

                        // otherwise prioritize argument from the config
                        _ => pick_args_from(id_str, &config_args),
                    }
                }
            }

            println!("{:?}", args);

            let clargs = Context::command().get_matches_from(args);
            return Context::from_arg_matches(&clargs).map_err(|e| Error::Config(e));
        }

        Context::from_arg_matches(&user_args).map_err(|e| Error::ArgParse(e))
    }

    /// Returns reference to the path of the root directory to be traversed.
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }

    /// The sort-order used for printing.
    pub fn sort(&self) -> SortType {
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
            builder.add("!.git")?;
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
