use crate::error::prelude::*;
use ahash::HashMap;
use clap::{parser::ValueSource, ArgMatches, Args, CommandFactory, FromArgMatches, Parser};
use std::{
    env,
    fs,
    path::PathBuf,
};

/// Enum definitions for enumerated command-line arguments.
pub mod args;

/// Concerned with properties of columns in the output which is essentially a 2D grid.
pub mod column;

/// Concerned with loading and parsing the optional `erdtree.toml` config file.
mod config;

#[cfg(test)]
mod test;

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

    /// Run the program ignoring hidden files
    #[arg(short = '.', long)]
    pub no_hidden: bool,

    /// Run the program skipping the .git directory
    #[arg(long)]
    pub no_git: bool,

    /// Report byte size in either binary or SI units
    #[arg(short, long, value_enum, default_value_t)]
    pub byte_units: args::BytePresentation,

    /// Use configuration of a named table rather than the top-level table in .erdtree.toml
    #[arg(short = 'c', long)]
    pub config: Option<String>,

    /// Sort directories before or after all other file types
    #[arg(short, long, value_enum, default_value_t)]
    pub dir_order: args::DirOrder,

    /// Filter for specified file types
    #[arg(short = 'F', long, value_enum)]
    pub file_type: Vec<args::FileType>,

    /// Follow symlinks
    #[arg(short = 'f', long)]
    pub follow: bool,

    /// Run the program ignoring files that match rules in all .gitignore files encountered during traversal
    #[arg(short = 'i', long)]
    pub gitignore: bool,

    /// Display file icons
    #[arg(short = 'I', long)]
    pub icons: bool,

    /// Ignore files that match rules in the global .gitignore file
    #[arg(long)]
    pub global_gitignore: bool,

    #[cfg(unix)]
    #[command(flatten)]
    pub long: Long,

    /// Maximum depth to display
    #[arg(short = 'L', long, value_name = "NUM")]
    pub level: Option<usize>,

    /// Metric used when reporting disk usage
    #[arg(short, long, value_enum, default_value_t)]
    pub metric: args::Metric,

    /// Run the program without reading .erdtree.toml
    #[arg(short, long)]
    pub no_config: bool,

    /// Hide the progress indicator
    #[arg(long)]
    pub no_progress: bool,

    #[command(flatten)]
    pub search: Search,

    /// Omit empty directories from the output
    #[arg(short = 'P', long)]
    pub prune: bool,

    /// Field whereby to sort entries
    #[arg(short, long, value_enum, default_value_t)]
    pub sort: args::Sort,

    /// Sort entries relative either to their siblings or all other entries
    #[arg(long, value_enum, default_value_t)]
    pub sort_type: args::SortType,

    /// Don't compute disk-usage and omit file size from output
    #[arg(short = 'S', long)]
    pub suppress_size: bool,

    /// Which kind of layout to use when rendering the output
    #[arg(short = 'y', long, value_enum, default_value_t)]
    pub layout: args::Layout,

    /// Number of threads to use for disk reads
    #[arg(short = 'T', long, default_value_t = Context::default_num_threads())]
    pub threads: usize,

    /// Prevent traversal into directories that are on different filesystems
    #[arg(short = 'x', long = "one-file-system")]
    pub same_fs: bool,

    #[arg(long)]
    /// Print completions for a given shell to stdout
    pub completions: Option<clap_complete::Shell>,

    //////////////////////////
    /* INTERNAL USAGE BELOW */
    //////////////////////////
    #[clap(skip = column::Metadata::default())]
    pub column_metadata: column::Metadata,

    #[cfg(debug_assertions)]
    #[arg(long)]
    // Output debug information at the end of the output
    pub debug: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Search {
    /// Regular expression (or glob if '--glob' or '--iglob' is used) used to match files by their
    /// relative path
    #[arg(short, long, group = "searching")]
    pub pattern: Option<String>,

    /// Enables glob based searching instead of regular expressions
    #[arg(long, requires = "searching")]
    pub glob: bool,

    /// Enables case-insensitive glob based searching instead of regular expressions
    #[arg(long, requires = "searching")]
    pub iglob: bool,
}

#[cfg(unix)]
#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
pub struct Long {
    /// Show extended metadata and attributes
    #[arg(short, long, group = "ls-long")]
    pub long: bool,

    /// Show file's groups
    #[arg(long, requires = "ls-long")]
    pub group: bool,

    /// Show each file's ino
    #[arg(long, requires = "ls-long")]
    pub ino: bool,

