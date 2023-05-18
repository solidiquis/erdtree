use super::{
    ByteDisplay,
    DuMetric,
    units::PrefixKind,
};
use std::{
    fmt::{self, Display},
    fs::Metadata,
};

pub struct Size {
    pub value: u64,
    pub prefix_kind: PrefixKind,
    pub human_readable: bool,
}

impl DuMetric for Size {}

impl Size {
    pub fn new(metadata: &Metadata, prefix_kind: PrefixKind, human_readable: bool) -> Self {
        let value = metadata.len();

        Self {
            value,
            prefix_kind,
            human_readable,
        }
    }
}

impl ByteDisplay for Size {
    fn human_readable(&self) -> bool {
        self.human_readable
    }

    fn prefix_kind(&self) -> PrefixKind {
        self.prefix_kind
    }

    fn value(&self) -> u64 {
        self.value
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as ByteDisplay>::fmt(self, f)
    }
}

#[test]
fn test_logical_size() -> std::io::Result<()> {
    use std::fs;

    let md = fs::metadata("./tests/data/nemesis.txt")?;

    let logical_size = Size::new(&md, PrefixKind::Bin, false);

    let display = format!("{logical_size}");

    assert_eq!(logical_size.value, 161);
    assert_eq!(display, "161 B");

    assert_eq!(
        format!(
            "{}",
            Size {
                value: 1_024,
                prefix_kind: PrefixKind::Bin,
                human_readable: true
            }
        ),
        "1.00 KiB"
    );

    Ok(())
}
