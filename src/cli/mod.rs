use crate::error::{self, ErrorReport, Result, WithContext};
use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "erdtree")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "4.0.0")]
#[command(
    about = "erdtree (erd) is a cross-platform, multi-threaded, and general purpose filesystem and disk usage utility.",
    long_about = None,
)]
pub struct Args {
    /// Directory to traverse; defaults to current working directory
    dir: Option<PathBuf>,
}

impl Args {
    pub fn init() -> Result<Self> {
        let mut clargs = Self::parse();
        clargs.set_dir()?;
        Ok(clargs)
    }

    fn set_dir(&mut self) -> Result<()> {
        let current_dir = env::current_dir().into_report(error::Category::System)?;

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "This is the underlying error",
        ))
        .into_report(error::Category::User)
        .context("Oh my god...")
        .context("Look at her butt")
        .context("omg..")?;

        //.into_report_ctx(error::Category::Internal, "hmmmm")?;

        Ok(())
    }
}
