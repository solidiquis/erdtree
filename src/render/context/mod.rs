use super::disk_usage::{file_size::DiskUsage, units::PrefixKind};
use crate::tty;
use clap::{parser::ValueSource, ArgMatches, CommandFactory, FromArgMatches, Id, Parser};
use error::Error;
use file::FileType;
use ignore::{
    overrides::{Override, OverrideBuilder},
    DirEntry,
};
use regex::Regex;
use sort::SortType;
use std::{
    borrow::Borrow,
    convert::From,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

/// Operations to load in defaults from configuration file.
pub mod config;

/// [Context] related errors.
pub mod error;

/// Common cross-platform file-types.
pub mod file;

/// Printing order kinds.
pub mod sort;

/// Different types of timestamps available in long view.
#[cfg(unix)]
pub mod time;

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
    /// Directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,

    /// Turn on colorization always
    #[arg(short = 'C', long)]
    pub force_color: bool,

    /// Print physical or logical file size
    #[arg(short, long, value_enum, default_value_t = DiskUsage::default())]
    pub disk_usage: DiskUsage,

    /// Show hidden files
    #[arg(short = '.', long)]
    pub hidden: bool,

    /// Disable traversal of .git directory when traversing hidden files
    #[arg(long, requires = "hidden")]
    pub no_git: bool,

    /// Do not respect .gitignore files
    #[arg(short = 'i', long)]
    pub no_ignore: bool,

    /// Print disk usage information in plain format without the ASCII tree
    #[arg(short = 'F', long)]
    pub flat: bool,

    /// Print human-readable disk usage in flat display
    #[arg(long, requires = "flat")]
    pub human: bool,

    /// Follow symlinks
    #[arg(short = 'f', long)]
    pub follow: bool,

    /// Display file icons
    #[arg(short = 'I', long)]
    pub icons: bool,

    /// Maximum depth to display
    #[arg(short = 'L', long, value_name = "NUM")]
    level: Option<usize>,

    /// Show extended metadata and attributes
    #[cfg(unix)]
    #[arg(short, long)]
    pub long: bool,

    /// Show permissions in numeric octal format instead of symbolic
    #[cfg(unix)]
    #[arg(long, requires = "long")]
    pub octal: bool,

    /// Which kind of timestamp to use; modified by default
    #[cfg(unix)]
    #[arg(long, value_enum, requires = "long")]
    pub time: Option<time::Stamp>,

    /// Regular expression (or glob if '--glob' or '--iglob' is used) used to match files
    #[arg(short, long)]
    pub pattern: Option<String>,

    /// Enables glob based searching
    #[arg(long, requires = "pattern")]
    pub glob: bool,

    /// Enables case-insensitive glob based searching
    #[arg(long, requires = "pattern")]
    pub iglob: bool,

    /// Restrict regex or glob search to a particular file-type
    #[arg(short = 't', long, requires = "pattern", value_enum)]
    pub file_type: Option<FileType>,

    /// Remove empty directories from output
    #[arg(short = 'P', long)]
    pub prune: bool,

    /// Total number of digits after the decimal to display for disk usage
    #[arg(short = 'n', long, default_value_t = 2, value_name = "NUM")]
    pub scale: usize,

    /// Sort-order to display directory content
    #[arg(short, long, value_enum, default_value_t = SortType::default())]
    pub sort: SortType,

    /// Sort directories above files
    #[arg(long)]
    pub dirs_first: bool,

    /// Number of threads to use
    #[arg(short = 'T', long, default_value_t = 3)]
    pub threads: usize,

    /// Report disk usage in binary or SI units
    #[arg(short, long, value_enum, default_value_t = PrefixKind::default())]
    pub unit: PrefixKind,

    #[arg(long)]
    /// Print completions for a given shell to stdout
    pub completions: Option<clap_complete::Shell>,

    /// Only print directories
    #[arg(long)]
    pub dirs_only: bool,

    /// Print plainly without ANSI escapes
    #[arg(long)]
    pub no_color: bool,

    /// Don't read configuration file
    #[arg(long)]
    pub no_config: bool,

    /// Omit disk usage from output
    #[arg(long)]
    pub suppress_size: bool,

    //////////////////////////
    /* INTERNAL USAGE BELOW */
    //////////////////////////
    /// Is stdin in a tty?
    #[clap(skip = tty::stdin_is_tty())]
    pub stdin_is_tty: bool,

    /// Is stdin in a tty?
    #[clap(skip = tty::stdout_is_tty())]
    pub stdout_is_tty: bool,

    /// Restricts column width of disk usage
    #[clap(skip = usize::default())]
    pub max_du_width: usize,

    /// Restricts column width of nlink for long view.
    #[clap(skip = usize::default())]
    #[cfg(unix)]
    pub max_nlink_width: usize,

    /// Restricts column width of ino for long view.
    #[clap(skip = usize::default())]
    #[cfg(unix)]
    pub max_ino_width: usize,

    /// Restricts column width of block for long view.
    #[clap(skip = usize::default())]
    #[cfg(unix)]
    pub max_block_width: usize,
}

