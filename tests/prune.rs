use indoc::indoc;

mod utils;

#[test]
fn prune() {
    assert_eq!(
        utils::run_cmd(&["--glob", "--pattern", "*.txt", "--prune", "tests/data"]),
        indoc!(
            "100  B ┌─ nylarlathotep.txt
            161  B ├─ nemesis.txt
            83   B ├─ necronomicon.txt
            446  B │  ┌─ lipsum.txt
            446  B ├─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            1098 B data

            2 directories, 5 files"
        )
    );
}
