use crate::{file::File, user::{Context, column}};
use std::fmt::{self, Write};

/// Concerned with how to present long-format for a particular file.
#[cfg(unix)]
mod long;

#[cfg(windows)]
pub fn formatter<'a>(
    buf: &'a mut String,
    ctx: &'a Context,
) -> Box<dyn FnMut(&File, String) -> fmt::Result + 'a> {
    Box::new(|file, prefix| {
        let size = format!("{}", file.size());
        let name = file.file_name().to_string_lossy();
        let column::Widths {
            size: size_width, ..
        } = ctx.col_widths();
        writeln!(buf, "{size:>size_width$} {prefix}{name}")
    })
}

#[cfg(unix)]
pub fn formatter<'a>(
    buf: &'a mut String,
    ctx: &'a Context,
) -> Box<dyn FnMut(&File, String) -> fmt::Result + 'a> {
    if !ctx.long {
        return Box::new(|file, prefix| {
            let size = format!("{}", file.size());
            let name = file.file_name().to_string_lossy();
            let column::Metadata {
                max_size_width, ..
            } = ctx.column_metadata;
            writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
        });
    }

    Box::new(|file: &File, prefix| {
        let size = format!("{}", file.size());
        let name = file.file_name().to_string_lossy();
        let col_widths = ctx.column_metadata;
        let column::Metadata { max_size_width, .. } = col_widths;
        let long_format = long::Format::new(file, ctx);

        writeln!(buf, "{long_format} {size:>max_size_width$} {prefix}{name}")
    })
}
