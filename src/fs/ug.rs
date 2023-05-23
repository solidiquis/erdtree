use errno::{errno, set_errno, Errno};
use ignore::DirEntry;
use std::{convert::AsRef, ffi::CStr, mem, os::unix::ffi::OsStrExt, path::Path, ptr};

impl UserGroupInfo for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
}

type Owner = String;
type Group = String;

/// Trait that allows for files to query their owner and group.
pub trait UserGroupInfo {
    fn path(&self) -> &Path;

    /// Attemps to query the owner of the implementor.
    fn try_get_owner(&self) -> Result<String, Errno> {
        unsafe {
            let libc::stat { st_uid, .. } = try_init_stat(self.path())?;
            try_get_user(st_uid)
        }
    }

    /// Attempts to query both the owner and group of the implementor.
    fn try_get_owner_and_group(&self) -> Result<(Owner, Group), Errno> {
        unsafe {
            let libc::stat { st_uid, st_gid, .. } = try_init_stat(self.path())?;
            let user = try_get_user(st_uid)?;
            let group = try_get_group(st_gid)?;

            Ok((user, group))
        }
    }
}

/// A wrapper around [`libc::stat`].
unsafe fn try_init_stat<P: AsRef<Path>>(path: P) -> Result<libc::stat, Errno> {
    let mut stat = mem::zeroed::<libc::stat>();

    let stat_ptr = ptr::addr_of_mut!(stat);
    let path_ptr = path
        .as_ref()
        .as_os_str()
        .as_bytes()
        .as_ptr()
        .cast::<libc::c_char>();

    if libc::stat(path_ptr, stat_ptr) == -1 {
        return Err(errno());
    }

    Ok(stat)
}

/// Attempts to return the name of the group associated with `gid`.
unsafe fn try_get_group(gid: libc::gid_t) -> Result<String, Errno> {
    set_errno(Errno(0));

    let group = libc::getgrgid(gid);

    let errno = errno();

    if errno.0 != 0 {
        return Err(errno);
    }

    let libc::group { gr_name, .. } = *group;

    Ok(CStr::from_ptr(gr_name).to_string_lossy().to_string())
}

/// Attempts to return the name of the user associated with `uid`.
unsafe fn try_get_user(uid: libc::uid_t) -> Result<String, Errno> {
    set_errno(Errno(0));

    let pwd = libc::getpwuid(uid);

    let errno = errno();

    if errno.0 != 0 {
        return Err(errno);
    }

    let libc::passwd { pw_name, .. } = *pwd;

    Ok(CStr::from_ptr(pw_name).to_string_lossy().to_string())
}
