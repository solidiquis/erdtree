use indoc::indoc;

mod utils;

#[test]
fn prune() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "*.txt",
            "--prune",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            └─ nylarlathotep.txt (100 B)"
        )
    )
}
