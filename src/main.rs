use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches};
use cli::Clargs;
use fs::erdtree::{self, tree::Tree};

/// CLI rules and definitions.
mod cli;

/// Filesystem operations.
mod fs;

/// Dev icons.
mod icons;

/// Common utilities.
mod utils;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    erdtree::tree::ui::init();
    let matches = Clargs::command().args_override_self(true).get_matches();
    let clargs = Clargs::from_arg_matches(&matches)?;
    let tree = Tree::try_from(clargs)?;

    println!("{tree}");

    Ok(())
}
