use crate::{
    disk::{BIN_PADDING, FLOAT_PRECISION, RAW_PADDING, SI_PADDING},
    file::File,
    user::{
        args::{BytePresentation, Metric},
        Context,
    },
};

#[cfg(unix)]
use crate::file::inode::Inode;

#[derive(Clone, Copy, Debug, Default)]
pub struct Metadata {
    pub max_size_width: usize,
    #[cfg(unix)]
    pub max_group_width: usize,
    #[cfg(unix)]
    pub max_owner_width: usize,
    #[cfg(unix)]
    pub max_nlink_width: usize,
    #[cfg(unix)]
    pub max_ino_width: usize,
    #[cfg(unix)]
    pub max_time_width: usize,
}

impl Metadata {
    pub fn update_size_width(&mut self, file: &File, ctx: &Context) {
        self.max_size_width = Self::get_size_width(file, ctx).max(self.max_size_width);
    }

    #[cfg(unix)]
    pub fn update_unix_attrs_widths(&mut self, file: &File, ctx: &Context) {
        let unix_attrs = file.unix_attrs();

        self.max_time_width = file
            .timestamp_from_ctx(ctx)
            .map_or(0, |s| s.len())
            .max(self.max_time_width);
        self.max_owner_width = unix_attrs
            .owner()
            .map_or(0, str::len)
            .max(self.max_owner_width);
        self.max_group_width = unix_attrs
            .group()
            .map_or(0, str::len)
            .max(self.max_group_width);
    }

    #[cfg(unix)]
    pub fn update_inode_attr_widths(&mut self, inode: &Inode) {
        self.max_ino_width = utils::num_integral(inode.ino).max(self.max_ino_width);
        self.max_nlink_width = utils::num_integral(inode.nlink).max(self.max_nlink_width);
    }

    fn get_size_width(file: &File, ctx: &Context) -> usize {
        if !matches!(ctx.metric, Metric::Logical | Metric::Physical) {
            return utils::num_integral(file.size().value());
        }

        match ctx.byte_units {
            BytePresentation::Raw => utils::num_integral(file.size().value()) + RAW_PADDING,
            // Explanation for 4:
            // - 123.1 KB, the '4' takes into account the integral numbers plus the '.'.
            BytePresentation::Si => 4 + FLOAT_PRECISION + SI_PADDING,
            // Explanation for 5:
            // Unlike SI units, we can have say 1008.0 KiB for binary units.
            BytePresentation::Bin => 5 + FLOAT_PRECISION + BIN_PADDING,
        }
    }
}

mod utils {
    /// How many integral digits are there?
    #[inline]
    pub fn num_integral(value: u64) -> usize {
        if value == 0 {
            return 0;
        }
        value.ilog10() as usize + 1
    }

    #[test]
    fn test_num_integral() {
        assert_eq!(num_integral(1000), 4);
        assert_eq!(num_integral(10), 2);
        assert_eq!(num_integral(10000), 5);
    }
}
