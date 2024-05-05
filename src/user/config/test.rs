use super::parse;
use crate::user::args::{Layout, Metric};
use config::{Config, File};

#[test]
fn test_toml_parse_top_level_table() {
    let config = load_example();

    let arg_matches = parse::args(config, None)
        .expect("Failed to parse example config.")
        .expect("Expected top level table to be found.");

    let icons = arg_matches.get_one::<bool>("icons").unwrap();
    assert!(icons);
}

#[test]
fn test_toml_parse_sub_table() {
    let config = load_example();

    let arg_matches = parse::args(config, Some("du"))
        .expect("Failed to parse example config.")
        .expect("Expected sub table to be found.");

    let metric = arg_matches.get_one::<Metric>("metric").unwrap();
    assert_eq!(metric, &Metric::Block);

    let layout = arg_matches.get_one::<Layout>("layout").unwrap();
    assert_eq!(layout, &Layout::Flat);

    let level = arg_matches.get_one::<usize>("level").unwrap();
    assert_eq!(*level, 1);
}

fn load_example() -> Config {
    let example_config = std::env::current_dir()
        .ok()
        .map(|p| p.join("example").join(super::ERDTREE_CONFIG_TOML))
        .and_then(|p| p.as_path().to_str().map(File::with_name))
        .unwrap();

    Config::builder()
        .add_source(example_config)
        .build()
        .expect("Failed to load example config.")
}
