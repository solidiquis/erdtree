#[test]
fn parse_toml() -> Result<(), Box<dyn std::error::Error>> {
    use config::{Config, File};
    use std::{ffi::OsString, io::Write};
    use tempfile::Builder;

    let mut config_file = Builder::new()
        .prefix(".erdtree")
        .suffix(".toml")
        .tempfile()?;

    let toml_contents = r#"
        icons = true
        human = true
        threads = 10

        [grogoroth]
        disk_usage = "block"
        icons = true
        human = false
        threads = 10
    "#;

    config_file.write(toml_contents.as_bytes())?;

    let file = config_file
        .path()
        .to_str()
        .and_then(|s| s.strip_suffix(".toml"))
        .map(File::with_name)
        .unwrap();

    let config = Config::builder().add_source(file).build()?;

    // TOP-LEVEL TABLE
    let mut toml = super::parse(config.clone(), None)?;

    let expected = vec![
        OsString::from("--"),
        OsString::from("--icons"),
        OsString::from("--human"),
        OsString::from("--threads"),
        OsString::from("10"),
    ];

    for (i, outer_item) in expected.iter().enumerate() {
        for j in 0..toml.len() {
            let inner_item = &toml[j];

            if outer_item == inner_item {
                toml.swap(i, j);
            }
        }
    }

    assert_eq!(toml.len(), expected.len());

    for (lhs, rhs) in toml.iter().zip(expected.iter()) {
        assert_eq!(lhs, rhs);
    }

    // NAMED-TABLE
    let mut toml = super::parse(config, Some("grogoroth"))?;

    let expected = vec![
        OsString::from("--"),
        OsString::from("--disk-usage"),
        OsString::from("block"),
        OsString::from("--icons"),
        OsString::from("--threads"),
        OsString::from("10"),
    ];

    for (i, outer_item) in expected.iter().enumerate() {
        for j in 0..toml.len() {
            let inner_item = &toml[j];

            if outer_item == inner_item {
                toml.swap(i, j);
            }
        }
    }

    assert_eq!(toml.len(), expected.len());

    for (lhs, rhs) in toml.iter().zip(expected.iter()) {
        assert_eq!(lhs, rhs);
    }

    Ok(())
}
