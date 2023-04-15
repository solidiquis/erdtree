use super::error::Error;
use nix::sys::stat::SFlag;

/// Unix file types.
#[derive(Debug, PartialEq)]
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
            Self::File => '.',
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
        let file_mask = mode & u32::from(SFlag::S_IFMT.bits());

        if file_mask == u32::from(SFlag::S_IFIFO.bits()) {
            Ok(Self::Fifo)
        } else if file_mask == u32::from(SFlag::S_IFCHR.bits()) {
            Ok(Self::CharDevice)
        } else if file_mask == u32::from(SFlag::S_IFDIR.bits()) {
            Ok(Self::Directory)
        } else if file_mask == u32::from(SFlag::S_IFBLK.bits()) {
            Ok(Self::BlockDevice)
        } else if file_mask == u32::from(SFlag::S_IFREG.bits()) {
            Ok(Self::File)
        } else if file_mask == u32::from(SFlag::S_IFLNK.bits()) {
            Ok(Self::Symlink)
        } else if file_mask == u32::from(SFlag::S_IFSOCK.bits()) {
            Ok(Self::Socket)
        } else {
            Err(Error::UnknownFileType)
        }
    }
}
