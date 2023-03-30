use indoc::indoc;

mod utils;

#[test]
fn file_count() {
    assert_eq!(
        utils::run_cmd(&[
            "--count",
            "--sort",
            "name",
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
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king (143 B)
               └─ cassildas_song.md (143 B)

            3 directories, 6 files"
        ),
    )
}
