use render::{
    context::Context,
    tree::{self, Tree},
};
use std::process::ExitCode;

/// Filesystem operations.
mod fs;

/// Dev icons.
mod icons;

/// Tools and operations to display root-directory.
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
    let ctx = Context::init()?;
    let tree = Tree::init(ctx)?;

    println!("{tree}");

    Ok(())
}
