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
    if let Err(e) = run() {
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

    let (tree, ctx) =
        Tree::try_init_and_update_context(ctx, indicator.as_ref()).map_err(|err| {
            if let Some(ref progress) = indicator {
                progress.mailbox().send(Message::RenderReady).unwrap();
            }
            err
        })?;

    let output = match ctx.layout {
        layout::Type::Flat => {
            let render = Engine::<Flat>::new(tree, ctx);
            format!("{render}")
        }
        layout::Type::Iflat => {
            let render = Engine::<FlatInverted>::new(tree, ctx);
            format!("{render}")
        }
        layout::Type::Inverted => {
            let render = Engine::<Inverted>::new(tree, ctx);
            format!("{render}")
        }
        layout::Type::Regular => {
            let render = Engine::<Regular>::new(tree, ctx);
            format!("{render}")
        }
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
