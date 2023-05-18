use std::fmt;
use units::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};

/// Binary and SI prefixes
pub mod units;

/// Rules to display disk usage for individual files.
pub mod file_size;

pub mod block;

pub mod logical;

pub mod physical;

pub trait DuMetric {}

pub trait ByteDisplay {
    fn prefix_kind(&self) -> PrefixKind;

    fn value(&self) -> u64;

    fn human_readable(&self) -> bool;

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value() as f64;

        match self.prefix_kind() {
            PrefixKind::Si => {
                let unit = SiPrefix::from(value);
                let base_value = unit.base_value();

                if !self.human_readable() || matches!(unit, SiPrefix::Base) {
                    write!(f, "{} {unit}", self.value())
                } else {
                    let size = value / (base_value as f64);
                    write!(f, "{size:.2} {unit}")
                }
            }
            PrefixKind::Bin => {
                let unit = BinPrefix::from(value);
                let base_value = unit.base_value();

                if !self.human_readable() || matches!(unit, BinPrefix::Base) {
                    write!(f, "{} {unit}", self.value())
                } else {
                    let size = value / (base_value as f64);
                    write!(f, "{size:.2} {unit}")
                }
            }
        }
    }
}
