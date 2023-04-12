use indoc::indoc;

mod utils;

#[test]
fn dirs_only() {
    assert_eq!(
        utils::run_cmd(&["--dirs-only", "--sort", "name", "tests/data"]),
        indoc!(
             "1.21 KiB data
              308   B ├─ dream_cycle
              446   B ├─ lipsum
              143   B └─ the_yellow_king

           3 directories"
        )
    )
}
