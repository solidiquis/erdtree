use super::super::order::Order;
use crate::fs::erdtree::init_ls_colors;
use std::io;
use tempdir::TempDir;

#[test]
fn test() -> io::Result<()> {
    init_ls_colors();
    let tmp_dir = utils::create_test_dir()?;

    test_size(&tmp_dir);
    test_alphabetical_ordering(&tmp_dir);
    test_size_ordering(&tmp_dir);

    if cfg!(unix) {
        test_symlink(&tmp_dir)?;
    }

    Ok(())
}

fn test_size(tmp_dir: &TempDir) {
    let tree = utils::init_tree(tmp_dir, Order::None);

    assert!(
        tree.root().children().is_some(),
        "Expected root directory to have children"
    );

    assert_eq!(
        tree.root().file_size,
        Some(254),
        "Root directory size doesn't add up"
    );
}

fn test_alphabetical_ordering(tmp_dir: &TempDir) {
    let tree = utils::init_tree(tmp_dir, Order::Name);

    let file_names = tree
        .root()
        .children()
        .map(|children| {
            children
                .into_iter()
                .map(|child| child.file_name())
                .collect::<Vec<&str>>()
        })
        .expect("Expected root directory to have children");

    for pair in file_names.windows(2) {
        assert!(pair[0] <= pair[1], "Expected alphabetical ordering")
    }
}

#[cfg(unix)]
fn test_symlink(tmp_dir: &TempDir) -> io::Result<()> {
    use std::os::unix::fs::symlink;

    let target = tmp_dir.path().join("nested_a");
    let link = tmp_dir.path().join("sym_a");

    symlink(target, link)?;
    let tree = utils::init_tree(tmp_dir, Order::None);

    let symlink_node = tree
        .root()
        .children()
        .map(|mut nodes| nodes.find(|node| node.is_symlink()).unwrap())
        .unwrap();

    assert!(
        symlink_node.children().is_none(),
        "Symlink should not have children"
    );

    assert!(
        symlink_node.file_size.is_none(),
        "Symlink should not have size"
    );

    Ok(())
}

fn test_size_ordering(tmp_dir: &TempDir) {
    let tree = utils::init_tree(tmp_dir, Order::Size);

    let file_names = tree
        .root()
        .children()
        .map(|children| {
            children
                .into_iter()
                .map(|child| child.file_size.unwrap_or(0))
                .collect::<Vec<u64>>()
        })
        .expect("Expected root directory to have children");

    for pair in file_names.windows(2) {
        assert!(pair[0] >= pair[1], "Expected descending ordering by size")
    }
}

pub(super) mod utils {
    use crate::fs::erdtree::{order::Order, tree::Tree};
    use ignore::WalkBuilder;
    use std::{
        fs::{self, File},
        io::{self, Write},
    };
    use tempdir::TempDir;

    pub fn create_test_dir() -> Result<TempDir, io::Error> {
        let tmp_dir_name = "erdtree_test";

        let tmp_dir = TempDir::new(tmp_dir_name)?;

        let the_call_of_cthulhu_path = tmp_dir.path().join("call_of_cthulhu.txt");
        File::create(the_call_of_cthulhu_path)
            .and_then(|mut f| writeln!(f, "That is not dead which can eternal lie; and with strange aeons even death may die."))?;

        let nested_dir_a = tmp_dir.path().join("nested_a");
        fs::create_dir(&nested_dir_a)?;

        let nemesis_path = nested_dir_a.join("nemesis.txt");
        File::create(nemesis_path).and_then(|mut f| {
            writeln!(
                f,
                "where they roll in their horror unheeded, without knowledge or lustre or name."
            )
        })?;

        let nested_dir_b = tmp_dir.path().join("nested_b");
        fs::create_dir(&nested_dir_b)?;

        let nyarlathotep_path = nested_dir_b.join("nyarlathotep.txt");
        File::create(nyarlathotep_path)
            .and_then(|mut f| writeln!(f, "Nyarlathotep ... the crawling chaos ... I am the last ... I will tell the audient void. ..."))?;

        Ok(tmp_dir)
    }

    pub fn init_tree(tmp_dir: &TempDir, order: Order) -> Tree {
        let walker = WalkBuilder::new(tmp_dir.path()).threads(1).build_parallel();
        Tree::new(walker, order, None).unwrap()
    }
}
