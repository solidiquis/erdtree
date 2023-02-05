use clap::Parser;
use cli::Clargs;
use fs::erdtree::tree::Tree;
use ignore::WalkParallel;
use lscolors::LsColors;

mod cli;
mod fs;

fn main() -> Result<(), fs::error::Error> {
    let clargs = Clargs::parse();
    let walker = WalkParallel::from(&clargs);
    let lscolors = LsColors::from_env().unwrap_or_default();
    let tree = Tree::new(walker, clargs.order(), clargs.max_depth(), lscolors)?;

    println!("{tree}");

    Ok(())
}
