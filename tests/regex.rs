use indoc::indoc;

mod utils;

#[test]
fn regex() {
    assert_eq!(
        utils::run_cmd(&["--pattern", r"\.txt$", "tests/data"]),
        indoc!(
            "100  B ┌─ nylarlathotep.txt
            161  B ├─ nemesis.txt
            83   B ├─ necronomicon.txt
            446  B │  ┌─ lipsum.txt
            446  B ├─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            1098 B data

            2 directories, 5 files"
        )
    );

    assert_eq!(
        utils::run_cmd(&["--pattern", r"^cassildas.", "--prune", "tests/data"]),
        indoc!(
            "143 B    ┌─ cassildas_song.md
            143 B ┌─ the_yellow_king
            143 B data

            1 directory, 1 file"
        )
    );
}

#[test]
fn regex_file_type() {
    assert_eq!(
        utils::run_cmd(&["--pattern", r"^dream.", "--file-type", "dir", "tests/data"]),
        indoc!(
            "308 B    ┌─ polaris.txt
            308 B ┌─ dream_cycle
            308 B data

            1 directory, 1 file"
        )
    );
}

#[should_panic]
#[test]
fn regex_empty_set_dir() {
    // Trying to look for a regular file when file type is specified to be directory should result
    // in an empty set which causes `main` to return an error.
    utils::run_cmd(&["--pattern", r"\.md$", "--file-type", "dir", "tests/data"]);
}

#[should_panic]
#[test]
fn regex_empty_set_file() {
    // Trying to look for a regular file when file type is specified to be directory should result
    // in an empty set which causes `main` to return an error.
    utils::run_cmd(&["--pattern", "weewoo", "tests/data"]);
}

#[should_panic]
#[test]
fn invalid_regex() {
    utils::run_cmd(&["--pattern", "*.txt", "tests/data"]);
}
