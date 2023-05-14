use indoc::indoc;

mod utils;

#[test]
fn flat() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "tests/data"]),
        indoc!(
            "143  B   the_yellow_king/cassildas_song.md
            143  B   the_yellow_king
            100  B   nylarlathotep.txt
            161  B   nemesis.txt
            83   B   necronomicon.txt
            446  B   lipsum/lipsum.txt
            446  B   lipsum
            308  B   dream_cycle/polaris.txt
            308  B   dream_cycle
            1241 B   data

            3 directories, 6 files"
        )
    )
}

#[test]
fn flat_human() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "--human", "tests/data"]),
        indoc!(
            "143   B   the_yellow_king/cassildas_song.md
             143   B   the_yellow_king
             100   B   nylarlathotep.txt
             161   B   nemesis.txt
              83   B   necronomicon.txt
             446   B   lipsum/lipsum.txt
             446   B   lipsum
             308   B   dream_cycle/polaris.txt
             308   B   dream_cycle
            1.21 KiB   data

            3 directories, 6 files"
        )
    )
}

#[test]
fn flat_with_level() {
    assert_eq!(
        utils::run_cmd(&["--layout", "flat", "--level", "1", "tests/data"]),
        indoc!(
            "143  B   the_yellow_king
            100  B   nylarlathotep.txt
            161  B   nemesis.txt
            83   B   necronomicon.txt
            446  B   lipsum
            308  B   dream_cycle
            1241 B   data

            3 directories, 6 files"
        )
    )
}
