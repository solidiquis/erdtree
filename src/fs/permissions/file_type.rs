use super::error::Error;

/// Unix file types.
#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Directory,
    File,
    Symlink,
    Fifo,
    Socket,
    CharDevice,
    BlockDevice,
}

impl FileType {
    /// Unix file identifiers that you'd find in the `ls -l` command.
    pub const fn identifier(&self) -> char {
        match self {
            Self::Directory => 'd',
            Self::File => '-',
            Self::Symlink => 'l',
            Self::Fifo => 'p',
            Self::Socket => 's',
            Self::CharDevice => 'c',
            Self::BlockDevice => 'b',
        }
    }
}

/// The argument `mode` is meant to come from the `mode` method of [std::fs::Permissions].
impl TryFrom<u32> for FileType {
    type Error = Error;

    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        let file_mask = mode & u32::from(libc::S_IFMT);

        if file_mask == u32::from(libc::S_IFIFO) {
            Ok(Self::Fifo)
        } else if file_mask == u32::from(libc::S_IFCHR) {
            Ok(Self::CharDevice)
        } else if file_mask == u32::from(libc::S_IFDIR) {
            Ok(Self::Directory)
        } else if file_mask == u32::from(libc::S_IFBLK) {
            Ok(Self::BlockDevice)
        } else if file_mask == u32::from(libc::S_IFREG) {
            Ok(Self::File)
        } else if file_mask == u32::from(libc::S_IFLNK) {
            Ok(Self::Symlink)
        } else if file_mask == u32::from(libc::S_IFSOCK) {
            Ok(Self::Socket)
        } else {
            Err(Error::UnknownFileType)
        }
    }
}
