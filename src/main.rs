mod file_tree;

use file_tree::FileTree;

fn main() {
    let file_tree = FileTree::new("./assets/").unwrap();
    file_tree.display();
}
