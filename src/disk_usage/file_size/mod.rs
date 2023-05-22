use crate::context::Context;
use clap::ValueEnum;
use std::{convert::From, ops::AddAssign};

pub mod byte;
//pub mod block;
//pub mod word_count;
//pub mod line_count;

pub enum FileSize {
    //Block(block::Metric),
    //WordCount(word_count::Metric),
    //LineCount(line_count::Metric),
    Byte(byte::Metric),
}

/// Determines between logical or physical size for display
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    Logical,

    /// How much actual space on disk in bytes, taking into account sparse files and compression.
    #[default]
    Physical,
}

impl FileSize {
    #[inline]
    pub const fn value(&self) -> u64 {
        match self {
            Self::Byte(metric) => metric.value,
        }
    }
}

impl AddAssign<&Self> for FileSize {
    fn add_assign(&mut self, rhs: &Self) {
        match self {
            Self::Byte(metric) => metric.value += rhs.value(),
        }
    }
}

impl From<&Context> for FileSize {
    fn from(ctx: &Context) -> Self {
        match ctx.disk_usage {
            DiskUsage::Logical | DiskUsage::Physical => Self::Byte(byte::Metric::from(ctx)),
        }
    }
}
