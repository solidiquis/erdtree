use crate::render::order::SortType;
use clap::{CommandFactory, FromArgMatches};

use super::{config, Context};

const TEST_CONFIG: &str = "./tests/data/.erdtreerc";

#[test]
fn config() {
    let context = context_from_config().expect("Failed to build Context from config path");

    let level = context.level().expect("'level' should not be 'None'");

    assert_eq!(level, 1, "Failed to properly read 'level' from config");

    let sort = context.sort();

    assert_eq!(
        sort,
        SortType::Size,
        "Failed to properly read 'sort' from config"
    );

    let icons = context.icons;

    assert!(icons, "Failed to properly read 'icons' from config");
}

fn context_from_config() -> Option<Context> {
    config::read_config_to_string(Some(TEST_CONFIG))
        .as_ref()
        .and_then(|config| {
            let raw_config_args = config::parse(config);
            let config_args = Context::command().get_matches_from(raw_config_args);
            Context::from_arg_matches(&config_args).ok()
        })
}
