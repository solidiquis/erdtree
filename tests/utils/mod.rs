use std::process::Command;
use std::process::Stdio;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub fn run_cmd(args: &[&str]) -> String {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "run",
        "--",
        "--threads",
        "1",
        "--disk-usage",
        "logical",
        "--sort",
        "name",
        "--no-config",
    ]);

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    if !output.status.success() {
        let err_msg = String::from_utf8(output.stderr).unwrap();
        panic!("{err_msg}");
    }

    String::from_utf8(strip_ansi_escapes(output.stdout).unwrap())
        .unwrap()
        .trim()
        .to_string()
}
