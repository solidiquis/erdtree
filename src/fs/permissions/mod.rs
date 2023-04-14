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

pub mod file_type;

pub mod mode;

pub struct FileMode {
    file_type: FileType,
    user_mode: Mode,
    group_mode: Mode,
    other_mode: Mode,
}

impl FileMode {
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

    const fn file_type(&self) -> &FileType {
        &self.file_type
    }

    const fn user_mode(&self) -> &Mode {
        &self.user_mode
    }

    const fn group_mode(&self) -> &Mode {
        &self.group_mode
    }

    const fn other_mode(&self) -> &Mode {
        &self.other_mode
    }
}

impl Display for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_iden = self.file_type().identifier();
        let user_mode = self.user_mode();
        let group_mode = self.group_mode();
        let other_mode = self.other_mode();

        write!(f, "{file_iden}{user_mode}{group_mode}{other_mode}")
    }
}

impl TryFrom<Permissions> for FileMode {
    type Error = Error;

    fn try_from(permissions: Permissions) -> Result<Self, Self::Error> {
        let mode = permissions.mode();
        let file_type = FileType::try_from(mode)?;
        let mode_mask = mode & !u32::from(SFlag::S_IFMT.bits());
        let user_mode = Mode::try_user_mode_from(mode_mask)?;
        let group_mode = Mode::try_group_mode_from(mode_mask)?;
        let other_mode = Mode::try_other_mode_from(mode_mask)?;

        Ok(Self {
            file_type,
            user_mode,
            group_mode,
            other_mode,
        })
    }
}
