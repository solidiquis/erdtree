use indoc::indoc;

mod utils;

#[test]
fn regex() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--pattern", r"\.txt$", "tests/data"]),
        indoc!(
            "1.07 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B ├─ nylarlathotep.txt
                       └─ the_yellow_king

            3 directories, 5 files"
        )
    );

    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--pattern",
            r"^cassildas.",
            "--prune",
            "tests/data"
        ]),
        indoc!(
            "143   B data
               143   B └─ the_yellow_king
               143   B    └─ cassildas_song.md

            1 directory, 1 file"
        )
    );
}

#[should_panic]
#[test]
fn invalid_regex() {
    utils::run_cmd(&["--pattern", "*.txt", "tests/data"]);
}
