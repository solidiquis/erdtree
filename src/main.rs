#![cfg_attr(windows, feature(windows_by_handle))]
use clap::CommandFactory;
use std::{
    io::stdout,
    process::ExitCode,
};

/// Concerned with disk usage calculation and presentation.
mod disk;

/// Error handling and reporting utilities to be used throughout the Erdtree.
mod error;
use error::prelude::*;

/// Erdtree's representation of a file.
mod file;

/// Concerned with file icons.
mod icon;

/// Progress indicator.
mod progress;

/// Defines the command-line interface and the context used throughout Erdtree.
mod user;
use user::Context;


/// For basic performance measurements when compiling for debug.
#[cfg(debug_assertions)]
#[macro_use]
mod perf;

/// Concerned with rendering the program output.
mod render;
use render::Renderer;

const BIN_NAME: &str = "erd";

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        perf::init_global();
        perf::begin_recording("crate::main");
    }

    #[cfg(debug_assertions)]
    crate::perf::begin_recording("crate::user::Context::init");

    let mut ctx = user::Context::init()?;

    #[cfg(debug_assertions)]
    crate::perf::finish_recording("crate::user::Context::init");

    if let Some(shell) = ctx.completions {
        clap_complete::generate(shell, &mut Context::command(), BIN_NAME, &mut stdout());
        return Ok(());
    }

    // TODO: Use accumulator
    let (mut file_tree, _accumulator, column_metadata) = if ctx.no_progress {
        if ctx.suppress_size {
            file::Tree::init_without_disk_usage(&ctx)
        } else {
            file::Tree::init(&ctx)
        }
    } else {
        progress::Indicator::init().show_progress(|| {
            if ctx.suppress_size {
                file::Tree::init_without_disk_usage(&ctx)
            } else {
                file::Tree::init(&ctx)
            }
        })
    }?;

    ctx.update_column_metadata(column_metadata);

    file_tree.filter_nodes(&ctx)?;

    Renderer::new(&ctx, &file_tree).render()?;

    #[cfg(debug_assertions)]
    {
        perf::finish_recording("crate::main");

        if ctx.debug {
            perf::output(std::io::stdout());
        }
    }

    Ok(())
}
