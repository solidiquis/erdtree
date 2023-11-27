use crate::{
    disk,
    user::{
        args::{Metric, TimeFormat, TimeStamp},
        Context,
    },
};
use ignore::DirEntry;
use std::{
    fmt::{self, Display},
    fs::{self, Metadata},
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

/// Concerned with querying information about a file's underlying inode.
pub mod inode;
use inode::{INodeError, Inode};

/// Rules on how to order entries relative to their siblings or all other files.
pub mod order;

/// Concerned with the tree data structure that is used to produce the program output.
pub mod tree;
pub use tree::Tree;

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
    symlink_target: Option<PathBuf>,

    #[cfg(unix)]
    unix_attrs: unix::Attrs,
}

pub struct DisplayName<'a> {
    file: &'a File,
}

impl File {
    /// Plain Jane constructor for [`File`].
    pub fn new(
        data: DirEntry,
        metadata: Metadata,
        size: disk::Usage,
        symlink_target: Option<PathBuf>,
        #[cfg(unix)] unix_attrs: unix::Attrs,
    ) -> Self {
        Self {
            data,
            metadata,
            size,
            symlink_target,
            #[cfg(unix)]
            unix_attrs,
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

        let (symlink_target, metadata) = if *follow {
            (fs::read_link(path).ok(), fs::metadata(path)?)
        } else {
            (None, fs::symlink_metadata(path)?)
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
            symlink_target,
            #[cfg(unix)]
            unix_attrs,
        ))
    }

    /// Attempts to query the [`File`]'s underlying inode which is represented by [`Inode`].
    pub fn inode(&self) -> Result<Inode, INodeError> {
        Inode::try_from(&self.metadata)
    }

    /// Reader for `metadata` field.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Gets a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut disk::Usage {
        &mut self.size
    }

    /// Gets an immmutable reference to the `size` field.
    pub fn size(&self) -> &disk::Usage {
        &self.size
    }

    pub fn symlink_target(&self) -> Option<&Path> {
        self.symlink_target.as_deref()
    }

    pub fn display_name(&self) -> DisplayName<'_> {
        DisplayName { file: self }
    }

    #[cfg(unix)]
    pub fn unix_attrs(&self) -> &unix::Attrs {
        &self.unix_attrs
    }

    #[cfg(unix)]
    pub fn timestamp_from_ctx(&self, ctx: &Context) -> Option<String> {
        use chrono::{DateTime, Local};

        let system_time = match ctx.time {
            TimeStamp::Mod => self.metadata().accessed().ok(),
            TimeStamp::Create => self.metadata().created().ok(),
            TimeStamp::Access => self.metadata().accessed().ok(),
        };

        system_time
            .map(DateTime::<Local>::from)
            .map(|local_time| match ctx.time_format {
                TimeFormat::Default => local_time.format("%d %h %H:%M %g"),
                TimeFormat::Iso => local_time.format("%Y-%m-%d %H:%M:%S"),
                TimeFormat::IsoStrict => local_time.format("%Y-%m-%dT%H:%M:%S%Z"),
                TimeFormat::Short => local_time.format("%Y-%m-%d"),
            })
            .map(|dt| format!("{dt}"))
    }

    pub fn is_dir(&self) -> bool {
        self.file_type().is_some_and(|ft| ft.is_dir())
    }
}

impl Deref for File {
    type Target = DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Display for DisplayName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_name = self.file.file_name().to_string_lossy();

        if let Some(link_target) = self.file.symlink_target() {
            write!(f, "{file_name} \u{2192} {}", link_target.display())
        } else {
            write!(f, "{file_name}")
        }
    }
}
