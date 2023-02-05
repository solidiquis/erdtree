use clap::Parser;
use cli::Clargs;
use fs::erdtree::{self, tree::Tree};
use ignore::WalkParallel;

mod cli;
mod fs;

fn main() -> Result<(), fs::error::Error> {
    erdtree::init_ls_colors();
    let clargs = Clargs::parse();
    let walker = WalkParallel::from(&clargs);
    let tree = Tree::new(walker, clargs.order(), clargs.max_depth())?;

    println!("{tree}");

    Ok(())
}
