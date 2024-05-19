use ahash::HashMap;
use clap::{error::Error as ClapError, ArgMatches, CommandFactory};
use config::{Config, ConfigError};
use toml::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Deserialization(#[from] ConfigError),

    #[error("Table '#{0}' does not exist.")]
    TableNotFound(String),

    #[error("Error while parsing config arguments: {0}")]
    Parse(#[from] ClapError),
}

type Result<T> = std::result::Result<T, Error>;
type KeyTransformer = fn(String) -> String;
type TomlConfig = toml::map::Map<String, Value>;

pub fn args(conf: Config, table_name: Option<&str>) -> Result<Option<ArgMatches>> {
    let maybe_config = conf.try_deserialize::<toml::Table>().map(|raw_config| {
        deep_transform_keys(raw_config, |mut key| {
            key.make_ascii_lowercase();
            format!("--{}", key.replace('_', "-"))
        })
    })?;

    let Some(mut config) = maybe_config else {
        return Ok(None);
    };

    match table_name {
        Some(name) => match config.remove(&format!("--{name}")) {
            Some(Value::Table(sub_table)) => {
                config = sub_table;
            }
            _ => return Err(Error::TableNotFound(name.to_string())),
        },
        None => remove_sub_tables(&mut config),
    }

    let arg_matches = into_args(config)?;

    Ok(Some(arg_matches))
}

fn into_args(conf: TomlConfig) -> Result<ArgMatches> {
    let mut args = vec![crate::BIN_NAME.to_string()];

    for (arg_name, param) in conf {
        match param {
            Value::Array(array_args) => {
                for arg in array_args {
                    if let Some(farg) = fmt_arg(arg) {
                        args.push(arg_name.clone());
                        args.push(farg)
                    }
                }
            }
            Value::Boolean(arg) if arg => {
                args.push(arg_name.clone());
            }
            _ => {
                if let Some(farg) = fmt_arg(param) {
                    args.push(arg_name.clone());
                    args.push(farg)
                }
            }
        }
    }

    let cmd = crate::user::Context::command().try_get_matches_from(args)?;

    Ok(cmd)
}

/// Formats basic primitive types into OS args. Will ignore table and array types.
fn fmt_arg(val: Value) -> Option<String> {
    match val {
        Value::Float(p) => Some(p.to_string()),
        Value::String(p) => Some(p.to_string()),
        Value::Datetime(p) => Some(p.to_string()),
        Value::Integer(p) => Some(p.to_string()),
        _ => None,
    }
}

fn remove_sub_tables(conf: &mut TomlConfig) {
    let mut sub_table_keys = Vec::new();

    conf.iter().for_each(|(k, v)| {
        if let Value::Table(_) = v {
            sub_table_keys.push(k.clone())
        }
    });

    sub_table_keys.iter().for_each(|k| {
        conf.remove(k);
    });
}

/// Transforms all keys and nested keys to the format computed by `transformer`.
fn deep_transform_keys(toml: TomlConfig, transformer: KeyTransformer) -> Option<TomlConfig> {
    let mut dfs_stack_src = vec![Value::Table(toml)];
    let mut dfs_stack_dst = vec![("".to_string(), toml::map::Map::default())];

    let mut key_iters = HashMap::default();

    'outer: while !dfs_stack_src.is_empty() {
        let Some(Value::Table(current_node)) = dfs_stack_src.last_mut() else {
            continue;
        };

        let Some((dst_key, copy_dst)) = dfs_stack_dst.last_mut() else {
            continue;
        };

        let keys = key_iters
            .entry(dst_key.clone())
            .or_insert_with(|| current_node.keys().cloned().collect::<Vec<_>>().into_iter());

        for key in keys {
            match current_node.remove(&key) {
                Some(value) => match value {
                    Value::Table(_) => {
                        let transformed_key = transformer(key);
                        dfs_stack_dst.push((transformed_key, toml::map::Map::default()));
                        dfs_stack_src.push(value);
                        continue 'outer;
                    }
                    _ => {
                        let transformed_key = transformer(key);
                        copy_dst.insert(transformed_key, value);
                    }
                },
                None => continue,
            }
        }

        dfs_stack_src.pop();

        if let Some((transformed_key, current_map)) = dfs_stack_dst.pop() {
            if let Some((_, parent_map)) = dfs_stack_dst.last_mut() {
                parent_map.insert(transformed_key, Value::Table(current_map));
            } else {
                return Some(current_map);
            }
        }
    }

    None
}
