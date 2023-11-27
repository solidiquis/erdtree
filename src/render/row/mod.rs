use crate::{
    file::File,
    user::{column, Context},
};
use std::fmt::{self, Write};

/// Concerned with how to present long-format for a particular file.
#[cfg(unix)]
mod long;

pub type RowFormatter<'a> = Box<dyn FnMut(&File, String) -> fmt::Result + 'a>;

#[cfg(windows)]
pub fn formatter<'a>(buf: &'a mut String, ctx: &'a Context) -> RowFormatter<'a> {
    Box::new(|file, prefix| {
        let size = format!("{}", file.size());
        let name = file.display_name();
        let column::Metadata { max_size_width, .. } = ctx.column_metadata;
        writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
    })
}

#[cfg(unix)]
pub fn formatter<'a>(buf: &'a mut String, ctx: &'a Context) -> RowFormatter<'a> {
    if !ctx.long {
        return Box::new(|file, prefix| {
            let size = format!("{}", file.size());
            let name = file.display_name();
            let column::Metadata { max_size_width, .. } = ctx.column_metadata;
            writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
        });
    }

    Box::new(|file: &File, prefix| {
        let size = format!("{}", file.size());
        let name = file.display_name();
        let col_widths = ctx.column_metadata;
        let column::Metadata { max_size_width, .. } = col_widths;
        let long_format = long::Format::new(file, ctx);

        writeln!(buf, "{long_format} {size:>max_size_width$} {prefix}{name}")
    })
}
