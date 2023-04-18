use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--level", "1", "tests/data"]),
        indoc!(
            "1.21 KiB data
                308   B ├─ dream_cycle
                446   B ├─ lipsum
                 83   B ├─ necronomicon.txt
                161   B ├─ nemesis.txt
                100   B ├─ nylarlathotep.txt
                143   B └─ the_yellow_king

            3 directories, 6 files"
        ),
        "Failed to print at max level of 1."
    )
}
