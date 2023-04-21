#![cfg_attr(windows, feature(windows_by_handle))]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(
    clippy::struct_excessive_bools,
    clippy::cast_precision_loss,
    clippy::items_after_statements,
    clippy::similar_names,
    clippy::doc_markdown,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::fallible_impl_from
)]
use clap::CommandFactory;
use render::{
    context::Context,
    tree::{
        display::{Flat, Regular},
        Tree,
    },
};
use std::{io::stdout, process::ExitCode};

/// Filesystem operations.
mod fs;

/// Dev icons.
mod icons;

/// Tools and operations to display root-directory.
mod render;

/// Determine if standard streams are connected to a tty.
mod tty;

/// Common utilities across all modules.
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

    render::styles::init(ctx.no_color());

    if ctx.flat {
        let tree = Tree::<Flat>::try_init(ctx)?;
        println!("{tree}");
    } else {
        let tree = Tree::<Regular>::try_init(ctx)?;
        println!("{tree}");
    }

    Ok(())
}
