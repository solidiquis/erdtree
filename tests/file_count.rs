use indoc::indoc;

mod utils;

#[test]
fn file_count() {
    assert_eq!(
        utils::run_cmd(&["--count", "--sort", "name", "--no-config", "tests/data"]),
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

#[test]
fn file_count_report() {
    assert_eq!(
        utils::run_cmd(&[
            "--count",
            "--report",
            "--sort",
            "name",
            "--no-config",
            "tests/data"
        ]),
        indoc!(
            "
            d   1241 B   data
            d    308 B   dream_cycle
            -    308 B   dream_cycle/polaris.txt
            d    446 B   lipsum
            -    446 B   lipsum/lipsum.txt
            -     83 B   necronomicon.txt
            -    161 B   nemesis.txt
            -    100 B   nylarlathotep.txt
            d    143 B   the_yellow_king
            -    143 B   the_yellow_king/cassildas_song.md

            3 directories, 6 files"
        ),
    )
}
