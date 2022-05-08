use std::env;

mod cli;
mod file_tree;
mod utils;

use cli::CommandLineArgs;
use file_tree::FileTree;

fn main() {
    let args = env::args();

    if args.len() <= 1 {
        FileTree::default().display()
    } else {
        let clargs = args.collect::<Vec<String>>();
        let (_, args) = clargs.split_first().unwrap();
        
        let CommandLineArgs {
            directory,
            depth,
            prefixes,
            sort_type
        } = cli::parse_args(args);

        let dir = match directory {
            Some(d) => d,
            None => ".".to_string()
        };

        let pre = match prefixes {
            Some(d) => d,
            None => "".to_string()
        };

        let maybe_pre = if pre == "".to_string() {
            None
        } else {
            Some(pre.as_str())
        };

        FileTree::new(&dir, maybe_pre, depth, sort_type).unwrap().display();
    }
}
