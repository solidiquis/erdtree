use indoc::indoc;

mod utils;

#[test]
fn sort_name() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308.00 B)
            │  └─ polaris.txt (308.00 B)
            ├─ lipsum (446.00 B)
            │  └─ lipsum.txt (446.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king (143.00 B)
               └─ cassildas_song.md (143.00 B)"
        ),
        "Failed to sort alphabetically by file name"
    )
}

#[test]
fn sort_name_dir_first() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--dirs-first", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308.00 B)
            │  └─ polaris.txt (308.00 B)
            ├─ lipsum (446.00 B)
            │  └─ lipsum.txt (446.00 B)
            ├─ the_yellow_king (143.00 B)
            │  └─ cassildas_song.md (143.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            └─ nylarlathotep.txt (100.00 B)"
        ),
        "Failed to sort by directory and alphabetically by file name"
    )
}

#[test]
fn sort_size() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ necronomicon.txt (83.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            ├─ the_yellow_king (143.00 B)
            │  └─ cassildas_song.md (143.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ dream_cycle (308.00 B)
            │  └─ polaris.txt (308.00 B)
            └─ lipsum (446.00 B)
               └─ lipsum.txt (446.00 B)"
        ),
        "Failed to sort by descending size"
    )
}

#[test]
fn sort_size_dir_first() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size", "--dirs-first", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ the_yellow_king (143.00 B)
            │  └─ cassildas_song.md (143.00 B)
            ├─ dream_cycle (308.00 B)
            │  └─ polaris.txt (308.00 B)
            ├─ lipsum (446.00 B)
            │  └─ lipsum.txt (446.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ nemesis.txt (161.00 B)"
        ),
        "Failed to sort by directory and descending size"
    )
}
