use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "*.txt", "tests/data"]),
        indoc!(
            "1.07 KiB data
              308   B ├─ dream_cycle
              308   B │  └─ polaris.txt
              446   B ├─ lipsum
              446   B │  └─ lipsum.txt
               83   B ├─ necronomicon.txt
              161   B ├─ nemesis.txt
              100   B ├─ nylarlathotep.txt
                      └─ the_yellow_king

           3 directories, 5 files"
        )
    );
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "!*.txt", "tests/data"]),
        indoc!(
          "143   B data
                   ├─ dream_cycle
                   ├─ lipsum
           143   B └─ the_yellow_king
           143   B    └─ cassildas_song.md

        3 directories, 1 file"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "*.TXT",
            "--glob-case-insensitive",
            "tests/data"
        ]),
        indoc!(
             "1.07 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B ├─ nylarlathotep.txt
                       └─ the_yellow_king

            3 directories, 5 files"
        )
    )
}

#[test]
fn iglob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--iglob", "*.TXT", "tests/data"]),
        indoc!(
             "1.07 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B ├─ nylarlathotep.txt
                       └─ the_yellow_king

            3 directories, 5 files"
        )
    )
}

#[test]
fn glob_stdin() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use strip_ansi_escapes::strip as strip_ansi_escapes;
    let expected = indoc!(
      "304   B data
               ├─ dream_cycle
               ├─ lipsum
       161   B ├─ nemesis.txt
       143   B └─ the_yellow_king
       143   B    └─ cassildas_song.md

    3 directories, 2 files
    "
    );
    let stdin = String::from("cassildas_song.md\nnemesis.txt\n");

    let cmd = Command::new("cargo")
        .args([
            "run",
            "--",
            "--threads",
            "1",
            "--no-config",
            "--sort",
            "name",
            "tests/data",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    write!(cmd.stdin.as_ref().unwrap(), "{}", stdin).unwrap();
    let output = cmd.wait_with_output().unwrap();

    let out = String::from_utf8(strip_ansi_escapes(output.stdout).unwrap()).unwrap();

    assert_eq!(out.trim_start(), expected);
    assert!(output.status.success());
}
