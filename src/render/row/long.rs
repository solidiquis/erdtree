use crate::{
    file::{
        inode::{INodeError, Inode},
        unix::{
            permissions::{FileModeXAttrs, SymbolicNotation},
            xattr::ExtendedAttr,
        },
        File,
    },
    user::{column, Context},
};
use std::{
    convert::From,
    fmt::{self, Display},
};

/// The width of the file-type, permissions of each class, as well as the indicator that there are
/// extended attributes e.g. `drwxrwxrwx@`.
const ATTRS_WIDTH: usize = 11;

/// e.g. 0744
const OCTAL_PERMISSIONS_WIDTH: usize = 4;

/// Use in place of a field that can't be computed in the output.
const PLACEHOLDER: &str = "-";

/// Data type whose [`Display`] implementation determines how the ls-like long-format should be
/// presented to the user.
pub struct Format<'a> {
    ctx: &'a Context,
    file: &'a File,
}

impl<'a> Format<'a> {
    pub fn new(file: &'a File, ctx: &'a Context) -> Self {
        Self { file, ctx }
    }
}

impl From<INodeError> for fmt::Error {
    fn from(_e: INodeError) -> Self {
        fmt::Error
    }
}

impl Display for Format<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Context {
            long:
                crate::user::Long {
                    group: enable_group,
                    ino: enable_ino,
                    nlink: enable_nlink,
                    octal: enable_octal,
                    ..
                },
            column_metadata,
            ..
        } = self.ctx;

        let timestamp = self
            .file
            .timestamp_from_ctx(self.ctx)
            .unwrap_or_else(|| String::from(PLACEHOLDER));

        let file_mode = self
            .file
            .metadata()
            .permissions()
            .try_mode_symbolic_notation()
            .map_err(|_e| fmt::Error)?;

        let attrs = self
            .file
            .has_xattrs()
            .then(|| format!("{}", FileModeXAttrs(&file_mode)))
            .unwrap_or_else(|| format!("{}", file_mode));

        let unix_attrs = self.file.unix_attrs();
        let owner = unix_attrs.owner().unwrap_or(PLACEHOLDER);

        match (enable_group, enable_ino, enable_nlink, enable_octal) {
            (false, false, false, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_time_width,
                    ..
                } = column_metadata;
                write!(
                    f,
                    "{attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {timestamp:>max_time_width$}"
                )
            },
            (true, false, false, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                write!(f, "{attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (false, true, false, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { ino, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (false, false, true, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { nlink, .. } = self.file.inode()?;
                write!(f, "{attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (true, true, false, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_ino_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { ino, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (true, false, true, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { nlink, .. } = self.file.inode()?;
                write!(f, "{attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (false, true, true, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { ino, nlink, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (true, true, true, false) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_nlink_width,
                    max_group_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { ino, nlink, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (false, false, false, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_time_width,
                    ..
                } = column_metadata;
                write!(f, "{file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (true, false, false, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                write!(f, "{file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (true, true, false, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_ino_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { ino, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (true, false, true, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_group_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { nlink, .. } = self.file.inode()?;
                write!(f, "{file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
            (false, false, true, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { nlink, .. } = self.file.inode()?;
                write!(f, "{file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (false, true, false, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { ino, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (false, true, true, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_nlink_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let Inode { ino, nlink, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {timestamp:>max_time_width$}")
            },
            (true, true, true, true) => {
                let column::Metadata {
                    max_owner_width,
                    max_ino_width,
                    max_nlink_width,
                    max_group_width,
                    max_time_width,
                    ..
                } = column_metadata;
                let group = unix_attrs.group().unwrap_or(PLACEHOLDER);
                let Inode { ino, nlink, .. } = self.file.inode()?;
                write!(f, "{ino:max_ino_width$} {file_mode:0OCTAL_PERMISSIONS_WIDTH$o} {attrs:<ATTRS_WIDTH$} {nlink:>max_nlink_width$} {owner:>max_owner_width$} {group:>max_group_width$} {timestamp:>max_time_width$}")
            },
        }
    }
}
