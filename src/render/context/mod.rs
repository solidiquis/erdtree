use super::disk_usage::{file_size::DiskUsage, units::PrefixKind};
use crate::tty;
use clap::{
    ArgMatches,
    CommandFactory,
    FromArgMatches,
    Id,
    Parser,
    parser::ValueSource,
};
use error::Error;
use ignore::{
    overrides::{Override, OverrideBuilder},
    DirEntry
};
use regex::Regex;
use sort::SortType;
use std::{
    convert::From,
    ffi::{OsStr, OsString},
    io::{stdin, BufRead},
    path::{Path, PathBuf},
};

/// Operations to load in defaults from configuration file.
pub mod config;

/// [Context] related errors.
pub mod error;

/// Printing order kinds.
pub mod sort;

/// Unit tests for [Context]
#[cfg(test)]
mod test;

/// Defines the CLI.
#[derive(Parser, Debug)]
#[command(name = "erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "1.8.1")]
#[command(about = "erdtree (et) is a multi-threaded file-tree visualization and disk usage analysis tool.", long_about = None)]
pub struct Context {
    /// Root directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Print physical or logical file size
    #[arg(short, long, value_enum, default_value_t = DiskUsage::default())]
    pub disk_usage: DiskUsage,

    /// Regular expression (or a glob if '--glob' is used) used to match files
    #[arg(short, long)]
    pub pattern: Option<String>,

    /// Enables glob based searching
    #[arg(long, requires = "pattern")]
    pub glob: bool,

    /// Enables case-insensitive glob based searching
    #[arg(long, requires = "pattern")]
    pub iglob: bool,

    /// Show hidden files
    #[arg(short = 'H', long)]
    pub hidden: bool,

    /// Disable traversal of .git directory when traversing hidden files
    #[arg(long, requires = "hidden")]
    pub ignore_git: bool,

    /// Display file icons
    #[arg(short = 'I', long)]
    pub icons: bool,

    /// Ignore .gitignore
    #[arg(short, long)]
    pub ignore_git_ignore: bool,

    /// Maximum depth to display
    #[arg(short, long, value_name = "NUM")]
    level: Option<usize>,

    /// Total number of digits after the decimal to display for disk usage
    #[arg(short = 'n', long, default_value_t = 2, value_name = "NUM")]
    pub scale: usize,

    /// Disable printing of empty branches
    #[arg(short = 'P', long)]
    pub prune: bool,

    /// Print disk usage information in plain format without ASCII tree
    #[arg(short, long)]
    pub report: bool,

    /// Print human-readable disk usage in report
    #[arg(long, requires = "report")]
    pub human: bool,

    /// Print file-name in report as opposed to full path
    #[arg(long, requires = "report")]
    pub file_name: bool,

    /// Sort-order to display directory content
    #[arg(short, long, value_enum, default_value_t = SortType::default())]
    pub sort: SortType,

    /// Always sorts directories above files
    #[arg(long)]
    pub dirs_first: bool,

    /// Display disk usage as binary or SI units
    #[arg(short, long, value_enum, default_value_t = PrefixKind::default())]
    pub unit: PrefixKind,

    /// Traverse symlink directories and consider their disk usage
    #[arg(short = 'S', long)]
    pub follow_links: bool,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 3)]
    pub threads: usize,

    #[arg(long)]
    /// Print completions for a given shell to stdout
    pub completions: Option<clap_complete::Shell>,

    /// Only print directories
    #[arg(long)]
    pub dirs_only: bool,

    /// Omit disk usage from output
    #[arg(long)]
    pub suppress_size: bool,

    /// Print plainly without ANSI escapes
    #[arg(long)]
    pub no_color: bool,

    /// Don't read configuration file
    #[arg(long)]
    pub no_config: bool,

    #[clap(skip = tty::stdin_is_tty())]
    pub stdin_is_tty: bool,

    #[clap(skip = tty::stdout_is_tty())]
    pub stdout_is_tty: bool,
}

