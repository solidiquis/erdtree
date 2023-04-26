use indoc::indoc;
use std::{env::current_dir, error::Error, fs};

mod utils;

#[test]
fn hardlink() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir()?;

    let src = current_dir
        .join("tests")
        .join("hardlinks")
        .join("kadath.txt");

    let link = current_dir
        .join("tests")
        .join("hardlinks")
        .join("curwin.hpl");

    fs::hard_link(src, &link)?;

    let out = utils::run_cmd(&["tests/hardlinks"]);

    fs::remove_file(&link)?;

    assert_eq!(
        out,
        indoc!(
            "157 B ┌─ kadath.txt
            157 B ├─ curwin.hpl
            157 B hardlinks

            2 files"
        )
    );

    Ok(())
}