impl Context {
    /// Initializes [Context], optionally reading in the configuration file to override defaults.
    /// Arguments provided will take precedence over config.
    pub fn init() -> Result<Self, Error> {
        let user_args = Self::command().args_override_self(true).get_matches();

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

    /// Determines whether or not it's appropriate to display color in output based on
    /// `--no-color`, `--force-color`, and whether or not stdout is connected to a tty.
    ///
    /// If `--force-color` is `true` then this will always evaluate to `false`.
    pub const fn no_color(&self) -> bool {
        if self.force_color {
            return false;
        }

        self.no_color || !self.stdout_is_tty
    }

    /// Returns [Path] of the root directory to be traversed.
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }

    /// Returns canonical [Path] of the root directory to be traversed.
    pub fn dir_canonical(&self) -> PathBuf {
        std::fs::canonicalize(self.dir()).unwrap_or_else(|_| self.dir().to_path_buf())
    }

    /// The max depth to print. Note that all directories are fully traversed to compute file
    /// sizes; this just determines how much to print.
    pub fn level(&self) -> usize {
        self.level.unwrap_or(usize::MAX)
    }

    /// Which timestamp type to use for long view; defaults to modified.
    pub fn time(&self) -> time::Stamp {
        self.time.unwrap_or_default()
    }

