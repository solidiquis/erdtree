use clap::Parser;
use cli::Clargs;
use fs::erdtree::{self, tree::Tree};
use ignore::WalkParallel;

/// CLI rules and definitions.
mod cli;

/// Filesystem operations.
mod fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    erdtree::init_ls_colors();
    let clargs = Clargs::parse();
    let walker = WalkParallel::try_from(&clargs)?;
    let tree = Tree::new(walker, clargs.order(), clargs.max_depth())?;

    println!("{tree}");

    Ok(())
}
