use super::error::Error;
use std::fmt::{self, Display};

/// The set of permissions for a particular class i.e. user, group, or other.
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    WriteExecute,
    ReadWriteExecute,
    None,
}

/// All `modes_mask` arguments represent the portions of `st_mode` which excludes the file-type.
impl Mode {
    /// Computes user permissions.
    pub fn try_user_mode_from(modes_mask: u32) -> Result<Self, Error> {
        let user = modes_mask & u32::from(libc::S_IRWXU);

        let read = Self::enabled(user, libc::S_IRUSR);
        let write = Self::enabled(user, libc::S_IWUSR);
        let execute = Self::enabled(user, libc::S_IXUSR);

        Self::try_mode_from_rwx(read, write, execute)
    }

    /// Computes group permissions.
    pub fn try_group_mode_from(modes_mask: u32) -> Result<Self, Error> {
        let group = modes_mask & u32::from(libc::S_IRWXG);

        let read = Self::enabled(group, libc::S_IRGRP);
        let write = Self::enabled(group, libc::S_IWGRP);
        let execute = Self::enabled(group, libc::S_IXGRP);

        Self::try_mode_from_rwx(read, write, execute)
    }

    /// Computes other permissions.
    pub fn try_other_mode_from(modes_mask: u32) -> Result<Self, Error> {
        let other = modes_mask & u32::from(libc::S_IRWXO);

        let read = Self::enabled(other, libc::S_IROTH);
        let write = Self::enabled(other, libc::S_IWOTH);
        let execute = Self::enabled(other, libc::S_IXOTH);

        Self::try_mode_from_rwx(read, write, execute)
    }

    /// Checks if a particular mode (read, write, or execute) is enabled.
    fn enabled<N>(class_mask: u32, mode_mask: N) -> bool
    where
        N: Copy + Into<u32>,
    {
        class_mask & mode_mask.into() == mode_mask.into()
    }

    /// Helper function to compute permissions.
    const fn try_mode_from_rwx(r: bool, w: bool, x: bool) -> Result<Self, Error> {
        match (r, w, x) {
            (true, false, false) => Ok(Self::Read),
            (false, true, false) => Ok(Self::Write),
            (false, false, true) => Ok(Self::Execute),
            (true, true, false) => Ok(Self::ReadWrite),
            (true, false, true) => Ok(Self::ReadExecute),
            (false, true, true) => Ok(Self::WriteExecute),
            (true, true, true) => Ok(Self::ReadWriteExecute),
            (false, false, false) => Ok(Self::None),
        }
    }
}

/// The `rwx` representation of a [Mode].
impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "r--"),
            Self::Write => write!(f, "-w-"),
            Self::Execute => write!(f, "--x"),
            Self::ReadWrite => write!(f, "rw-"),
            Self::ReadExecute => write!(f, "r-x"),
            Self::WriteExecute => write!(f, "-wx"),
            Self::ReadWriteExecute => write!(f, "rwx"),
            Self::None => write!(f, "---"),
        }
    }
}
