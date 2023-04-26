use indoc::indoc;

mod utils;

#[test]
fn suppress_size() {
    assert_eq!(
        utils::run_cmd(&["--suppress-size", "tests/data"]),
        indoc!(
            "┌─ cassildas_song.md
             ┌─ the_yellow_king
             ├─ nylarlathotep.txt
             ├─ nemesis.txt
             ├─ necronomicon.txt
             │  ┌─ lipsum.txt
             ├─ lipsum
             │  ┌─ polaris.txt
             ├─ dream_cycle
             data

            3 directories, 6 files"
        ),
        "Failed to suppress size."
    )
}
