use super::super::units::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};
use filesize::PathExt;
use std::{
    fmt::{self, Display},
    fs::Metadata,
    path::Path,
};

/// Concerned with measuring file size in bytes, whether logical or physical determined by `kind`.
/// Binary or SI units used for reporting determined by `prefix_kind`.
pub struct Metric {
    pub value: u64,
    pub human_readable: bool,
    #[allow(dead_code)]
    kind: MetricKind,
    prefix_kind: PrefixKind,
}

/// Represents the appropriate method in which to compute bytes. `Logical` represent the total amount
/// of bytes in a file; `Physical` represents how many bytes are actually used to store the file on
/// disk.
pub enum MetricKind {
    Logical,
    Physical,
}

impl Metric {
    /// Initializes a [Metric] that stores the total amount of bytes in a file.
    pub fn init_logical(
        metadata: &Metadata,
        prefix_kind: PrefixKind,
        human_readable: bool,
    ) -> Self {
        let value = metadata.len();
        let kind = MetricKind::Logical;

        Self {
            value,
            human_readable,
            kind,
            prefix_kind,
        }
    }

    /// Initializes an empty [Metric] used to represent the total amount of bytes of a file.
    pub const fn init_empty_logical(human_readable: bool, prefix_kind: PrefixKind) -> Self {
        Self {
            value: 0,
            human_readable,
            kind: MetricKind::Logical,
            prefix_kind,
        }
    }

    /// Initializes an empty [Metric] used to represent the total disk space of a file in bytes.
    pub const fn init_empty_physical(human_readable: bool, prefix_kind: PrefixKind) -> Self {
        Self {
            value: 0,
            human_readable,
            kind: MetricKind::Physical,
            prefix_kind,
        }
    }

    /// Initializes a [Metric] that stores the total amount of bytes used to store a file on disk.
    pub fn init_physical(
        path: &Path,
        metadata: &Metadata,
        prefix_kind: PrefixKind,
        human_readable: bool,
    ) -> Self {
        let value = path.size_on_disk_fast(metadata).unwrap_or(metadata.len());
        let kind = MetricKind::Physical;

        Self {
            value,
            human_readable,
            kind,
            prefix_kind,
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value as f64;

        match self.prefix_kind {
            PrefixKind::Si => {
                if !self.human_readable {
                    return write!(f, "{} {}", self.value, SiPrefix::Base);
                }

                let unit = SiPrefix::from(self.value);

                if matches!(unit, SiPrefix::Base) {
                    write!(f, "{} {unit}", self.value)
                } else {
                    let base_value = unit.base_value();
                    let size = value / (base_value as f64);
                    write!(f, "{size:.1} {unit}")
                }
            }
            PrefixKind::Bin => {
                if !self.human_readable {
                    return write!(f, "{} {}", self.value, BinPrefix::Base);
                }

                let unit = BinPrefix::from(self.value);

                if matches!(unit, BinPrefix::Base) {
                    write!(f, "{} {unit}", self.value)
                } else {
                    let base_value = unit.base_value();
                    let size = value / (base_value as f64);
                    write!(f, "{size:.1} {unit}")
                }
            }
        }
    }
}

#[test]
fn test_metric() {
    let metric = Metric {
        value: 100,
        kind: MetricKind::Logical,
        human_readable: false,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "100 B");

    let metric = Metric {
        value: 1000,
        kind: MetricKind::Logical,
        human_readable: true,
        prefix_kind: PrefixKind::Si,
    };
    assert_eq!(format!("{}", metric), "1.0 KB");

    let metric = Metric {
        value: 1000,
        kind: MetricKind::Logical,
        human_readable: true,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "1000 B");

    let metric = Metric {
        value: 1024,
        kind: MetricKind::Logical,
        human_readable: true,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "1.0 KiB");

    let metric = Metric {
        value: 2_u64.pow(20),
        kind: MetricKind::Logical,
        human_readable: true,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "1.0 MiB");

    let metric = Metric {
        value: 123454,
        kind: MetricKind::Logical,
        human_readable: false,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "123454 B");
}
