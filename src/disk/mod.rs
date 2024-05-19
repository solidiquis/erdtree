use crate::user::args::BytePresentation;
use ignore::DirEntry;
use std::{
    fmt::{self, Display},
    fs::{self, Metadata},
    io,
    ops::AddAssign,
};

/// Binary and SI prefixes.
pub mod prefix;

/// <https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.blocks>
#[cfg(unix)]
const BLOCK_SIZE: u64 = 512;

/// Padding between the numerical value the byte-unit, "B" used when reporting bytes.
pub const RAW_PADDING: usize = 2;

/// Padding between the numerical value and the SI units used when reporting bytes.
pub const SI_PADDING: usize = 3;

/// Padding between the numerical value and the binary units used when reporting bytes.
pub const BIN_PADDING: usize = 4;

/// Precision to use when reporting bytes in human-readable format.
pub const FLOAT_PRECISION: usize = 1;

/// Different metrics for reporting file size.
#[derive(Debug)]
pub enum Usage {
    /// Apparent size in bytes rather than actual disk usage.
    Logical {
        value: u64,
        presentation: BytePresentation,
    },

    /// The amount of bytes used to store the relevant file on disk.
    Physical {
        value: u64,
        presentation: BytePresentation,
    },

    /// The amount of blocks used to store the relevant file on disk.
    #[cfg(unix)]
    Blocks(u64),

    /// The total amount of words in a file
    WordCount(u64),

    /// The total amount of lines in a file
    LineCount(u64),
}

impl Usage {
    /// Gets the actual bytes stored on disk for a particular file. Directory sizes must be
    /// recursively computed so they will be initialized to a size of 0.
    #[cfg(unix)]
    pub fn init_physical(metadata: &Metadata, presentation: BytePresentation) -> Self {
        use std::os::unix::fs::MetadataExt;

        let value = metadata
            .is_dir()
            .then_some(0)
            .unwrap_or_else(|| metadata.blocks() * BLOCK_SIZE);

        Self::Physical {
            value,
            presentation,
        }
    }

    /// Gets the actual bytes stored on disk for a particular file. Directory sizes must be
    /// recursively computed so they will be initialized to a size of 0.
    #[cfg(windows)]
    pub fn init_physical(metadata: &Metadata, presentation: BytePresentation) -> Self {
        use std::os::windows::fs::MetadataExt;

        let value = metadata
            .is_dir()
            .then_some(0)
            .unwrap_or_else(|| metadata.file_size());

        Self::Physical {
            value,
            presentation,
        }
    }

    #[cfg(not(any(windows, unix)))]
    pub fn init_physical(metadata: &Metadata, presentation: BytePresentation) -> Self {
        Self::init_logical(metadata, presentation)
    }

    /// Gets the apparent file size rather than disk usage. Refer to `--apparent-size` in the man
    /// pages of `du`: <https://man7.org/linux/man-pages/man1/du.1.html>
    pub fn init_logical(metadata: &Metadata, presentation: BytePresentation) -> Self {
        let value = metadata.is_dir().then_some(0).unwrap_or(metadata.len());

        Self::Logical {
            value,
            presentation,
        }
    }

    /// Gets the word count. Words are delimited by a whitespace or a sequence of whitespaces.
    /// Directories are initialized to 0. The `follow` argument determines whether or not to query the
    /// symlink target, otherwise the symlink will have a word count of 0.
    pub fn init_word_count(
        data: &DirEntry,
        metadata: &Metadata,
        follow: bool,
    ) -> Result<Self, io::Error> {
        if metadata.is_dir() || (metadata.is_symlink() && !follow) {
            return Ok(Self::WordCount(0));
        }

        let word_count =
            std::fs::read_to_string(data.path()).map(|data| data.split_whitespace().count())?;

        let word_count = u64::try_from(word_count)
            .map_or_else(|_| Self::WordCount(word_count as u64), Self::WordCount);

        Ok(word_count)
    }

