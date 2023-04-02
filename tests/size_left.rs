mod utils;

#[test]
fn size_left_bin() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--size-left", "tests/data"]),
        "1.21 KiB data
   308   B ├─ dream_cycle
   308   B │  └─ polaris.txt
   446   B ├─ lipsum
   446   B │  └─ lipsum.txt
    83   B ├─ necronomicon.txt
   161   B ├─ nemesis.txt
   100   B ├─ nylarlathotep.txt
   143   B └─ the_yellow_king
   143   B    └─ cassildas_song.md",
    )
}

#[test]
fn size_left_si() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "-p",
            "si",
            "--size-left",
            "tests/data"
        ]),
        "1.24 KB data
   308  B ├─ dream_cycle
   308  B │  └─ polaris.txt
   446  B ├─ lipsum
   446  B │  └─ lipsum.txt
    83  B ├─ necronomicon.txt
   161  B ├─ nemesis.txt
   100  B ├─ nylarlathotep.txt
   143  B └─ the_yellow_king
   143  B    └─ cassildas_song.md",
    )
}

#[test]
fn size_left_altered_precision() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "-n",
            "3",
            "--size-left",
            "tests/data"
        ]),
        "1.212 KiB data
    308   B ├─ dream_cycle
    308   B │  └─ polaris.txt
    446   B ├─ lipsum
    446   B │  └─ lipsum.txt
     83   B ├─ necronomicon.txt
    161   B ├─ nemesis.txt
    100   B ├─ nylarlathotep.txt
    143   B └─ the_yellow_king
    143   B    └─ cassildas_song.md",
    )
}
