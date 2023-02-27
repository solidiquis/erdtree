use std::process::ExitCode;

use clap::Parser;
use cli::Clargs;
use fs::erdtree::{self, tree::Tree};

/// CLI rules and definitions.
mod cli;

/// Filesystem operations.
mod fs;

/// Dev icons. Reference: https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua
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

fn run() -> Result<(), fs::error::Error> {
    erdtree::tree::ui::init();
    let clargs = Clargs::parse();
    let tree = Tree::try_from(clargs)?;

    println!("{tree}");

    Ok(())
}
