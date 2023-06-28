use super::{config, error::Error, Context};
use clap::{
    builder::ArgAction, parser::ValueSource, ArgMatches, Command, CommandFactory, FromArgMatches,
};
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

/// Allows the implementor to compute [`ArgMatches`] that reconciles arguments from both the
/// command-line as well as the config file that gets loaded.
pub trait Reconciler: CommandFactory + FromArgMatches {
    /// Loads in arguments from both the command-line as well as the config file and reconciles
    /// identical arguments between the two using these rules:
    ///
    /// 1. If no config file is present, use arguments strictly from the command-line.
    /// 2. If an argument was provided via the CLI then override the argument from the config.
    /// 3. If an argument is sourced from its default value because a user didn't provide it via
    ///    the CLI, then select the argument from the config if it exists.
    fn compute_args() -> Result<ArgMatches, Error> {
        let cmd = Self::command().args_override_self(true);

        let user_args = Command::clone(&cmd).get_matches();

        if user_args.get_one::<bool>("no_config").is_some_and(|b| *b) {
            return Ok(user_args);
        }

        let maybe_config_args = {
            if let Some(rc) = load_rc_config_args() {
                Some(rc)
            } else {
                let named_table = user_args.get_one::<String>("config");

                load_toml_config_args(named_table.map(String::as_str))?
            }
        };

        let Some(config_args) = maybe_config_args else {
            return Ok(user_args);
        };

        let mut final_args = init_empty_args();

        for arg in cmd.get_arguments() {
            let arg_id = arg.get_id();
            let id_str = arg_id.as_str();

            if id_str == "dir" {
                if let Some(dir) = user_args.try_get_one::<PathBuf>(id_str)? {
                    final_args.push(OsString::from(dir));
                }
                continue;
            }

            let argument_source = user_args
                .value_source(id_str)
                .map_or(&config_args, |source| {
                    if matches!(source, ValueSource::CommandLine) {
                        &user_args
                    } else {
                        &config_args
                    }
                });

            let Some(key) = arg.get_long().map(|l| format!("--{l}")).map(OsString::from) else {
                continue
            };

            match arg.get_action() {
                ArgAction::SetTrue => {
                    if argument_source
                        .try_get_one::<bool>(id_str)?
                        .is_some_and(|b| *b)
                    {
                        final_args.push(key);
                    };
                },
                ArgAction::SetFalse => continue,
                _ => {
                    let Ok(Some(raw)) = argument_source.try_get_raw(id_str) else {
                        continue;
                    };
                    final_args.push(key);
                    final_args.extend(raw.map(OsStr::to_os_string));
                },
            }
        }

        Ok(cmd.get_matches_from(final_args))
    }
}

impl Reconciler for Context {}

/// Creates a properly formatted `Vec<OsString>` that [`clap::Command`] would understand.
#[inline]
fn init_empty_args() -> Vec<OsString> {
    vec![OsString::from("--")]
}

/// Loads an [`ArgMatches`] from `.erdtreerc`.
#[inline]
fn load_rc_config_args() -> Option<ArgMatches> {
    if let Some(rc_config) = config::rc::read_config_to_string() {
        let parsed_args = config::rc::parse(&rc_config);
        let config_args = Context::command().get_matches_from(parsed_args);

        return Some(config_args);
    }

    None
}

/// Loads an [`ArgMatches`] from `.erdtree.toml`.
#[inline]
fn load_toml_config_args(named_table: Option<&str>) -> Result<Option<ArgMatches>, Error> {
    if let Ok(toml_config) = config::toml::load() {
        let parsed_args = config::toml::parse(toml_config, named_table)?;
        let config_args = Context::command().get_matches_from(parsed_args);

        return Ok(Some(config_args));
    }

    Ok(None)
}
