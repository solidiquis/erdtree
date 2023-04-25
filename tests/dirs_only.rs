use indoc::indoc;

mod utils;

#[test]
fn dirs_only() {
    assert_eq!(
        utils::run_cmd(&["--dirs-only", "--sort", "name", "tests/data"]),
        indoc!(
            "143  B ┌─ the_yellow_king
    446  B ├─ lipsum
    308  B ├─ dream_cycle
    1241 B data

    3 directories"
        )
    )
}
