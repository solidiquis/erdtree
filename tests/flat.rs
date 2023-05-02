use indoc::indoc;

mod utils;

#[test]
fn flat() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "tests/data"]),
        indoc!(
            "1241 B   data
    308  B   dream_cycle
    308  B   dream_cycle/polaris.txt
    446  B   lipsum
    446  B   lipsum/lipsum.txt
    83   B   necronomicon.txt
    161  B   nemesis.txt
    100  B   nylarlathotep.txt
    143  B   the_yellow_king
    143  B   the_yellow_king/cassildas_song.md

    3 directories, 6 files"
        )
    )
}

#[test]
fn flat_human() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "--human", "tests/data"]),
        indoc!(
            "1.21 KiB   data
     308   B   dream_cycle
     308   B   dream_cycle/polaris.txt
     446   B   lipsum
     446   B   lipsum/lipsum.txt
      83   B   necronomicon.txt
     161   B   nemesis.txt
     100   B   nylarlathotep.txt
     143   B   the_yellow_king
     143   B   the_yellow_king/cassildas_song.md

    3 directories, 6 files"
        )
    )
}

#[test]
fn flat_with_level() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "--level", "1", "tests/data"]),
        indoc!(
            "1241 B   data
    308  B   dream_cycle
    446  B   lipsum
    83   B   necronomicon.txt
    161  B   nemesis.txt
    100  B   nylarlathotep.txt
    143  B   the_yellow_king

    3 directories, 6 files"
        )
    )
}

#[test]
#[should_panic]
fn flat_requires_file_name() {
    utils::run_cmd(&["--file-name"]);
}
