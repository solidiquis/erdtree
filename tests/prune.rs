use indoc::indoc;

mod utils;

#[test]
fn prune() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "--pattern",
            "*.txt",
            "--prune",
            "tests/data"
        ]),
        indoc!(
            "1.07 KiB data
                308   B ├─ dream_cycle
                308   B │  └─ polaris.txt
                446   B ├─ lipsum
                446   B │  └─ lipsum.txt
                 83   B ├─ necronomicon.txt
                161   B ├─ nemesis.txt
                100   B └─ nylarlathotep.txt

            2 directories, 5 files"
        )
    );
}
