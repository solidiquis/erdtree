use super::Context;
use crate::user::config;
use ::config::{Config, File, FileFormat};
use clap::{CommandFactory, Parser};

const MOCK_CONFIG_TOML: &'static str = r#"
icons = true
no-git = true
threads = 3

[du]
metric = "block"
icons = true
layout = "flat"
level = 1

# Do as `ls -l`
[ls]
icons = true
level = 1
suppress-size = true
long = true
gitignore = true
no_hidden = true

# How many lines of Rust are in this code base?
[rs]
metric = "line"
level = 1
pattern = "\\.rs$"
"#;

#[test]
fn test_reconcile_arg_matches_top_level_table() {
    let user_args = vec![
        crate::BIN_NAME.to_string(),
        "--threads".to_string(),
        "6".to_string(),
    ];
    let clargs = Context::command().get_matches_from(user_args);

    let config = load_config(MOCK_CONFIG_TOML);
    let config_args = config::parse::args(config, None).unwrap().unwrap();

    let args = Context::reconcile_args(&clargs, &config_args);
    let ctx = Context::try_parse_from(args).unwrap();

    // Config args
    assert!(ctx.icons);
    assert!(ctx.no_git);

    // Default args
    assert!(matches!(
        ctx.byte_units,
        crate::user::args::BytePresentation::Raw
    ));
    assert!(!ctx.follow);
    assert!(!ctx.no_hidden);

    // User args takes precedence over config
    assert_eq!(ctx.threads, 6)
}

#[test]
fn test_reconcile_arg_matches_sub_table_du() {
    let user_args = vec![crate::BIN_NAME.to_string()];
    let clargs = Context::command().get_matches_from(user_args);

    let config = load_config(MOCK_CONFIG_TOML);
    let config_args = config::parse::args(config, Some("du")).unwrap().unwrap();

    let args = Context::reconcile_args(&clargs, &config_args);
    let ctx = Context::try_parse_from(args).unwrap();

    // Config args
    assert!(matches!(ctx.metric, crate::user::args::Metric::Block));
    assert!(ctx.icons);
    assert!(matches!(ctx.layout, crate::user::args::Layout::Flat));
    assert_eq!(ctx.level, Some(1));

    // Default args
    assert!(matches!(
        ctx.byte_units,
        crate::user::args::BytePresentation::Raw
    ));
    assert!(!ctx.follow);
    assert!(!ctx.no_hidden);
}

#[test]
fn test_reconcile_arg_matches_sub_table_rs() {
    let user_args = vec![crate::BIN_NAME.to_string()];
    let clargs = Context::command().get_matches_from(user_args);

    let config = load_config(MOCK_CONFIG_TOML);
    let config_args = config::parse::args(config, Some("rs")).unwrap().unwrap();

    let args = Context::reconcile_args(&clargs, &config_args);
    let ctx = Context::try_parse_from(args).unwrap();

    // Config args
    assert!(matches!(ctx.metric, crate::user::args::Metric::Line));
    assert_eq!(ctx.level, Some(1));
    assert_eq!(ctx.search.pattern, Some("\\.rs$".to_string()));

    // Default args
    assert!(!ctx.follow);
    assert!(!ctx.no_hidden);
    assert!(!ctx.icons);
}

fn load_config(config: &str) -> Config {
    Config::builder()
        .add_source(File::from_str(config, FileFormat::Toml))
        .build()
        .unwrap()
}
