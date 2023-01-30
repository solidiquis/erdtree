use ignore::WalkBuilder;

#[test]
fn dir_sizes() {
    use std::{
        fs::{self, File},
        io::Write,
    };
    use super::tree::Tree;
    use tempdir::TempDir;

    let tmp_dir_name = "erdtree_test";

    let tmp_dir = TempDir::new(tmp_dir_name)
        .expect("Failed to create tmp directory");

    let text_a = "Where they roll in their horror unheeded";
    let text_b = "Without knowledge, or lustre, or name";

    let file_path = tmp_dir.path().join("cthulhu.txt");
    let mut tmp_file = File::create(file_path)
        .expect("Failed to create file.");
    writeln!(tmp_file, "{}", text_a).unwrap();

   let nested_dir_path = tmp_dir.path().join("nested");
   let _nested_dir = fs::create_dir_all(&nested_dir_path)
       .expect("Failed to created nested directory");
   let nested_file_path = nested_dir_path.join("nyarlathotep.txt");
   let mut nested_tmp_file = File::create(nested_file_path)
       .expect("Failed to create nested file");
    writeln!(nested_tmp_file, "{}", text_b).unwrap();

    let walker = WalkBuilder::new(tmp_dir.path())
        .follow_links(false)
        .git_ignore(false)
        .hidden(false)
        .max_depth(None)
        .threads(1)
        .build_parallel();

    let tree = Tree::new(walker).unwrap();

    let texts = vec![
        text_a,
        text_b,
    ];

    assert!(tree.root.file_size.is_some());

    assert_eq!(
        tree.root.file_size.unwrap(),
        texts.iter().map(|txt| txt.len() as u64).sum::<u64>() + (texts.len() as u64)
    )
}
