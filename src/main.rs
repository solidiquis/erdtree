#![cfg_attr(windows, feature(windows_by_handle))]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::let_underscore_untyped,
    clippy::struct_excessive_bools,
    clippy::too_many_arguments
)]

use clap::CommandFactory;
use context::{layout, Context};
use progress::Message;
use render::{Engine, Flat, FlatInverted, Inverted, Regular};
use std::{error::Error, io::stdout, process::ExitCode};
use tree::Tree;

/// Operations to wrangle ANSI escaped strings.
mod ansi;

/// CLI rules and definitions as well as context to be injected throughout the entire program.
mod context;

/// Operations relevant to the computation and presentation of disk usage.
mod disk_usage;

/// Filesystem operations.
mod fs;

/// All things related to icons on how to map certain files to the appropriate icons.
mod icons;

/// Concerned with displaying a progress indicator when stdout is a tty.
mod progress;

/// Concerned with taking an initialized [`Tree`] and its [`Node`]s and rendering the output.
///
/// [`Tree`]: tree::Tree
/// [`Node`]: tree::node::Node
mod render;

/// Global used throughout the program to paint the output.
mod styles;

/// Houses the primary data structures that are used to virtualize the filesystem, containing also
/// information on how the tree output should be ultimately rendered.
mod tree;

/// Utilities relating to interacting with tty properties.
mod tty;

/// Common utilities across all modules.
mod utils;

fn main() -> ExitCode {
    let result = run();

    tty::restore_tty();

    if let Err(e) = result {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn Error>> {
    let ctx = Context::try_init()?;

    if let Some(shell) = ctx.completions {
        clap_complete::generate(shell, &mut Context::command(), "erd", &mut stdout());
        return Ok(());
    }

    context::color::no_color_env();

    styles::init(ctx.no_color());

    let indicator = (ctx.stdout_is_tty && !ctx.no_progress).then(progress::Indicator::measure);

    let (tree, ctx) = match Tree::try_init(ctx, indicator.as_ref()) {
        Ok(res) => res,
        Err(err) => {
            if let Some(thread) = indicator.map(|i| i.join_handle) {
                thread.join().unwrap()?;
            }
            return Err(Box::new(err));
        },
    };

    macro_rules! compute_output {
        ($t:ty) => {{
            let render = Engine::<$t>::new(tree, ctx);
            format!("{render}")
        }};
    }

    let output = match ctx.layout {
        layout::Type::Flat => compute_output!(Flat),
        layout::Type::Iflat => compute_output!(FlatInverted),
        layout::Type::Inverted => compute_output!(Inverted),
        layout::Type::Regular => compute_output!(Regular),
    };

    if let Some(progress) = indicator {
        progress.mailbox().send(Message::RenderReady)?;
        progress.join_handle.join().unwrap()?;
    }

    #[cfg(debug_assertions)]
    {
        if std::env::var_os("ERDTREE_DEBUG").is_none() {
            println!("{output}");
        }
    }

    #[cfg(not(debug_assertions))]
    {
        println!("{output}");
    }

    Ok(())
}
