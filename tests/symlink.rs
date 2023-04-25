mod utils;

#[cfg(unix)]
mod test {
    use indoc::indoc;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn link() -> Result<(), Box<dyn std::error::Error>> {
        let tmp = TempDir::new()?;
        let target = Path::new("./tests/data/the_yellow_king").canonicalize()?;
        let link = tmp.path().join("the_yellow_link");

        symlink(target, &link)?;

        let link_canonical = link
            .canonicalize()
            .map(|c| c.to_string_lossy().into_owned())?;

        let out = super::utils::run_cmd(&["--sort", "name", "--follow", &link_canonical]);

        println!("{}", out);

        assert_eq!(
            out,
            indoc!(
                "143 B ┌─ cassildas_song.md
                143 B the_yellow_king

                1 file"
            ),
            "Failed to print symlink"
        );

        Ok(())
    }
}
