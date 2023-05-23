use crate::context::Context;
use clap::ValueEnum;
use std::{convert::From, ops::AddAssign};

/// Concerned with measuring file size in blocks.
#[cfg(unix)]
pub mod block;

/// Concerned with measuring file size in bytes, logical or physical.
pub mod byte;

/// Concerned with measuring file size by line count.
pub mod line_count;

/// Concerned with measuring file size by word count.
pub mod word_count;

/// Represents all the different ways in which a filesize could be reported using various metrics.
pub enum FileSize {
    Word(word_count::Metric),
    Line(line_count::Metric),
    Byte(byte::Metric),
    Block(block::Metric),
}

/// Determines between logical or physical size for display
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    Logical,

    /// How much actual space on disk in bytes, taking into account sparse files and compression.
    #[default]
    Physical,

    /// How many total lines a file contains
    Line,

    /// How many total words a file contains
    Word,

    /// How many blocks are allocated to store the file
    #[cfg(unix)]
    Block,
}

impl FileSize {
    /// Extracts the inner value of [`FileSize`] which represents the file size for various metrics.
    #[inline]
    pub const fn value(&self) -> u64 {
        match self {
            Self::Byte(metric) => metric.value,
            Self::Line(metric) => metric.value,
            Self::Word(metric) => metric.value,

            #[cfg(unix)]
            Self::Block(metric) => metric.value,
        }
    }
}

impl AddAssign<&Self> for FileSize {
    fn add_assign(&mut self, rhs: &Self) {
        match self {
            Self::Byte(metric) => metric.value += rhs.value(),
            Self::Line(metric) => metric.value += rhs.value(),
            Self::Word(metric) => metric.value += rhs.value(),

            #[cfg(unix)]
            Self::Block(metric) => metric.value += rhs.value(),
        }
    }
}

impl From<&Context> for FileSize {
    fn from(ctx: &Context) -> Self {
        use DiskUsage::{Line, Logical, Physical, Word};

        match ctx.disk_usage {
            Logical => Self::Byte(byte::Metric::init_empty_logical(ctx.human, ctx.unit)),
            Physical => Self::Byte(byte::Metric::init_empty_physical(ctx.human, ctx.unit)),
            Line => Self::Line(line_count::Metric::default()),
            Word => Self::Word(word_count::Metric::default()),

            #[cfg(unix)]
            DiskUsage::Block => Self::Block(block::Metric::default()),
        }
    }
}
