use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--no-config", "--dirs-only", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308 B)
            ├─ lipsum (446 B)
            └─ the_yellow_king (143 B)"
        ),
        "Failed to print dirs-only."
    )
}
