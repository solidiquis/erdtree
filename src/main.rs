#![cfg_attr(windows, feature(windows_by_handle))]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::struct_excessive_bools,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

use clap::CommandFactory;
use context::{Context, layout};
use render::{Engine, Flat, Inverted, Regular};
use tree::Tree;
use std::{error::Error, io::stdout};

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

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::init()?;

    if let Some(shell) = ctx.completions {
        clap_complete::generate(shell, &mut Context::command(), "erd", &mut stdout().lock());
        return Ok(());
    }

    styles::init(ctx.no_color());

    let (tree, ctx) = Tree::try_init_and_update_context(ctx)?;

    match ctx.layout {
        layout::Type::Flat => {
            let render = Engine::<Flat>::new(tree, ctx);
            println!("{render}");
        },
        layout::Type::Inverted => {
            let render = Engine::<Inverted>::new(tree, ctx);
            println!("{render}");
        },
        layout::Type::Regular => {
            let render = Engine::<Regular>::new(tree, ctx);
            println!("{render}");
        },
    }

    Ok(())
}