    /// Gets the line count. Lines are delimited by the new-line ASCII char. Directories are
    /// initialized to 0. The `follow` argument determines whether or not to query the symlink
    /// target, otherwise the symlink will have a count of 0.
    pub fn init_line_count(
        data: &DirEntry,
        metadata: &Metadata,
        follow: bool,
    ) -> Result<Self, io::Error> {
        if metadata.is_dir() || (metadata.is_symlink() && !follow) {
            return Ok(Self::LineCount(0));
        }

        let line_count = fs::read_to_string(data.path()).map(|data| data.lines().count())?;

        let line_count = u64::try_from(line_count)
            .map_or_else(|_| Self::WordCount(line_count as u64), Self::LineCount);

        Ok(line_count)
    }

    /// Gets the underlying numeric value representing the disk usage
    pub fn value(&self) -> u64 {
        match self {
            Self::WordCount(count) => *count,
            Self::LineCount(count) => *count,
            Self::Logical { value, .. } => *value,
            Self::Physical { value, .. } => *value,

            #[cfg(unix)]
            Self::Blocks(blocks) => *blocks,
        }
    }

    /// Gets the actual amount of blocks allocated to a particular file. Directories are
    /// initialized to 0.
    #[cfg(unix)]
    pub fn init_blocks(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        let value = metadata.blocks();
        Self::Blocks(value)
    }
}

impl Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! byte_display {
            ($p:expr, $v:expr) => {
                match $p {
                    BytePresentation::Raw => write!(f, "{}{:>RAW_PADDING$}", $v, "B"),
                    BytePresentation::Bin => {
                        let prefix = prefix::Binary::from($v);

                        if matches!(prefix, prefix::Binary::Base) {
                            write!(f, "{}{:>BIN_PADDING$}", $v, "B")
                        } else {
                            let bytes = ($v as f64) / prefix.base_value();
                            write!(f, "{bytes:.FLOAT_PRECISION$} {prefix}B")
                        }
                    }
                    BytePresentation::Si => {
                        let prefix = prefix::Si::from($v);

                        if matches!(prefix, prefix::Si::Base) {
                            write!(f, "{}{:>SI_PADDING$}", $v, "B")
                        } else {
                            let bytes = ($v as f64) / prefix.base_value();
                            write!(f, "{bytes:.1} {prefix}B")
                        }
                    }
                }
            };
        }

        match self {
            Self::WordCount(count) => <u64 as Display>::fmt(count, f),
            Self::LineCount(count) => <u64 as Display>::fmt(count, f),
            Self::Logical {
                value,
                presentation,
            } => byte_display!(presentation, *value),
            Self::Physical {
                value,
                presentation,
            } => byte_display!(presentation, *value),

            #[cfg(unix)]
            Self::Blocks(blocks) => <u64 as Display>::fmt(blocks, f),
        }
    }
}

impl AddAssign<u64> for Usage {
    fn add_assign(&mut self, rhs: u64) {
        match self {
            Self::WordCount(count) => *count += rhs,
            Self::LineCount(count) => *count += rhs,
            Self::Logical { value, .. } => *value += rhs,
            Self::Physical { value, .. } => *value += rhs,

            #[cfg(unix)]
            Self::Blocks(blocks) => *blocks += rhs,
        }
    }
}

#[test]
fn test_bytes_display() {
    let size = Usage::Physical {
        value: 998,
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("998   B"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(10),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 KiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(20),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 MiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(30),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 GiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(40),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 TiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(50),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 PiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 2_u64.pow(30),
        presentation: BytePresentation::Bin,
    };

    assert_eq!(String::from("1.0 GiB"), format!("{size}"));

    let size = Usage::Physical {
        value: 10_u64.pow(3),
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("1.0 KB"), format!("{size}"));

    let size = Usage::Physical {
        value: 10_u64.pow(6),
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("1.0 MB"), format!("{size}"));

    let size = Usage::Physical {
        value: 10_u64.pow(9),
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("1.0 GB"), format!("{size}"));

    let size = Usage::Physical {
        value: 10_u64.pow(12),
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("1.0 TB"), format!("{size}"));

    let size = Usage::Physical {
        value: 10_u64.pow(15),
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("1.0 PB"), format!("{size}"));

    let size = Usage::Physical {
        value: 998,
        presentation: BytePresentation::Si,
    };

    assert_eq!(String::from("998  B"), format!("{size}"));

    let size = Usage::Physical {
        value: 1000,
        presentation: BytePresentation::Raw,
    };

    assert_eq!(String::from("1000 B"), format!("{size}"));
}
