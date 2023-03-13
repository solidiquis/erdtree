use crate::render::order::SortType;
use clap::{CommandFactory, FromArgMatches};

use super::{config, Context};

const TEST_CONFIG: &str = "./tests/data/.erdtreerc";

#[test]
fn config() {
    let context = context_from_config().expect("Failed to build Context from config path");

    let level = context.level().expect("'level' should not be 'None'");

    assert_eq!(level, 1, "Failed to properly read 'level' from config");

    let sort = context
        .sort()
        .expect("Failed to properly read 'sort' from config");

    assert_eq!(
        sort,
        SortType::Size,
        "Failed to properly read 'sort' from config"
    );

    let icons = context.icons;

    assert!(icons, "Failed to propertly read 'icons' from config")
}

fn context_from_config() -> Option<Context> {
    if let Some(ref config) = config::read_config_to_string(Some(TEST_CONFIG)) {
        let raw_config_args = config::parse_config(config);
        let config_args = Context::command().get_matches_from(raw_config_args);
        Context::from_arg_matches(&config_args).ok()
    } else {
        None
    }
}
