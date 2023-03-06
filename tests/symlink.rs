mod utils;

#[cfg(unix)]
mod test {
    use indoc::indoc;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn link() -> Result<(), Box<dyn std::error::Error>> {
        let tmp = TempDir::new("hastur")?;
        let target = Path::new("./tests/data/the_yellow_king").canonicalize()?;
        let link = tmp.path().join("the_yellow_link");

        symlink(target, &link)?;

        let link_canonical = link
            .canonicalize()
            .map(|c| c.to_string_lossy().into_owned())?;

        assert_eq!(
            super::utils::run_cmd(&["--sort", "name", "--follow-links", &link_canonical]),
            indoc!(
                "
                the_yellow_king (143 B)
                └─ cassildas_song.md (143 B)"
            ),
            "Failed to print symlink"
        );

        Ok(())
    }
}
