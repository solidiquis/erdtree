use indoc::indoc;

mod utils;

#[test]
fn sort_name() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "tests/data"]),
        indoc!(
            "1.21 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B ├─ nylarlathotep.txt
               143   B └─ the_yellow_king
               143   B    └─ cassildas_song.md

            3 directories, 6 files"
        ),
        "Failed to sort alphabetically by file name"
    )
}

#[test]
fn sort_name_dir_first() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--dirs-first", "tests/data"]),
        indoc!(
            "1.21 KiB data
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
               143   B ├─ the_yellow_king
               143   B │  └─ cassildas_song.md
                83   B ├─ necronomicon.txt
               161   B ├─ nemesis.txt
               100   B └─ nylarlathotep.txt

            3 directories, 6 files"
        ),
        "Failed to sort by directory and alphabetically by file name"
    )
}

#[test]
fn sort_size() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size", "tests/data"]),
        indoc!(
            "1.21 KiB data
                83   B ├─ necronomicon.txt
               100   B ├─ nylarlathotep.txt
               143   B ├─ the_yellow_king
               143   B │  └─ cassildas_song.md
               161   B ├─ nemesis.txt
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B └─ lipsum
               446   B    └─ lipsum.txt

            3 directories, 6 files"
        ),
        "Failed to sort by descending size"
    )
}

#[test]
fn sort_size_dir_first() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size", "--dirs-first", "tests/data"]),
        indoc!(
            "1.21 KiB data
               143   B ├─ the_yellow_king
               143   B │  └─ cassildas_song.md
               308   B ├─ dream_cycle
               308   B │  └─ polaris.txt
               446   B ├─ lipsum
               446   B │  └─ lipsum.txt
                83   B ├─ necronomicon.txt
               100   B ├─ nylarlathotep.txt
               161   B └─ nemesis.txt

            3 directories, 6 files"
        ),
        "Failed to sort by directory and descending size"
    )
}
