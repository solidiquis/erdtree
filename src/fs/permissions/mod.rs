use error::Error;
use file_type::FileType;
use mode::Mode;
use nix::sys::stat::SFlag;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    fs::Permissions,
    os::unix::fs::PermissionsExt,
};

/// File permission related errors.
pub mod error;

/// For working with Unix file identifiers.
pub mod file_type;

/// For working with permissions for a particular class i.e. user, group, or other.
pub mod mode;

#[cfg(test)]
mod test;

/// Trait that is used to extend [std::fs::Permissions] behavior such that it allows for `mode` to
/// be expressed in Unix's symbolic notation for file permissions.
pub trait SymbolicNotation: PermissionsExt {
    /// Attempts to return a [FileMode] which implements [Display] allowing it to be presented in
    /// symbolic notation for file permissions.
    fn try_mode_symbolic_notation(&self) -> Result<FileMode, Error> {
        let mode = self.mode();
        FileMode::try_from(mode)
    }
}

impl SymbolicNotation for Permissions {}

/// A struct which holds information about the permissions of a particular file. [FileMode]
/// implements [Display] which allows it to be conveniently represented in symbolic notation when
/// expressing file permissions.
pub struct FileMode {
    file_type: FileType,
    user_mode: Mode,
    group_mode: Mode,
    other_mode: Mode,
}

impl FileMode {
    /// Constructor for [FileMode].
    pub const fn new(
        file_type: FileType,
        user_mode: Mode,
        group_mode: Mode,
        other_mode: Mode,
    ) -> Self {
        Self {
            file_type,
            user_mode,
            group_mode,
            other_mode,
        }
    }

    /// Returns a reference to `file_type`.
    pub const fn file_type(&self) -> &FileType {
        &self.file_type
    }

    /// Returns a reference to a [Mode] which represents the permissions of the user class.
    pub const fn user_mode(&self) -> &Mode {
        &self.user_mode
    }

    /// Returns a reference to a [Mode] which represents the permissions of the group class.
    pub const fn group_mode(&self) -> &Mode {
        &self.group_mode
    }

    /// Returns a reference to a [Mode] which represents the permissions of the other class.
    pub const fn other_mode(&self) -> &Mode {
        &self.other_mode
    }
}

/// For representing [FileMode] in symbolic notation.
impl Display for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_iden = self.file_type().identifier();
        let user_mode = self.user_mode();
        let group_mode = self.group_mode();
        let other_mode = self.other_mode();

        write!(f, "{file_iden}{user_mode}{group_mode}{other_mode}")
    }
}

/// The argument `mode` is meant to come from the `mode` method of [std::fs::Permissions].
impl TryFrom<u32> for FileMode {
    type Error = Error;

    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        let file_type = FileType::try_from(mode)?;
        let modes_mask = mode & !u32::from(SFlag::S_IFMT.bits());
        let user_mode = Mode::try_user_mode_from(modes_mask)?;
        let group_mode = Mode::try_group_mode_from(modes_mask)?;
        let other_mode = Mode::try_other_mode_from(modes_mask)?;

        Ok(Self {
            file_type,
            user_mode,
            group_mode,
            other_mode,
        })
    }
}
