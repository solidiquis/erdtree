use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&["--glob", "--pattern", "*.txt", "tests/data"]),
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
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&["--glob", "--pattern", "!*.txt", "tests/data"]),
        indoc!(
            "143 B    ┌─ cassildas_song.md
            143 B ┌─ the_yellow_king
            143 B data

            1 directory, 1 file"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        utils::run_cmd(&["--iglob", "--pattern", "*.TXT", "tests/data"]),
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
    )
}

#[test]
fn glob_with_filetype() {
    assert_eq!(
        utils::run_cmd(&[
            "--glob",
            "--pattern",
            "dream*",
            "--file-type",
            "dir",
            "tests/data"
        ]),
        indoc!(
            "308 B    ┌─ polaris.txt
            308 B ┌─ dream_cycle
            308 B data

            1 directory, 1 file"
        )
    )
}

#[test]
#[should_panic]
fn glob_empty_set_dir() {
    utils::run_cmd(&[
        "--glob",
        "--pattern",
        "*.txt",
        "--file-type",
        "dir",
        "tests/data",
    ]);
}

#[test]
#[should_panic]
fn glob_empty_set_file() {
    utils::run_cmd(&["--glob", "--pattern", "*weewoo*", "tests/data"]);
}
