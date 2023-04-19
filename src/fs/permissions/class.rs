use std::fmt::{self, Display};

/// The set of permissions for a particular class i.e. user, group, or other.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ClassPermissions {
    class: Class,
    attr: Option<Attribute>,
    pub(super) triad: PermissionsTriad,
}

/// The class type that is associated with a permissions triad.
#[derive(Debug)]
pub enum Class {
    User,
    Group,
    Other,
}

/// Represents the special attributes that exist on the overall file corresponding to the setuid,
/// setgid, and the sticky bit.
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Attribute {
    SUID,
    SGID,
    Sticky,
}

/// Read, write, execute permissions.
#[derive(Debug, PartialEq, Eq)]
pub enum PermissionsTriad {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    WriteExecute,
    ReadWriteExecute,
    None,
}

/// All `permissions_mask` arguments represents the bits of `st_mode` which excludes the file-type
/// and the setuid, setgid, and sticky bit.
impl ClassPermissions {
    /// Computes user permissions.
    pub fn user_permissions_from(st_mode: u32) -> Self {
        let read = Self::enabled(st_mode, libc::S_IRUSR);
        let write = Self::enabled(st_mode, libc::S_IWUSR);
        let execute = Self::enabled(st_mode, libc::S_IXUSR);
        let suid = Self::enabled(st_mode, libc::S_ISUID).then_some(Attribute::SUID);

        Self::permissions_from_rwx(Class::User, read, write, execute, suid)
    }

    /// Computes group permissions.
    pub fn group_permissions_from(st_mode: u32) -> Self {
        let read = Self::enabled(st_mode, libc::S_IRGRP);
        let write = Self::enabled(st_mode, libc::S_IWGRP);
        let execute = Self::enabled(st_mode, libc::S_IXGRP);
        let sgid = Self::enabled(st_mode, libc::S_ISGID).then_some(Attribute::SGID);

        Self::permissions_from_rwx(Class::Group, read, write, execute, sgid)
    }

    /// Computes other permissions.
    pub fn other_permissions_from(st_mode: u32) -> Self {
        let read = Self::enabled(st_mode, libc::S_IROTH);
        let write = Self::enabled(st_mode, libc::S_IWOTH);
        let execute = Self::enabled(st_mode, libc::S_IXOTH);
        let sticky = Self::enabled(st_mode, libc::S_ISVTX).then_some(Attribute::Sticky);

        Self::permissions_from_rwx(Class::Other, read, write, execute, sticky)
    }

    /// Checks if a particular mode (read, write, or execute) is enabled.
    fn enabled<N>(st_mode: u32, mask: N) -> bool
    where
        N: Copy + Into<u32>,
    {
        st_mode & mask.into() == mask.into()
    }

    /// Returns `true` if sticky bit is enabled.
    pub fn attr_is_sticky(&self) -> bool {
        self.attr
            .as_ref()
            .map_or(false, |attr| attr == &Attribute::Sticky)
    }

    /// Helper function to compute permissions.
    const fn permissions_from_rwx(
        class: Class,
        r: bool,
        w: bool,
        x: bool,
        attr: Option<Attribute>,
    ) -> Self {
        let triad = match (r, w, x) {
            (true, false, false) => PermissionsTriad::Read,
            (false, true, false) => PermissionsTriad::Write,
            (false, false, true) => PermissionsTriad::Execute,
            (true, true, false) => PermissionsTriad::ReadWrite,
            (true, false, true) => PermissionsTriad::ReadExecute,
            (false, true, true) => PermissionsTriad::WriteExecute,
            (true, true, true) => PermissionsTriad::ReadWriteExecute,
            (false, false, false) => PermissionsTriad::None,
        };

        Self { class, attr, triad }
    }
}

/// The symbolic representation of a [PermissionsTriad].
impl Display for ClassPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.class {
            Class::Other if self.attr_is_sticky() => match self.triad {
                PermissionsTriad::Read => write!(f, "r-T"),
                PermissionsTriad::Write => write!(f, "-wT"),
                PermissionsTriad::Execute => write!(f, "--t"),
                PermissionsTriad::ReadWrite => write!(f, "rwT"),
                PermissionsTriad::ReadExecute => write!(f, "r-t"),
                PermissionsTriad::WriteExecute => write!(f, "-wt"),
                PermissionsTriad::ReadWriteExecute => write!(f, "rwt"),
                PermissionsTriad::None => write!(f, "--T"),
            },

            _ if self.attr.is_some() => match self.triad {
                PermissionsTriad::Read => write!(f, "r-S"),
                PermissionsTriad::Write => write!(f, "-wS"),
                PermissionsTriad::Execute => write!(f, "--s"),
                PermissionsTriad::ReadWrite => write!(f, "rwS"),
                PermissionsTriad::ReadExecute => write!(f, "r-s"),
                PermissionsTriad::WriteExecute => write!(f, "-ws"),
                PermissionsTriad::ReadWriteExecute => write!(f, "rws"),
                PermissionsTriad::None => write!(f, "--S"),
            },

            _ => match self.triad {
                PermissionsTriad::Read => write!(f, "r--"),
                PermissionsTriad::Write => write!(f, "-w-"),
                PermissionsTriad::Execute => write!(f, "--x"),
                PermissionsTriad::ReadWrite => write!(f, "rw-"),
                PermissionsTriad::ReadExecute => write!(f, "r-x"),
                PermissionsTriad::WriteExecute => write!(f, "-wx"),
                PermissionsTriad::ReadWriteExecute => write!(f, "rwx"),
                PermissionsTriad::None => write!(f, "---"),
            },
        }
    }
}
