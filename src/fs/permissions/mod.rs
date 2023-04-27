use error::Error;
use file_type::FileType;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Octal},
    os::unix::fs::PermissionsExt,
};

/// For working with permissions for a particular class i.e. user, group, or other.
pub mod class;

/// File permission related errors.
pub mod error;

/// For working with Unix file identifiers.
pub mod file_type;

#[cfg(test)]
mod test;

impl SymbolicNotation for std::fs::Permissions {}

/// Trait that is used to extend [`std::fs::Permissions`] behavior such that it allows for `mode` to
/// be expressed in Unix's symbolic notation for file permissions.
pub trait SymbolicNotation: PermissionsExt {
    /// Attempts to return a [`FileMode`] which implements [Display] allowing it to be presented in
    /// symbolic notation for file permissions.
    fn try_mode_symbolic_notation(&self) -> Result<FileMode, Error> {
        let mode = self.mode();
        FileMode::try_from(mode)
    }
}

/// A struct which holds information about the permissions of a particular file. [`FileMode`]
/// implements [Display] which allows it to be conveniently presented in symbolic notation when
/// expressing file permissions.
pub struct FileMode {
    pub st_mode: u32,
    file_type: FileType,
    user_permissions: class::Permissions,
    group_permissions: class::Permissions,
    other_permissions: class::Permissions,
}

impl FileMode {
    /// Constructor for [`FileMode`].
    pub const fn new(
        st_mode: u32,
        file_type: FileType,
        user_permissions: class::Permissions,
        group_permissions: class::Permissions,
        other_permissions: class::Permissions,
    ) -> Self {
        Self {
            st_mode,
            file_type,
            user_permissions,
            group_permissions,
            other_permissions,
        }
    }

    /// Returns a reference to `file_type`.
    pub const fn file_type(&self) -> &FileType {
        &self.file_type
    }

    /// Returns a reference to a [`class::Permissions`] which represents the permissions of the user class.
    pub const fn user_permissions(&self) -> &class::Permissions {
        &self.user_permissions
    }

    /// Returns a reference to a [`class::Permissions`] which represents the permissions of the group class.
    pub const fn group_permissions(&self) -> &class::Permissions {
        &self.group_permissions
    }

    /// Returns a reference to a [`class::Permissions`] which represents the permissions of the other class.
    pub const fn other_permissions(&self) -> &class::Permissions {
        &self.other_permissions
    }
}

/// For representing [`FileMode`] in symbolic notation.
impl Display for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_iden = self.file_type().identifier();
        let user_permissions = self.user_permissions();
        let group_permissions = self.group_permissions();
        let other_permissions = self.other_permissions();

        write!(
            f,
            "{file_iden}{user_permissions}{group_permissions}{other_permissions}"
        )
    }
}

/// For the octal representation of permissions
impl Octal for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modes_mask = self.st_mode & !u32::from(libc::S_IFMT);
        fmt::Octal::fmt(&modes_mask, f)
    }
}

/// The argument `st_mode` is meant to come from the `mode` method of [`std::fs::Permissions`].
impl TryFrom<u32> for FileMode {
    type Error = Error;

    fn try_from(st_mode: u32) -> Result<Self, Self::Error> {
        let file_type = FileType::try_from(st_mode)?;
        let user_permissions = class::Permissions::user_permissions_from(st_mode);
        let group_permissions = class::Permissions::group_permissions_from(st_mode);
        let other_permissions = class::Permissions::other_permissions_from(st_mode);

        Ok(Self::new(
            st_mode,
            file_type,
            user_permissions,
            group_permissions,
            other_permissions,
        ))
    }
}
