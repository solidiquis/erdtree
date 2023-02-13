use indoc::indoc;

mod utils;

#[test]
fn gitignore() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--ignore-git-ignore", "tests/data"]),
        indoc!(
            "
            data (795.00 B)
            ├─ dream_cycle (308.00 B)
            │  └─ polaris.txt (308.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king (143.00 B)
               └─ cassildas_song.md (143.00 B)"
        )
    )
}

#[test]
fn hidden() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--hidden", "tests/data"]),
        indoc!(
            "
            data (585.00 B)
            ├─ .dagon (86.00 B)
            ├─ .gitignore (12.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king (143.00 B)
               └─ cassildas_song.md (143.00 B)"
        )
    )
}
