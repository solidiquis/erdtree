use super::{class::PermissionsTriad, file_type::FileType, SymbolicNotation};
use std::{error::Error, fs::File, os::unix::fs::PermissionsExt};

#[test]
fn test_symbolic_notation() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join("yogsothoth.hpl");

    // File is created with read + write for user and read-only for all others.
    let file = File::create(temp)?;
    let metadata = file.metadata()?;

    let permissions = metadata.permissions();

    let file_mode = permissions.try_mode_symbolic_notation()?;

    let file_type = file_mode.file_type();
    let user = file_mode.user_permissions();
    let group = file_mode.group_permissions();
    let other = file_mode.other_permissions();

    assert_eq!(file_type, &FileType::File);
    assert_eq!(&user.triad, &PermissionsTriad::ReadWrite);
    assert_eq!(&group.triad, &PermissionsTriad::Read);
    assert_eq!(&other.triad, &PermissionsTriad::Read);

    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rw-r--r--");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "644");

    Ok(())
}

#[test]
fn test_symbolic_notation_special_attr() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join("sub-niggurath.hpl");

    // File is created with read + write for user and read-only for all others.
    let file = File::create(temp)?;

    let metadata = file.metadata()?;
    let mut permissions = metadata.permissions();

    // Set the sticky bit
    permissions.set_mode(0o101_644);

    let file_mode = permissions.try_mode_symbolic_notation()?;
    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rw-r--r-T");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "1644");

    // Set the getuid bit
    permissions.set_mode(0o102_644);

    let file_mode = permissions.try_mode_symbolic_notation()?;
    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rw-r-Sr--");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "2644");

    // Set the setuid bit
    permissions.set_mode(0o104_644);

    let file_mode = permissions.try_mode_symbolic_notation()?;
    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rwSr--r--");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "4644");

    // Set the all the attr bits
    permissions.set_mode(0o107_644);

    let file_mode = permissions.try_mode_symbolic_notation()?;
    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rwSr-Sr-T");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "7644");

    // Set all the attr bits and give all classes execute permissions
    permissions.set_mode(0o107_777);

    let file_mode = permissions.try_mode_symbolic_notation()?;
    let rwx = format!("{file_mode}");
    assert_eq!(rwx, ".rwsrwsrwt");

    let octal = format!("{file_mode:o}");
    assert_eq!(octal, "7777");

    Ok(())
}
