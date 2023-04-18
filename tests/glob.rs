use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "--pattern",
            "*.txt",
            "tests/data"
        ]),
        indoc!(
            "1.07 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B ├─ nylarlathotep.txt
                     - └─ the_yellow_king

           3 directories, 5 files"
        )
    );
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "--pattern",
            "!*.txt",
            "tests/data"
        ]),
        indoc!(
            "143   B data
                  - ├─ dream_cycle
                  - ├─ lipsum
            143   B └─ the_yellow_king
            143   B    └─ cassildas_song.md

        3 directories, 1 file"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--iglob",
            "--pattern",
            "*.TXT",
            "tests/data"
        ]),
        indoc!(
            "1.07 KiB data
                308   B ├─ dream_cycle
                308   B │  └─ polaris.txt
                446   B ├─ lipsum
                446   B │  └─ lipsum.txt
                 83   B ├─ necronomicon.txt
                161   B ├─ nemesis.txt
                100   B ├─ nylarlathotep.txt
                      - └─ the_yellow_king

            3 directories, 5 files"
        )
    )
}

#[test]
fn iglob() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--iglob",
            "--pattern",
            "*.TXT",
            "tests/data"
        ]),
        indoc!(
            "1.07 KiB data
                308   B ├─ dream_cycle
                308   B │  └─ polaris.txt
                446   B ├─ lipsum
                446   B │  └─ lipsum.txt
                 83   B ├─ necronomicon.txt
                161   B ├─ nemesis.txt
                100   B ├─ nylarlathotep.txt
                      - └─ the_yellow_king

            3 directories, 5 files"
        )
    )
}
