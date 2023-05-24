use super::{Context, PrefixKind};
use std::convert::From;

/// Utility struct to help store maximum column widths for attributes of each node. Each width is
/// measured as the number of columns of the tty's window.
pub struct Properties {
    pub max_size_width: usize,
    pub max_size_unit_width: usize,

    #[cfg(unix)]
    pub max_nlink_width: usize,

    #[cfg(unix)]
    pub max_ino_width: usize,

    #[cfg(unix)]
    pub max_block_width: usize,

    #[cfg(unix)]
    pub max_owner_width: usize,

    #[cfg(unix)]
    pub max_group_width: usize,
}

impl From<&Context> for Properties {
    fn from(ctx: &Context) -> Self {
        let unit_width = match ctx.unit {
            PrefixKind::Bin if ctx.human => 3,
            PrefixKind::Si if ctx.human => 2,
            _ => 1,
        };

        Self {
            max_size_width: 0,
            max_size_unit_width: unit_width,
            #[cfg(unix)]
            max_nlink_width: 0,
            #[cfg(unix)]
            max_ino_width: 0,
            #[cfg(unix)]
            max_block_width: 0,
            #[cfg(unix)]
            max_owner_width: 0,
            #[cfg(unix)]
            max_group_width: 0,
        }
    }
}
