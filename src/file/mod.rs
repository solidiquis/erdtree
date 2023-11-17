use crate::{
    disk,
    user::{args::Metric, Context},
};
use ignore::DirEntry;
use std::{
    fs::{self, Metadata},
    io,
    ops::Deref,
};

/// Concerned with querying information about a file's underlying inode.
pub mod inode;
use inode::{INodeError, Inode};

/// File attributes specific to Unix systems.
#[cfg(unix)]
pub mod unix;

/// Erdtree's wrapper around [`DirEntry`], it's metadata ([`Metadata`]). Also contains disk usage
/// information of files. Directories will always be initialized to have a size of zero as they
/// must be recursively computed.
#[derive(Debug)]
pub struct File {
    data: DirEntry,
    metadata: Metadata,
    size: disk::Usage,
    #[cfg(unix)]
    unix_attrs: unix::Attrs,
}

impl File {
    /// Plain Jane constructor for [`File`].
    pub fn new(
        data: DirEntry,
        metadata: Metadata,
        size: disk::Usage,
        #[cfg(unix)]
        unix_attrs: unix::Attrs,
    ) -> Self {
        Self {
            data,
            metadata,
            size,
            #[cfg(unix)]
            unix_attrs
        }
    }

    /// Initializes [`File`] from the given [`DirEntry`] and [`Context`].
    pub fn init(
        data: DirEntry,
        Context {
            metric,
            byte_units,
            follow,
            #[cfg(unix)]
            long,
            ..
        }: &Context,
    ) -> Result<Self, io::Error> {
        let path = data.path();

        let metadata = if *follow {
            fs::metadata(path)?
        } else {
            fs::symlink_metadata(path)?
        };

        let size = match metric {
            Metric::Physical => disk::Usage::init_physical(&metadata, *byte_units),
            Metric::Logical => disk::Usage::init_logical(&metadata, *byte_units),
            Metric::Word => disk::Usage::init_word_count(&data, &metadata, *follow)?,
            Metric::Line => disk::Usage::init_line_count(&data, &metadata, *follow)?,

            #[cfg(unix)]
            Metric::Blocks => disk::Usage::init_blocks(&metadata),
        };

        #[cfg(unix)]
        let unix_attrs = long
            .then(|| unix::Attrs::from((&metadata, &data)))
            .unwrap_or_else(unix::Attrs::default);

        Ok(Self::new(
            data,
            metadata,
            size,
            #[cfg(unix)]
            unix_attrs
        ))
    }

    /// Attempts to query the [`File`]'s underlying inode which is represented by [`Inode`].
    pub fn inode(&self) -> Result<Inode, INodeError> {
        Inode::try_from(&self.metadata)
    }

    /// Gets a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut disk::Usage {
        &mut self.size
    }

    /// Gets an immmutable reference to the `size` field.
    pub fn size(&self) -> &disk::Usage {
        &self.size
    }
}

impl Deref for File {
    type Target = DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
