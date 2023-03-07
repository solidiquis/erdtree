use indoc::indoc;

mod utils;

#[test]
fn suppress_size() {
    assert_eq!(
        utils::run_cmd(&["--suppress-size", "tests/data"]),
        indoc!(
            "
            data 
            ├─ nylarlathotep.txt 
            ├─ lipsum 
            │  └─ lipsum.txt 
            ├─ dream_cycle 
            │  └─ polaris.txt 
            ├─ necronomicon.txt 
            ├─ the_yellow_king 
            │  └─ cassildas_song.md 
            └─ nemesis.txt"
        ),
        "Failed to suppress size."
    )
}
