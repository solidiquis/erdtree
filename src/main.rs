#![cfg_attr(windows, feature(windows_by_handle))]
#![allow(
    clippy::cast_precision_loss,
    clippy::struct_excessive_bools,
    clippy::wildcard_imports,
    clippy::obfuscated_if_else
)]
use log::Log;
use std::process::ExitCode;

/// Defines the command-line interface and the context used throughout Erdtree.
mod user;

/// Concerned with disk usage calculation and presentation.
mod disk;

/// Error handling and reporting utilities to be used throughout the Erdtree.
mod error;

/// Erdtree's representation of a file.
mod file;

/// Concerned with logging throughout the application.
mod logging;

/// Virtual file-tree data structure and relevant operations.
mod tree;
use tree::FileTree;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> error::Result<()> {
    let ctx = user::Context::init()?;

    let logger = ctx
        .verbose
        .then_some(logging::LoggityLog::init())
        .transpose()?;

    let file_tree = if ctx.threads > 1 {
        FileTree::init(&ctx)?
    } else {
        FileTree::init(&ctx)?
    };

    let Some(indextree::NodeEdge::Start(id)) = file_tree.traverse().next() else {
        panic!("womp");
    };

    let root = file_tree[id].get();

    println!("{root:?}");

    if let Some(logger) = logger {
        logger.flush();
    }

    Ok(())
}
