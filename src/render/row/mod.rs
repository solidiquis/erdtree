use crate::{
    error::prelude::*,
    file::File,
    user::{args::Layout, column, Context},
};
use std::fmt::{self, Write};

/// Concerned with how to present long-format for a particular file.
#[cfg(unix)]
mod long;

pub type RowFormatter<'a> = Box<dyn FnMut(&File, String) -> fmt::Result + 'a>;

#[cfg(unix)]
pub fn formatter<'a>(buf: &'a mut String, ctx: &'a Context) -> Result<RowFormatter<'a>> {
    match ctx.layout {
        Layout::Flat => {
            let root = ctx.dir_canonical()?;

            match (ctx.long, ctx.suppress_size) {
                (false, false) => Ok(Box::new(move |file, prefix| {
                    let size = format!("{}", file.size());
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    let column::Metadata { max_size_width, .. } = ctx.column_metadata;

                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{size:>max_size_width$} {prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
                    }
                })),

                (false, true) => Ok(Box::new(move |file, prefix| {
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{prefix}{name}")
                    }
                })),

                (true, false) => Ok(Box::new(move |file, prefix| {
                    let size = format!("{}", file.size());
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    let col_widths = ctx.column_metadata;
                    let column::Metadata { max_size_width, .. } = col_widths;
                    let long_format = long::Format::new(file, ctx);
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(
                            buf,
                            "{long_format} {size:>max_size_width$} {prefix}{icon:<2}{name}"
                        )
                    } else {
                        writeln!(buf, "{long_format} {size:>max_size_width$} {prefix}{name}")
                    }
                })),

                (true, true) => Ok(Box::new(move |file, prefix| {
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    let long_format = long::Format::new(file, ctx);

                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{long_format} {prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{long_format} {prefix}{name}")
                    }
                })),
            }
        },
        _ => match (ctx.long, ctx.suppress_size) {
            (false, false) => Ok(Box::new(|file, prefix| {
                let size = format!("{}", file.size());
                let name = file.display_name();
                let column::Metadata { max_size_width, .. } = ctx.column_metadata;
                if ctx.icons {
                    let icon = file.icon();
                    writeln!(buf, "{size:>max_size_width$} {prefix}{icon:<2}{name}")
                } else {
                    writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
                }
            })),

            (false, true) => Ok(Box::new(|file, prefix| {
                let name = file.display_name();
                if ctx.icons {
                    let icon = file.icon();
                    writeln!(buf, "{prefix}{icon:<2}{name}")
                } else {
                    writeln!(buf, "{prefix}{name}")
                }
            })),

            (true, false) => Ok(Box::new(|file, prefix| {
                let size = format!("{}", file.size());
                let name = file.display_name();
                let col_widths = ctx.column_metadata;
                let column::Metadata { max_size_width, .. } = col_widths;
                let long_format = long::Format::new(file, ctx);
                if ctx.icons {
                    let icon = file.icon();
                    writeln!(
                        buf,
                        "{long_format} {size:>max_size_width$} {prefix}{icon:<2}{name}"
                    )
                } else {
                    writeln!(buf, "{long_format} {size:>max_size_width$} {prefix}{name}")
                }
            })),

            (true, true) => Ok(Box::new(|file, prefix| {
                let name = file.display_name();
                let long_format = long::Format::new(file, ctx);
                if ctx.icons {
                    let icon = file.icon();
                    writeln!(buf, "{long_format} {prefix}{icon:<2}{name}")
                } else {
                    writeln!(buf, "{long_format} {prefix}{name}")
                }
            })),
        },
    }
}

#[cfg(windows)]
pub fn formatter<'a>(buf: &'a mut String, ctx: &'a Context) -> Result<RowFormatter<'a>> {
    match ctx.layout {
        Layout::Flat => {
            let root = ctx.dir_canonical()?;

            if ctx.suppress_size {
                Ok(Box::new(move |file, prefix| {
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{prefix}{name}")
                    }
                }))
            } else {
                Ok(Box::new(move |file, prefix| {
                    let size = format!("{}", file.size());
                    let base = root.ancestors().nth(1);
                    let name = file.display_path(base);
                    let column::Metadata { max_size_width, .. } = ctx.column_metadata;
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{size:>max_size_width$} {prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
                    }
                }))
            }
        },
        _ => {
            if ctx.suppress_size {
                Ok(Box::new(|file, prefix| {
                    let name = file.display_name();
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{prefix}{name}")
                    }
                }))
            } else {
                Ok(Box::new(|file, prefix| {
                    let size = format!("{}", file.size());
                    let name = file.display_name();
                    let column::Metadata { max_size_width, .. } = ctx.column_metadata;
                    if ctx.icons {
                        let icon = file.icon();
                        writeln!(buf, "{size:>max_size_width$} {prefix}{icon:<2}{name}")
                    } else {
                        writeln!(buf, "{size:>max_size_width$} {prefix}{name}")
                    }
                }))
            }
        },
    }
}
