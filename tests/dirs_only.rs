use indoc::indoc;

mod utils;

#[test]
fn dirs_only() {
    assert_eq!(
        utils::run_cmd(&["--dirs-only", "--sort", "name", "--no-config", "tests/data"]),
        indoc!(
            "
            data (1.21 KiB)
            ├─ dream_cycle (308 B)
            ├─ lipsum (446 B)
            └─ the_yellow_king (143 B)"
        )
    )
}
