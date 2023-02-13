use std::process::Command;
use std::process::Stdio;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub fn run_cmd(args: &[&str]) -> String {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("--threads").arg("1");

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
