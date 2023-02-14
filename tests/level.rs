use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--level", "1", "tests/data"]),
        indoc!(
            "
            data (795.00 B)
            ├─ dream_cycle (308.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king (143.00 B)"
        ),
        "Failed to print at max level of 1."
    )
}
