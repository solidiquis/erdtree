use super::{file_type::FileType, mode::Mode, SymbolicNotation};
use std::{error::Error, fs::File};

#[test]
fn test_symbolic_notation() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join("yogsothoth.hpl");
    let file = File::create(temp)?;
    let metadata = file.metadata()?;
    let persmissions = metadata.permissions();

    // Star of the show
    let file_mode = persmissions.try_mode_symbolic_notation()?;

    let file_type = file_mode.file_type();
    let user = file_mode.user_mode();
    let group = file_mode.group_mode();
    let other = file_mode.other_mode();

    assert_eq!(file_type, &FileType::File);
    assert_eq!(user, &Mode::ReadWrite);
    assert_eq!(group, &Mode::Read);
    assert_eq!(other, &Mode::Read);

    let rwx_string = format!("{file_mode}");

    assert_eq!(rwx_string, ".rw-r--r--");

    Ok(())
}
