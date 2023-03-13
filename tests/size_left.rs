mod utils;

#[test]
fn size_left_bin() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--no-config", "--size-left", "tests/data"]),
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
        "Failed to show size on the left"
    )
}

#[test]
fn size_left_si() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--no-config", "-p", "si", "--size-left", "tests/data"]),
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
        "Failed to show size on the left"
    )
}
