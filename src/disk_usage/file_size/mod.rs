use crate::context::Context;
use clap::ValueEnum;
use std::{convert::From, ops::AddAssign};

#[cfg(unix)]
pub mod block;

pub mod byte;
//pub mod word_count;
pub mod line_count;

/// Represents all the different ways in which a filesize could be reported using various metrics.
pub enum FileSize {
    //WordCount(word_count::Metric),
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

    /// How many blocks are allocated to store the file
    #[cfg(unix)]
    Block,
}

impl FileSize {
    #[inline]
    pub const fn value(&self) -> u64 {
        match self {
            Self::Byte(metric) => metric.value,
            Self::Line(metric) => metric.value,

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

            #[cfg(unix)]
            Self::Block(metric) => metric.value += rhs.value(),
        }
    }
}

impl From<&Context> for FileSize {
    fn from(ctx: &Context) -> Self {
        use DiskUsage::*;

        match ctx.disk_usage {
            Logical => Self::Byte(byte::Metric::init_empty_logical(ctx.human, ctx.unit)),
            Physical => Self::Byte(byte::Metric::init_empty_physical(ctx.human, ctx.unit)),
            Line => Self::Line(line_count::Metric::default()),

            #[cfg(unix)]
            Block => Self::Block(block::Metric::default()),
        }
    }
}