impl Context {
    /// Initializes [Context], optionally reading in the configuration file to override defaults.
    /// Arguments provided will take precedence over config.
    pub fn init() -> Result<Self, Error> {
        let mut args: Vec<_> = std::env::args().collect();

        // If there's input on stdin we add each line as a separate glob pattern
        if !tty::stdin_is_tty() {
            stdin()
                .lock()
                .lines()
                .filter_map(Result::ok)
                .filter(|l| !l.is_empty())
                .for_each(|line| {
                    args.push("--glob".into());
                    args.push(line);
                });
        }

        let user_args = Self::command()
            .args_override_self(true)
            .get_matches_from(args);

        let no_config = user_args
            .get_one::<bool>("no_config")
            .copied()
            .unwrap_or(false);

        if no_config {
            return Self::from_arg_matches(&user_args).map_err(Error::ArgParse);
        }

        if let Some(ref config) = config::read_config_to_string::<&str>(None) {
            let raw_config_args = config::parse(config);
            let config_args = Self::command().get_matches_from(raw_config_args);

            // If the user did not provide any arguments just read from config.
            if !user_args.args_present() {
                return Self::from_arg_matches(&config_args).map_err(Error::Config);
            }

            // If the user did provide arguments we need to reconcile between config and
            // user arguments.
            let mut args = vec![OsString::from("--")];

            let mut ids = user_args.ids().map(Id::as_str).collect::<Vec<&str>>();

            ids.extend(config_args.ids().map(Id::as_str).collect::<Vec<&str>>());

            ids = crate::utils::uniq(ids);

            for id in ids {
                if id == "Context" {
                    continue;
                }
                if id == "dir" {
                    if let Ok(Some(raw)) = user_args.try_get_raw(id) {
                        let raw_args = raw.map(OsStr::to_owned).collect::<Vec<OsString>>();

                        args.extend(raw_args);
                        continue;
                    }
                }

                if let Some(user_arg) = user_args.value_source(id) {
                    match user_arg {
                        // prioritize the user arg if user provided a command line argument
                        ValueSource::CommandLine => Self::pick_args_from(id, &user_args, &mut args),

                        // otherwise prioritize argument from the config
                        _ => Self::pick_args_from(id, &config_args, &mut args),
                    }
                } else {
                    Self::pick_args_from(id, &config_args, &mut args);
                }
            }

            let clargs = Self::command().get_matches_from(args);
            return Self::from_arg_matches(&clargs).map_err(Error::Config);
        }

        Self::from_arg_matches(&user_args).map_err(Error::ArgParse)
    }

    /// Determines whether or not it's appropriate to display color in output based on `--no-color`
    /// and whether or not stdout is connected to a tty.
    pub const fn no_color(&self) -> bool {
        self.no_color || !self.stdout_is_tty
    }

    /// Returns reference to the path of the root directory to be traversed.
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn level(&self) -> usize {
        self.level.unwrap_or(usize::MAX)
    }

    /// Ignore file overrides.
    pub fn overrides(&self) -> Result<Override, Error> {
        let mut builder = OverrideBuilder::new(self.dir());

        if self.ignore_git {
            builder.add("!.git")?;
        }

        if !self.glob && !self.iglob {
            let builder = builder.build()?;
            return Ok(builder);
        }

        if self.iglob {
            builder.case_insensitive(true).unwrap();
        }

        if let Some(ref p) = self.pattern {
            builder.add(p)?;
        }

        let builder = builder.build()?;

        Ok(builder)
    }

    /// Used to pick either from config or user args when constructing [Context].
    fn pick_args_from(id: &str, matches: &ArgMatches, args: &mut Vec<OsString>) {
        if let Ok(Some(raw)) = matches.try_get_raw(id) {
            let kebap = id.replace('_', "-");

            let raw_args = raw
                .map(OsStr::to_owned)
                .map(|s| vec![OsString::from(format!("--{kebap}")), s])
                .filter(|pair| pair[1] != "false")
                .flatten()
                .filter(|s| s != "true")
                .collect::<Vec<OsString>>();

            args.extend(raw_args);
        }
    }

    /// Returns a closure that is used to determine if a non-directory directory entry matches the
    /// provided regular expression. If there is a match then that entry will be included in the
    /// output.
    pub fn regex_predicate(
        &self,
    ) -> Result<Box<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>, Error> {
        if self.iglob || self.glob {
            return Err(Error::RegexDisabled);
        }

        let Some(pattern) = self.pattern.as_ref() else {
            return Err(Error::PatternNotProvided);
        };

        let re = Regex::new(pattern)?;

        Ok(Box::new(move |dir_entry: &DirEntry| {
            if dir_entry.file_type().map_or(false, |ft| ft.is_dir()) {
                return true;
            }

            let file_name = dir_entry.file_name().to_string_lossy();
            re.is_match(&file_name)
        }))
    }
}
