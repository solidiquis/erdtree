use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "*.txt", "tests/data"]),
        indoc!(
            "
            data (344.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "!*.txt", "tests/data"]),
        indoc!(
            "
            data (143.00 B)
            └─ the_yellow_king (143.00 B)
               └─ cassildas_song.md (143.00 B)"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--glob", "*.TXT", "--glob-case-insensitive", "tests/data"]),
        indoc!(
            "
            data (344.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn iglob() {
    assert_eq!(
        utils::run_cmd(&["--sort", "name", "--iglob", "*.TXT", "tests/data"]),
        indoc!(
            "
            data (344.00 B)
            ├─ necronomicon.txt (83.00 B)
            ├─ nemesis.txt (161.00 B)
            ├─ nylarlathotep.txt (100.00 B)
            └─ the_yellow_king"
        )
    )
}
