use indoc::indoc;

mod utils;

#[test]
fn line_count() {
    assert_eq!(
        utils::run_cmd(&["--disk-usage", "line", "tests/data"]),
        indoc!(
            "6    ┌─ cassildas_song.md
 6 ┌─ the_yellow_king
 1 ├─ nylarlathotep.txt
 4 ├─ nemesis.txt
 2 ├─ necronomicon.txt
 1 │  ┌─ lipsum.txt
 1 ├─ lipsum
10 │  ┌─ polaris.txt
10 ├─ dream_cycle
24 data

3 directories, 6 files"
        )
    )
}
