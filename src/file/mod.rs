use crate::{
    disk, icon,
    user::{args::Metric, Context},
};
use ignore::DirEntry;
use std::{
    fmt::{self, Display},
    fs::{self, Metadata},
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use crate::{
    file::unix::permissions::{file_type::FileType, SymbolicNotation},
    user::args::{TimeFormat, TimeStamp},
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

// For keeping track of the count of file-types while loading from disk.
#[derive(Default)]
pub struct Accumulator {
    num_file: usize,
    num_dir: usize,
    num_link: usize,
}

/// [`Display`] implementation concerned with human-readable presentation of the file-name.
pub struct DisplayName<'a> {
    file: &'a File,
}

/// [`Display`] implementation concerned with human-readable presentation of the file-path.
pub struct DisplayPath<'a> {
    file: &'a File,
    path_prefix: Option<&'a Path>,
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

        let (symlink_target, metadata) = if data.file_type().is_some_and(|ft| ft.is_symlink()) {
            if *follow {
                (fs::read_link(path).ok(), fs::metadata(path)?)
            } else {
                (fs::read_link(path).ok(), fs::symlink_metadata(path)?)
            }
        } else {
            (None, fs::symlink_metadata(path)?)
        };

        let size = match metric {
            Metric::Physical => disk::Usage::init_physical(&metadata, *byte_units),
            Metric::Logical => disk::Usage::init_logical(&metadata, *byte_units),
            Metric::Word => disk::Usage::init_word_count(&data, &metadata, *follow)?,
            Metric::Line => disk::Usage::init_line_count(&data, &metadata, *follow)?,

            #[cfg(unix)]
            Metric::Block => disk::Usage::init_blocks(&metadata),
        };

        #[cfg(unix)]
        let unix_attrs = long
            .long
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

    pub fn display_path<'a>(&'a self, path_prefix: Option<&'a Path>) -> DisplayPath<'a> {
        DisplayPath {
            file: self,
            path_prefix,
        }
    }

    pub fn icon(&self) -> &str {
        icon::compute(self)
    }

    #[cfg(unix)]
    pub fn unix_attrs(&self) -> &unix::Attrs {
        &self.unix_attrs
    }

    #[cfg(unix)]
    pub fn timestamp_from_ctx(&self, ctx: &Context) -> Option<String> {
        use chrono::{DateTime, Local};

        let system_time = match ctx.long.time.unwrap_or_default() {
            TimeStamp::Mod => self.metadata().accessed().ok(),
            TimeStamp::Create => self.metadata().created().ok(),
            TimeStamp::Access => self.metadata().accessed().ok(),
        };

        system_time
            .map(DateTime::<Local>::from)
            .map(
                |local_time| match ctx.long.time_format.unwrap_or_default() {
                    TimeFormat::Default => local_time.format("%d %h %H:%M %g"),
                    TimeFormat::Iso => local_time.format("%Y-%m-%d %H:%M:%S"),
                    TimeFormat::IsoStrict => local_time.format("%Y-%m-%dT%H:%M:%S%Z"),
                    TimeFormat::Short => local_time.format("%Y-%m-%d"),
                },
            )
            .map(|dt| format!("{dt}"))
    }

    #[cfg(unix)]
    pub fn is_fifo(&self) -> bool {
        self.metadata()
            .permissions()
            .try_mode_symbolic_notation()
            .map_or(false, |mode| mode.file_type() == &FileType::Fifo)
    }

    #[cfg(unix)]
    pub fn is_socket(&self) -> bool {
        self.metadata()
            .permissions()
            .try_mode_symbolic_notation()
            .map_or(false, |mode| mode.file_type() == &FileType::Socket)
    }

    #[cfg(unix)]
    pub fn is_char_device(&self) -> bool {
        self.metadata()
            .permissions()
            .try_mode_symbolic_notation()
            .map_or(false, |mode| mode.file_type() == &FileType::CharDevice)
    }

    #[cfg(unix)]
    pub fn is_block_device(&self) -> bool {
        self.metadata()
            .permissions()
            .try_mode_symbolic_notation()
            .map_or(false, |mode| mode.file_type() == &FileType::BlockDevice)
    }

    pub fn is_dir(&self) -> bool {
        self.file_type().is_some_and(|ft| ft.is_dir())
    }

    pub fn is_file(&self) -> bool {
        self.file_type().is_some_and(|ft| ft.is_file())
    }

    pub fn is_symlink(&self) -> bool {
        self.file_type().is_some_and(|ft| ft.is_symlink())
    }
}

impl Deref for File {
    type Target = DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Accumulator {
    pub fn total(&self) -> usize {
        self.num_dir + self.num_file + self.num_link
    }

    pub fn increment(&mut self, ft: Option<fs::FileType>) {
        let Some(file_type) = ft else { return };

        if file_type.is_file() {
            self.num_file += 1;
        } else if file_type.is_dir() {
            self.num_dir += 1;
        } else if file_type.is_symlink() {
            self.num_link += 1;
        }
    }
}

impl Display for DisplayName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_name = self.file.file_name().to_string_lossy();
        let link_target = self.file.symlink_target().map(|p| p.canonicalize());

        if let Some(Ok(target)) = link_target {
            write!(f, "{file_name} \u{2192} {}", target.display())
        } else {
            write!(f, "{file_name}")
        }
    }
}

impl Display for DisplayPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self.path_prefix {
            Some(prefix) => {
                let path = self.file.path();
                path.strip_prefix(prefix)
                    .map_or_else(|_| path.display(), |p| p.display())
            }
            None => self.file.path().display(),
        };

        let link_target = self.file.symlink_target().map(|p| p.canonicalize());

        if let Some(Ok(target)) = link_target {
            write!(f, "{display} \u{2192} {}", target.display())
        } else {
            write!(f, "{display}")
        }
    }
}

impl Display for Accumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut displayed_count = Vec::new();

        match self.num_dir {
            n if n > 1 => displayed_count.push(format!("{} directories", n)),
            1 => displayed_count.push("1 directory".to_string()),
            _ => (),
        }

        match self.num_file {
            n if n > 1 => displayed_count.push(format!("{} files", n)),
            1 => displayed_count.push("1 file".to_string()),
            _ => (),
        }

        match self.num_link {
            n if n > 1 => displayed_count.push(format!("{} links", n)),
            1 => displayed_count.push("1 link".to_string()),
            _ => (),
        }

        writeln!(f, "{}", displayed_count.join(", "))
    }
}