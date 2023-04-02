use indoc::indoc;

mod utils;

#[test]
fn glob() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "*.txt",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
        )
    )
}

#[test]
fn glob_negative() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "!*.txt",
            "tests/data"
        ]),
        indoc!(
            "
            data (143 B)
            ├─ dream_cycle 
            ├─ lipsum 
            └─ the_yellow_king (143 B)
               └─ cassildas_song.md (143 B)"
        )
    )
}

#[test]
fn glob_case_insensitive() {
    assert_eq!(
        utils::run_cmd(&[
            "--sort",
            "name",
            "--glob",
            "*.TXT",
            "--glob-case-insensitive",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
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
            "*.TXT",
            "tests/data"
        ]),
        indoc!(
            "
            data (1.07 KiB)
            ├─ dream_cycle (308 B)
            │  └─ polaris.txt (308 B)
            ├─ lipsum (446 B)
            │  └─ lipsum.txt (446 B)
            ├─ necronomicon.txt (83 B)
            ├─ nemesis.txt (161 B)
            ├─ nylarlathotep.txt (100 B)
            └─ the_yellow_king"
        )
    )
}
