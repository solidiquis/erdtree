use indoc::indoc;

mod utils;

#[test]
fn suppress_size() {
    assert_eq!(
        utils::run_cmd(&[
            "--suppress-size",
            "--sort",
            "name",
            "tests/data"
        ]),
        indoc!(
            "
            data 
            ├─ dream_cycle 
            │  └─ polaris.txt 
            ├─ lipsum 
            │  └─ lipsum.txt 
            ├─ necronomicon.txt 
            ├─ nemesis.txt 
            ├─ nylarlathotep.txt 
            └─ the_yellow_king 
               └─ cassildas_song.md"
        ),
        "Failed to suppress size."
    )
}
