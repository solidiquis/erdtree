use std::env;

mod cli;
mod file_tree;
mod utils;

use file_tree::FileTree;

fn main() {
    //let args = env::args();

    FileTree::default().display()
}