    /// Show the total number of hardlinks to the underlying inode
    #[arg(long, requires = "ls-long")]
    pub nlink: bool,

    /// Show permissions in numeric octal format instead of symbolic
    #[arg(long, requires = "ls-long")]
    pub octal: bool,

    /// Which kind of timestamp to use
    #[arg(long, value_enum, requires = "ls-long")]
    pub time: Option<args::TimeStamp>,

    /// Which format to use for the timestamp; default by default
    #[arg(long = "time-format", requires = "ls-long", value_enum)]
    pub time_format: Option<args::TimeFormat>,
}

impl Context {
    pub fn init() -> Result<Self> {
        let clargs = Self::command().get_matches();
        let user_config = Self::load_config(&clargs)?;

        let mut ctx = if let Some(ref config) = user_config {
            let reconciled_args = Self::reconcile_args(&clargs, config);
            Self::try_parse_from(reconciled_args).into_report(ErrorCategory::User)?
        } else {
            Self::from_arg_matches(&clargs).into_report(ErrorCategory::User)?
        };

        if let Some(dir_arg) = clargs.get_one::<PathBuf>("dir").cloned() {
            ctx.dir = Some(dir_arg) 
        } else {
            let current_dir = Self::get_current_dir()?;
            ctx.dir = Some(current_dir);
        }

        Ok(ctx)
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

    pub fn update_column_metadata(&mut self, new_metadata: column::Metadata) {
        self.column_metadata = new_metadata;
    }

    fn default_num_threads() -> usize {
        std::thread::available_parallelism().map_or(3, usize::from)
    }

    fn load_config(clargs: &ArgMatches) -> Result<Option<ArgMatches>> {
        let cmd = Self::from_arg_matches(clargs).into_report(ErrorCategory::User)?;

        if cmd.no_config {
            return Ok(None);
        }

        let Some(raw_config) = config::toml::load() else {
            return Ok(None);
        };

        match config::parse::args(raw_config, cmd.config.as_deref()) {
            Ok(config) => Ok(config),
            Err(err) => match err {
                config::parse::Error::TableNotFound(_) => Err(err).into_report(ErrorCategory::User),
                _ => Err(err).into_report(ErrorCategory::Internal),
            },
        }
    }

    /// Reconcile args between command-line and user config.
    fn reconcile_args(clargs: &ArgMatches, config: &ArgMatches) -> Vec<String> {
        let mut arg_id_map = HashMap::<clap::Id, clap::Arg>::default();

        for arg_def in Self::command().get_arguments() {
            if arg_def.is_positional() {
                continue;
            }
            arg_id_map.insert(arg_def.get_id().clone(), arg_def.clone());
        }

        let mut args = vec![crate::BIN_NAME.to_string()];

        let mut push_args = |arg_name: String, arg_id: &str, src: &ArgMatches| {
            if let Ok(Some(mut bool_args)) = src.try_get_many::<bool>(arg_id) {
                if bool_args.all(|arg| *arg) {
                    args.push(arg_name);
                }
                return;
            }

            let vals = src
                .get_raw_occurrences(arg_id)
                .unwrap()
                .flat_map(|i| {
                    i.map(|o| o.to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            args.push(arg_name);
            args.extend_from_slice(&vals);
        };

        for arg_id in arg_id_map.keys() {
            let arg_id_str = arg_id.as_str();

            let Some(arg_def) = arg_id_map.get(arg_id) else {
                continue;
            };

            let arg_name = arg_def.get_long().map_or_else(
                || arg_def.get_short().map(|c| format!("-{c}")).unwrap(),
                |long| format!("--{long}"),
            );

            let confarg_vs = config.value_source(arg_id_str);
            let clarg_vs = clargs.value_source(arg_id_str);

            match (clarg_vs, confarg_vs) {
                (None, None) => continue,
                (Some(_), None) => push_args(arg_name, arg_id_str, clargs),
                (None, Some(_)) => push_args(arg_name, arg_id_str, config),
                (Some(clarg), Some(conf)) => match (clarg, conf) {
                    // Prioritize config argument over default
                    (ValueSource::DefaultValue, ValueSource::CommandLine) => {
                        push_args(arg_name, arg_id_str, config)
                    },

                    // Prioritize user argument in all other cases
                    _ => push_args(arg_name, arg_id_str, clargs),
                },
            }
        }

        args.into_iter().collect::<Vec<_>>()
    }
}
