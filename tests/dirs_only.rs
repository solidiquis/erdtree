use indoc::indoc;

mod utils;

#[test]
fn level() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--no-config",
            "--dirs-only",
            "tests/data"
        ]),
        indoc!(
            "
            data 
            ├─ dream_cycle 
            ├─ lipsum 
            └─ the_yellow_king"
        ),
        "Failed to print dirs-only."
    )
}
