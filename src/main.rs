use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches};
use context::Context;
use render::tree::{self, Tree};

/// CLI rules and definitions and context wherein [Tree] will operate.
mod context;

/// Filesystem operations.
mod fs;

/// Dev icons.
mod icons;

/// Tools and operations to display root-directory interact with ANSI configs.
mod render;

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
    tree::ui::init();
    let matches = Context::command().args_override_self(true).get_matches();
    let clargs = Context::from_arg_matches(&matches)?;

    let tree = Tree::try_from(clargs)?;

    println!("{tree}");

    Ok(())
}
