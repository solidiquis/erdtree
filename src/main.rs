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
#![allow(clippy::cast_precision_loss, clippy::struct_excessive_bools, clippy::wildcard_imports)]

use clap::CommandFactory;
use context::{layout, Context};
use progress::{Indicator, IndicatorHandle, Message};
use render::{Engine, Flat, FlatInverted, Inverted, Regular};
use std::{
    error::Error,
    io::{stdout, Write},
    process::ExitCode,
};
use tree::Tree;

use erdtree::*;

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

    styles::init(ctx.no_color());

    let indicator = Indicator::maybe_init(&ctx);

    let (tree, ctx) = {
        match Tree::try_init(ctx, indicator.as_ref()) {
            Ok(res) => res,
            Err(err) => {
                IndicatorHandle::terminate(indicator);
                return Err(Box::new(err));
            },
        }
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

    if let Some(mut progress) = indicator {
        progress
            .mailbox()
            .send(Message::RenderReady)
            .map_err(|_e| tree::error::Error::Terminated)?;

        progress
            .join_handle
            .take()
            .map(|h| h.join().unwrap())
            .transpose()?;
    }

    #[cfg(debug_assertions)]
    {
        if std::env::var_os("ERDTREE_DEBUG").is_none() {
            let _ = writeln!(stdout(), "{output}");
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let _ = writeln!(stdout(), "{output}");
    }

    Ok(())
}
