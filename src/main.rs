#![cfg_attr(windows, feature(windows_by_handle))]

use std::process::ExitCode;

mod cli;
mod error;
mod file;
mod tree;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> error::Result<()> {
    let clargs = cli::Args::init()?;
    Ok(())
}
