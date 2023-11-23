use crate::{
    file::File,
    user::{args::Metric, Context},
};

#[derive(Default)]
pub struct Widths {
    size: usize,
    name: usize,
    attrs: usize,
    group: usize,
    nlink: usize,
    octal: usize,
    time: usize,
}

impl Widths {
    pub fn update(&mut self, file: &File, ctx: &Context) {
        self.size = Self::get_size_width(file, ctx).max(self.size);
        self.name = Self::get_name_width(file).max(self.name);
    }

    fn get_name_width(file: &File) -> usize {
        file.file_name().to_string_lossy().len()
    }

    fn get_size_width(file: &File, ctx: &Context) -> usize {
        if !matches!(ctx.metric, Metric::Logical | Metric::Physical) {
            return utils::num_integral(file.size().value());
        }

        format!("{}", file.size()).len()
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