    /// Which filetype to filter on; defaults to regular file.
    pub fn file_type(&self) -> FileType {
        self.file_type.unwrap_or_default()
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

    /// Predicate used for filtering via regular expressions and file-type. When matching regular
    /// files, directories will always be included since matched files will need to be bridged back
    /// to the root node somehow. Empty sets not producing an output is handled by [`Tree`].
    ///
    /// [`Tree`]: crate::render::tree::Tree
    pub fn regex_predicate(
        &self,
    ) -> Result<Box<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>, Error> {
        let Some(pattern) = self.pattern.as_ref() else {
            return Err(Error::PatternNotProvided);
        };

        let re = Regex::new(pattern)?;

        let file_type = self.file_type();

        match file_type {
            FileType::Dir => Ok(Box::new(move |dir_entry: &DirEntry| {
                let is_dir = dir_entry.file_type().map_or(false, |ft| ft.is_dir());

                if is_dir {
                    // Problem right here.
                    return Self::ancestor_regex_match(dir_entry.path(), &re, 0);
                }

                Self::ancestor_regex_match(dir_entry.path(), &re, 1)
            })),

            _ => Ok(Box::new(move |dir_entry: &DirEntry| {
                let entry_type = dir_entry.file_type();
                let is_dir = entry_type.map_or(false, |ft| ft.is_dir());

                if is_dir {
                    return true;
                }

                match file_type {
                    FileType::File if entry_type.map_or(true, |ft| !ft.is_file()) => return false,
                    FileType::Link if entry_type.map_or(true, |ft| !ft.is_symlink()) => {
                        return false
                    }
                    _ => (),
                }
                let file_name = dir_entry.file_name().to_string_lossy();
                re.is_match(&file_name)
            })),
        }
    }

    /// Predicate used for filtering via globs and file-types.
    pub fn glob_predicate(
        &self,
    ) -> Result<Box<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>, Error> {
        let mut builder = OverrideBuilder::new(self.dir());

        let mut negated_glob = false;

        let overrides = if !self.glob && !self.iglob {
            // Shouldn't really ever be hit but placing here as a safeguard.
            return Err(Error::EmptyGlob);
        } else {
            if self.iglob {
                builder.case_insensitive(true)?;
            }

            if let Some(ref glob) = self.pattern {
                let trim = glob.trim_start();
                negated_glob = trim.starts_with('!');

                if negated_glob {
                    builder.add(trim.trim_start_matches('!'))?;
                } else {
                    builder.add(trim)?;
                }
            }

            builder.build()?
        };

        let file_type = self.file_type();

        match file_type {
            FileType::Dir => Ok(Box::new(move |dir_entry: &DirEntry| {
                let is_dir = dir_entry.file_type().map_or(false, |ft| ft.is_dir());

                if is_dir {
                    return Self::ancestor_glob_match(dir_entry.path(), &overrides, 0);
                }
                let matched = Self::ancestor_glob_match(dir_entry.path(), &overrides, 1);

                if negated_glob {
                    !matched
                } else {
                    matched
                }
            })),

            _ => Ok(Box::new(move |dir_entry: &DirEntry| {
                let entry_type = dir_entry.file_type();
                let is_dir = entry_type.map_or(false, |ft| ft.is_dir());

                if is_dir {
                    return true;
                }

                match file_type {
                    FileType::File if entry_type.map_or(true, |ft| !ft.is_file()) => return false,
                    FileType::Link if entry_type.map_or(true, |ft| !ft.is_symlink()) => {
                        return false
                    }
                    _ => (),
                }

                let matched = overrides.matched(dir_entry.path(), false);

                if negated_glob {
                    !matched.is_whitelist()
                } else {
                    matched.is_whitelist()
                }
            })),
        }
    }

    /// Special override to toggle the visibility of the git directory.
    pub fn no_git_override(&self) -> Result<Override, Error> {
        let mut builder = OverrideBuilder::new(self.dir());

        if self.no_git {
            builder.add("!.git")?;
        }

        Ok(builder.build()?)
    }

    /// Setter for `max_du_width` to inform formatters what the width of the disk usage column
    /// should be.
    pub fn set_max_du_width(&mut self, width: usize) {
        self.max_du_width = width;
    }

    /// Setter for `max_nlink_width` which is used to inform formatters what the width of the
    /// `nlink` column should be.
    #[cfg(unix)]
    pub fn set_max_nlink_width(&mut self, width: usize) {
        self.max_nlink_width = width;
    }

    /// Setter for `max_ino_width` which is used to inform formatters what the width of the
    /// `ino` column should be.
    #[cfg(unix)]
    pub fn set_max_ino_width(&mut self, width: usize) {
        self.max_ino_width = width;
    }

    /// Setter for `max_block_width` which is used to inform formatters what the width of the
    /// `block` column should be.
    #[cfg(unix)]
    pub fn set_max_block_width(&mut self, width: usize) {
        self.max_block_width = width;
    }

    /// Do any of the components of a path match the provided glob? This is used for ensuring that
    /// all children of a directory that a glob targets gets captured.
    #[inline]
    fn ancestor_glob_match(path: &Path, ovr: &Override, skip: usize) -> bool {
        path.components()
            .rev()
            .skip(skip)
            .any(|c| ovr.matched(c, false).is_whitelist())
    }

    /// Like [Self::ancestor_glob_match] except uses [Regex] rather than [Override].
    #[inline]
    fn ancestor_regex_match(path: &Path, re: &Regex, skip: usize) -> bool {
        path.components()
            .rev()
            .skip(skip)
            .any(|comp| re.is_match(comp.as_os_str().to_string_lossy().borrow()))
    }
}
