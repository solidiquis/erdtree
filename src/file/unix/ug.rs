use errno::{errno, set_errno, Errno};
use std::{ffi::CStr, fs::Metadata, os::unix::fs::MetadataExt};

type Owner = String;
type Group = String;

impl UserGroupInfo for Metadata {}

/// Trait that allows for files to query their owner and group.
pub trait UserGroupInfo: MetadataExt {
    /// Attemps to query the owner of the implementor.
    fn try_get_owner(&self) -> Result<String, Error> {
        unsafe {
            let uid = self.uid();
            try_get_user(uid)
        }
    }

    /// Attempts to query both the owner and group of the implementor.
    fn try_get_owner_and_group(&self) -> Result<(Owner, Group), Error> {
        unsafe {
            let uid = self.uid();
            let gid = self.gid();
            let user = try_get_user(uid)?;
            let group = try_get_group(gid)?;

            Ok((user, group))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("libc error")]
    LibC(Errno),

    #[error("Invalid user")]
    InvalidUser,

    #[error("Invalid group")]
    InvalidGroup,
}

/// Attempts to return the name of the group associated with `gid`.
unsafe fn try_get_group(gid: libc::gid_t) -> Result<String, Error> {
    set_errno(Errno(0));

    let group = libc::getgrgid(gid);

    let errno = errno();

    if errno.0 != 0 {
        return Err(Error::LibC(errno));
    }

    if group.is_null() {
        return Err(Error::InvalidGroup);
    }

    let libc::group { gr_name, .. } = *group;

    Ok(CStr::from_ptr(gr_name).to_string_lossy().to_string())
}

/// Attempts to return the name of the user associated with `uid`.
unsafe fn try_get_user(uid: libc::uid_t) -> Result<String, Error> {
    set_errno(Errno(0));

    let pwd = libc::getpwuid(uid);

    let errno = errno();

    if errno.0 != 0 {
        return Err(Error::LibC(errno));
    }

    if pwd.is_null() {
        return Err(Error::InvalidUser);
    }

    let libc::passwd { pw_name, .. } = *pwd;

    Ok(CStr::from_ptr(pw_name).to_string_lossy().to_string())
}
