#![cfg_attr(windows, feature(windows_by_handle))]
use clap::CommandFactory;
use log::Log;
use std::{
    io::{stdout, Write},
    process::ExitCode,
};

/// Defines the command-line interface and the context used throughout Erdtree.
mod user;
use user::Context;

/// Concerned with disk usage calculation and presentation.
mod disk;

/// Error handling and reporting utilities to be used throughout the Erdtree.
mod error;
use error::prelude::*;

/// Erdtree's representation of a file.
mod file;

/// Concerned with file icons.
mod icon;

/// Concerned with logging throughout the application.
mod logging;

/// Concerned with rendering the program output.
mod render;

const BIN_NAME: &str = "erd";

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let mut ctx = user::Context::init()?;

    if let Some(shell) = ctx.completions {
        clap_complete::generate(shell, &mut Context::command(), BIN_NAME, &mut stdout());
        return Ok(());
    }

    let logger = ctx
        .verbose
        .then_some(logging::LoggityLog::init())
        .transpose()?;

    let mut file_tree = if ctx.suppress_size {
        file::Tree::init_without_disk_usage(&ctx).map(|(tree, column_metadata)| {
            ctx.update_column_metadata(column_metadata);
            tree
        })?
    } else {
        file::Tree::init(&ctx).map(|(tree, column_metadata)| {
            ctx.update_column_metadata(column_metadata);
            tree
        })?
    };

    file_tree.filter_nodes(&ctx)?;

    let output = render::output(&file_tree, &ctx)?;

    let mut stdout = stdout().lock();
    writeln!(stdout, "{output}").into_report(ErrorCategory::Warning)?;

    if let Some(logger) = logger {
        logger.flush();
    }

    Ok(())
}
