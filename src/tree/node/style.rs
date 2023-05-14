use super::Node;
use ansi_term::{Color, Style};
use std::{borrow::Cow, ffi::OsStr};

#[cfg(unix)]
use crate::{
    fs::permissions::FileMode,
    styles::{get_octal_permissions_style, get_permissions_theme},
};

impl Node {
    /// Stylizes input, `entity` based on `LS_COLORS`. If `style` is `None` then the entity is
    /// returned unmodified.
    pub fn stylize(file_name: &OsStr, style: Option<Style>) -> Cow<'_, str> {
        let name = file_name.to_string_lossy();

        if let Some(Style {
            foreground: Some(ref fg),
            ..
        }) = style
        {
            Cow::from(fg.bold().paint(name).to_string())
        } else {
            name
        }
    }

    /// Stylizes symlink name for display.
    pub fn stylize_link_name<'a>(
        link_name: &'a OsStr,
        target_name: &'a OsStr,
        style: Option<Style>,
    ) -> Cow<'a, str> {
        if style.is_some() {
            let styled_name = Self::stylize(link_name, style);
            let target_name =
                Color::Red.paint(format!("\u{2192} {}", target_name.to_string_lossy()));

            Cow::from(format!("{styled_name} {target_name}"))
        } else {
            let link = link_name.to_string_lossy();
            let target = target_name.to_string_lossy();
            Cow::from(format!("{link} \u{2192} {target}"))
        }
    }

    /// Styles the symbolic notation file permissions.
    #[cfg(unix)]
    pub fn style_sym_permissions(mode: &FileMode, has_xattrs: bool) -> String {
        let sym = if has_xattrs {
            format!("{mode}@")
        } else {
            format!("{mode} ")
        };

        if let Ok(theme) = get_permissions_theme() {
            sym.chars()
                .filter_map(|ch| {
                    theme.get(&ch).map(|color| {
                        let chstr = ch.to_string();
                        color.paint(chstr).to_string()
                    })
                })
                .collect::<String>()
        } else {
            sym
        }
    }

    /// Styles the numeric octal format of permissions.
    #[cfg(unix)]
    pub fn style_octal_permissions(mode: &FileMode) -> String {
        let oct = format!("{mode:04o}");
        if let Ok(style) = get_octal_permissions_style() {
            style.paint(oct).to_string()
        } else {
            oct
        }
    }
}
