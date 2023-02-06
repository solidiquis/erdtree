use indoc::indoc;
use std::process::Command;
use std::process::Stdio;
use strip_ansi_escapes::strip as strip_ansi_escapes;

fn run_cmd(args: &[&str]) -> String {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("tests/data");

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    assert!(output.status.success());

    String::from_utf8(strip_ansi_escapes(output.stdout).unwrap())
        .unwrap()
        .trim()
        .to_string()
}

#[test]
fn glob() {
    assert_eq!(
        run_cmd(&["--glob", "*.txt"]),
        indoc!(
            "
            data (10.00 B)
            ├─ b.txt (5.00 B)
            ├─ a.txt (5.00 B)
            └─ nested"
        )
    )
}

#[test]
fn glob_negative() {
    assert_eq!(
        run_cmd(&["--glob", "!*.txt"]),
        indoc!(
            "
            data (21.00 B)
            ├─ c.md (7.00 B)
            └─ nested (14.00 B)
               └─ other.md (14.00 B)"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        run_cmd(&["--glob", "*.TXT", "--glob-case-insensitive"]),
        indoc!(
            "
            data (10.00 B)
            ├─ b.txt (5.00 B)
            ├─ a.txt (5.00 B)
            └─ nested"
        )
    )
}

#[test]
fn iglob() {
    assert_eq!(
        run_cmd(&["--iglob", "*.TXT"]),
        indoc!(
            "
            data (10.00 B)
            ├─ b.txt (5.00 B)
            ├─ a.txt (5.00 B)
            └─ nested"
        )
    )
}
