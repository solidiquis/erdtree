#![cfg_attr(windows, feature(windows_by_handle))]
use log::Log;
use std::{
    io::{stdout, Write},
    process::ExitCode,
};

/// Defines the command-line interface and the context used throughout Erdtree.
mod user;

/// Concerned with disk usage calculation and presentation.
mod disk;

/// Error handling and reporting utilities to be used throughout the Erdtree.
mod error;
use error::prelude::*;

/// Erdtree's representation of a file.
mod file;

/// Concerned with logging throughout the application.
mod logging;

/// Concerned with rendering the program output.
mod render;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let mut ctx = user::Context::init()?;

    let logger = ctx
        .verbose
        .then_some(logging::LoggityLog::init())
        .transpose()?;

    let file_tree = file::Tree::init(&ctx).and_then(|(tree, column_metadata)| {
        ctx.update_column_metadata(column_metadata);
        Ok(tree)
    })?;

    let output = render::tree(&file_tree, &ctx)?;

    let mut stdout = stdout().lock();
    writeln!(stdout, "{output}").into_report(ErrorCategory::Warning)?;

    if let Some(logger) = logger {
        logger.flush();
    }

    Ok(())
}
