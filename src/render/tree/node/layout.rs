use crate::render::{context::Context, disk_usage::file_size::FileSize};

/// Simple struct to define location to put the `FileSize` while printing a `Node`
#[derive(Copy, Clone, Default)]
pub enum SizeLocation {
    #[default]
    Right,
    Left,
}

impl SizeLocation {
    /// Returns a string to use when a node has no filesize, such as empty directories
    pub fn default_string(self, ctx: &Context) -> String {
        match self {
            Self::Right => String::new(),
            Self::Left => FileSize::empty_string(ctx),
        }
    }

    /// Given a [`FileSize`], style it in the expected way for its printing location
    pub fn format(self, size: &FileSize) -> String {
        match self {
            Self::Right => format!("({})", size.format(false)),
            Self::Left => size.format(true),
        }
    }
}

impl From<&Context> for SizeLocation {
    fn from(ctx: &Context) -> Self {
        if ctx.size_left && !ctx.suppress_size {
            Self::Left
        } else {
            Self::Right
        }
    }
}
