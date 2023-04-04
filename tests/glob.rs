use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "*.txt", "tests/data"]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "!*.txt", "tests/data"]),
        indoc!(
            "
            data (143 B)
            ├─ dream_cycle
            ├─ lipsum
            └─ the_yellow_king (143 B)
               └─ cassildas_song.md (143 B)"
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
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn iglob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--iglob", "*.TXT", "tests/data"]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn glob_stdin() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use strip_ansi_escapes::strip as strip_ansi_escapes;
    let expected = indoc!(
        "
        data (304 B)
        ├─ dream_cycle
        ├─ lipsum
        ├─ nemesis.txt (161 B)
        └─ the_yellow_king (143 B)
           └─ cassildas_song.md (143 B)

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

    assert_eq!(
        String::from_utf8(strip_ansi_escapes(output.stdout).unwrap()).unwrap(),
        expected
    );
    assert!(output.status.success());
}
