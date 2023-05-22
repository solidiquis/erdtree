use super::super::units::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};
use crate::context::Context;
use filesize::PathExt;
use std::{
    convert::From,
    fmt::{self, Display},
    fs::Metadata,
    path::Path,
};

pub struct Metric {
    pub value: u64,
    pub human_readable: bool,
    #[allow(dead_code)]
    kind: MetricKind,
    prefix_kind: PrefixKind,
}

pub enum MetricKind {
    Logical,
    Physical,
}

impl Metric {
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

impl From<&Context> for Metric {
    fn from(ctx: &Context) -> Self {
        let metric_kind = match ctx.disk_usage {
            super::DiskUsage::Logical => MetricKind::Logical,
            super::DiskUsage::Physical => MetricKind::Physical,
        };

        Self {
            value: 0,
            prefix_kind: ctx.unit,
            human_readable: ctx.human,
            kind: metric_kind,
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
                    write!(f, "{size:.2} {unit}")
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
                    write!(f, "{size:.2} {unit}")
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
    assert_eq!(format!("{}", metric), "1.00 KB");

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
    assert_eq!(format!("{}", metric), "1.00 KiB");

    let metric = Metric {
        value: 2_u64.pow(20),
        kind: MetricKind::Logical,
        human_readable: true,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "1.00 MiB");

    let metric = Metric {
        value: 123454,
        kind: MetricKind::Logical,
        human_readable: false,
        prefix_kind: PrefixKind::Bin,
    };
    assert_eq!(format!("{}", metric), "123454 B");
}
