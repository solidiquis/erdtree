use indoc::indoc;

mod utils;

#[test]
fn report() {
    assert_eq!(
        utils::run_cmd(&["--report", "--sort", "name", "tests/data"]),
        indoc!(
            "d   1241 B   data
             d    308 B   dream_cycle
             -    308 B   dream_cycle/polaris.txt
             d    446 B   lipsum
             -    446 B   lipsum/lipsum.txt
             -     83 B   necronomicon.txt
             -    161 B   nemesis.txt
             -    100 B   nylarlathotep.txt
             d    143 B   the_yellow_king
             -    143 B   the_yellow_king/cassildas_song.md
 
             3 directories, 6 files"
        )
    )
}

#[test]
fn report_human() {
    assert_eq!(
        utils::run_cmd(&["--report", "--human", "--sort", "name", "tests/data"]),
        indoc!(
            "d     1.21 KiB   data
            d        308 B   dream_cycle
            -        308 B   dream_cycle/polaris.txt
            d        446 B   lipsum
            -        446 B   lipsum/lipsum.txt
            -         83 B   necronomicon.txt
            -        161 B   nemesis.txt
            -        100 B   nylarlathotep.txt
            d        143 B   the_yellow_king
            -        143 B   the_yellow_king/cassildas_song.md

            3 directories, 6 files"
        )
    )
}

#[test]
fn report_with_level() {
    assert_eq!(
        utils::run_cmd(&["--report", "--level", "1", "--sort", "name", "tests/data"]),
        indoc!(
            "d   1241 B   data
            d    308 B   dream_cycle
            d    446 B   lipsum
            -     83 B   necronomicon.txt
            -    161 B   nemesis.txt
            -    100 B   nylarlathotep.txt
            d    143 B   the_yellow_king

            3 directories, 6 files"
        )
    )
}

#[test]
#[should_panic]
fn report_requires_human() {
    utils::run_cmd(&["--human"]);
}

#[test]
#[should_panic]
fn report_requires_file_name() {
    utils::run_cmd(&["--file-name"]);
}
