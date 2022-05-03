mod file_tree;
mod utils;

use file_tree::FileTree;

fn main() {
    let file_tree = FileTree::new(".", Some(".")).unwrap();
    file_tree.display();
}
