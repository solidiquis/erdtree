use super::error::Error;
use nix::sys::stat::Mode as SMode;
use std::fmt::{self, Display};

pub enum Mode {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    WriteExecute,
    ReadWriteExecute,
}

impl Mode {
    pub fn try_user_mode_from(mode_mask: u32) -> Result<Self, Error> {
        let owner = mode_mask & u32::from(SMode::S_IRWXU.bits());

        let read = owner & u32::from(SMode::S_IRUSR.bits()) != 0;
        let write = owner & u32::from(SMode::S_IWUSR.bits()) != 0;
        let execute = owner & u32::from(SMode::S_IXUSR.bits()) != 0;

        Self::try_mode_from_rwx(read, write, execute)
    }

    pub fn try_group_mode_from(mode_mask: u32) -> Result<Self, Error> {
        let owner = mode_mask & u32::from(SMode::S_IRWXG.bits());

        let read = owner & u32::from(SMode::S_IRGRP.bits()) != 0;
        let write = owner & u32::from(SMode::S_IWGRP.bits()) != 0;
        let execute = owner & u32::from(SMode::S_IXGRP.bits()) != 0;

        Self::try_mode_from_rwx(read, write, execute)
    }

    pub fn try_other_mode_from(mode_mask: u32) -> Result<Self, Error> {
        let owner = mode_mask & u32::from(SMode::S_IRWXO.bits());

        let read = owner & u32::from(SMode::S_IROTH.bits()) != 0;
        let write = owner & u32::from(SMode::S_IWOTH.bits()) != 0;
        let execute = owner & u32::from(SMode::S_IXOTH.bits()) != 0;

        Self::try_mode_from_rwx(read, write, execute)
    }

    const fn try_mode_from_rwx(r: bool, w: bool, x: bool) -> Result<Self, Error> {
        match (r, w, x) {
            (true, false, false) => Ok(Self::Read),
            (false, true, false) => Ok(Self::Write),
            (false, false, true) => Ok(Self::Execute),
            (true, true, false) => Ok(Self::ReadWrite),
            (true, false, true) => Ok(Self::ReadExecute),
            (false, true, true) => Ok(Self::WriteExecute),
            (true, true, true) => Ok(Self::ReadWriteExecute),
            _ => Err(Error::UnknownMode),
        }
    }
}

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
        }
    }
}
