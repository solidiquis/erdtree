use std::{
    fmt::{self, Display},
    fs::Metadata,
    os::unix::fs::MetadataExt,
};

#[derive(Default)]
pub struct Metric {
    pub value: u64,
}

impl Metric {
    pub fn init(md: &Metadata) -> Self {
        Self { value: md.blocks() }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u64 as Display>::fmt(&self.value, f)
    }
}
