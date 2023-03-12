use indoc::indoc;

mod utils;

#[test]
fn sort_name() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--no-config", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king (143 B)
               └─ cassildas_song.md (143 B)"
        ),
        "Failed to sort alphabetically by file name"
    )
}

#[test]
fn sort_name_dir_first() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--dirs-first",
            "--no-config",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ the_yellow_king (143 B)
            │  └─ cassildas_song.md (143 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            └─ nylarlathotep.txt (100 B)"
        ),
        "Failed to sort by directory and alphabetically by file name"
    )
}

#[test]
fn sort_size() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size", "--no-config", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ necronomicon.txt (83 B)
            ├─ nylarlathotep.txt (100 B)
            ├─ the_yellow_king (143 B)
            │  └─ cassildas_song.md (143 B)
            ├─ nemesis.txt (161 B)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            └─ lipsum (446 B)
               └─ lipsum.txt (446 B)"
        ),
        "Failed to sort by descending size"
    )
}

#[test]
fn sort_size_dir_first() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "size",
            "--dirs-first",
            "--no-config",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ the_yellow_king (143 B)
            │  └─ cassildas_song.md (143 B)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nylarlathotep.txt (100 B)
            └─ nemesis.txt (161 B)"
        ),
        "Failed to sort by directory and descending size"
    )
}
