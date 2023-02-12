use indoc::indoc;

mod utils;

#[test]
fn sort_name() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name"]),
        indoc!(
            "
            data (487.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king (143.00 B)
               └─ cassildas_song.md (143.00 B)",
        ),
        "Failed to sort alphabetically by file name"
    )
}

#[test]
fn sort_size() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size"]),
        indoc!(
            "
            data (487.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ the_yellow_king (143.00 B)
            │  └─ cassildas_song.md (143.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ necronomicon.txt (83.00 B)"
        ),
        "Failed to sort by descending size"
    )
}
