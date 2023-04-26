use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&["--level", "1", "tests/data"]),
        indoc!(
            "143  B ┌─ the_yellow_king
            100  B ├─ nylarlathotep.txt
            161  B ├─ nemesis.txt
            83   B ├─ necronomicon.txt
            446  B ├─ lipsum
            308  B ├─ dream_cycle
            1241 B data

            3 directories, 6 files"
        ),
        "Failed to print at max level of 1."
    )
}
