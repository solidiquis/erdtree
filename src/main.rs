use clap::Parser;
use cli::Clargs;
use fs::erdtree::tree::Tree;
use ignore::WalkParallel;

mod cli;
mod fs;

fn main() -> Result<(), fs::error::Error> {
    let clargs = Clargs::parse();
    let walker = WalkParallel::from(&clargs);
    let tree = Tree::new(walker)?;

    println!("{tree}");

    Ok(())
}
