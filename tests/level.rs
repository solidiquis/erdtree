use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--level", "1", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308 B)
            ├─ lipsum (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king (143 B)"
        ),
        "Failed to print at max level of 1."
    )
}
