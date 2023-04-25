use indoc::indoc;

mod utils;

#[test]
fn sort_name() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "tests/data"]),
        indoc!(
            "143  B    ┌─ cassildas_song.md
            143  B ┌─ the_yellow_king
            100  B ├─ nylarlathotep.txt
            161  B ├─ nemesis.txt
            83   B ├─ necronomicon.txt
            446  B │  ┌─ lipsum.txt
            446  B ├─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            1241 B data

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
            "100  B ┌─ nylarlathotep.txt
            161  B ├─ nemesis.txt
            83   B ├─ necronomicon.txt
            143  B │  ┌─ cassildas_song.md
            143  B ├─ the_yellow_king
            446  B │  ┌─ lipsum.txt
            446  B ├─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            1241 B data

            3 directories, 6 files"
        ),
        "Failed to sort by directory and alphabetically by file name"
    )
}

#[test]
fn sort_size() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size-rev", "tests/data"]),
        indoc!(
            "446  B    ┌─ lipsum.txt
            446  B ┌─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            161  B ├─ nemesis.txt
            143  B │  ┌─ cassildas_song.md
            143  B ├─ the_yellow_king
            100  B ├─ nylarlathotep.txt
            83   B ├─ necronomicon.txt
            1241 B data

            3 directories, 6 files"
        ),
        "Failed to sort by descending size"
    )
}

#[test]
fn sort_size_dir_first() {
    assert_eq!(
        utils::run_cmd(&["--sort", "size-rev", "--dirs-first", "tests/data"]),
        indoc!(
            "161  B ┌─ nemesis.txt
            100  B ├─ nylarlathotep.txt
            83   B ├─ necronomicon.txt
            446  B │  ┌─ lipsum.txt
            446  B ├─ lipsum
            308  B │  ┌─ polaris.txt
            308  B ├─ dream_cycle
            143  B │  ┌─ cassildas_song.md
            143  B ├─ the_yellow_king
            1241 B data

            3 directories, 6 files"
        ),
        "Failed to sort by directory and descending size"
    )
}
