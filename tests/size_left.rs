use indoc::indoc;

mod utils;

#[test]
fn size_left() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--no-config", "--size_left", "tests/data"]),
        indoc!(
            "
            1.21 KiB data
             308   B ├─ dream_cycle
             308   B │  └─ polaris.txt
             446   B ├─ lipsum
             446   B │  └─ lipsum.txt
              83   B ├─ necronomicon.txt
             161   B ├─ nemesis.txt
             100   B ├─ nylarlathotep.txt
             143   B └─ the_yellow_king
             143   B    └─ cassildas_song.md"
        ),
        "Failed to show size on the left"
    )
}
