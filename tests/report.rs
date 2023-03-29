use indoc::indoc;

mod utils;

#[test]
fn report() {
    assert_eq!(
        utils::run_cmd(&["--report", "--sort", "name", "--no-config", "tests/data"]),
        indoc!(
            "
            d   1241 B   data
            d    308 B   dream_cycle
            -    308 B   dream_cycle/polaris.txt
            d    446 B   lipsum
            -    446 B   lipsum/lipsum.txt
            -     83 B   necronomicon.txt
            -    161 B   nemesis.txt
            -    100 B   nylarlathotep.txt
            d    143 B   the_yellow_king
            -    143 B   the_yellow_king/cassildas_song.md"
        )
    )
}

#[test]
fn report_human() {
    assert_eq!(
        utils::run_cmd(&[
            "--report",
            "--human",
            "--sort",
            "name",
            "--no-config",
            "tests/data"
        ]),
        indoc!(
            "
            d     1.21 KiB   data
            d        308 B   dream_cycle
            -        308 B   dream_cycle/polaris.txt
            d        446 B   lipsum
            -        446 B   lipsum/lipsum.txt
            -         83 B   necronomicon.txt
            -        161 B   nemesis.txt
            -        100 B   nylarlathotep.txt
            d        143 B   the_yellow_king
            -        143 B   the_yellow_king/cassildas_song.md"
        )
    )
}
