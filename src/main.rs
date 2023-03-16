use clap::CommandFactory;
use render::{
    context::Context,
    tree::{self, Tree},
};
use std::{io::stdout, process::ExitCode};

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
    let ctx = Context::init()?;

    if let Some(shell) = ctx.completions {
        clap_complete::generate(shell, &mut Context::command(), "et", &mut stdout().lock());
        return Ok(());
    }

    tree::ui::init();
    let tree = Tree::init(ctx)?;

    println!("{tree}");

    Ok(())
}
