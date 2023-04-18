use super::Node;
use ansi_term::{Color, Style};
use std::{borrow::Cow, ffi::OsStr};

#[cfg(unix)]
use crate::fs::permissions::FileMode;

#[cfg(unix)]
use crate::render::styles::{get_octal_permissions_style, get_permissions_theme};

impl Node {
    /// Stylizes input, `entity` based on `LS_COLORS`. If `style` is `None` then the entity is
    /// returned unmodified.
    pub(super) fn stylize(file_name: &OsStr, style: Option<Style>) -> Cow<'_, str> {
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
    pub(super) fn stylize_link_name<'a>(
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
    pub(super) fn style_sym_permissions(perm_str: &str) -> String {
        let theme = get_permissions_theme().unwrap();

        perm_str
            .chars()
            .filter_map(|ch| {
                theme.get(&ch).map(|color| {
                    let chstr = ch.to_string();
                    color.paint(chstr).to_string()
                })
            })
            .collect::<String>()
    }

    #[cfg(unix)]
    pub(super) fn style_octal_permissions(mode: &FileMode) -> String {
        get_octal_permissions_style()
            .unwrap()
            .paint(format!("{mode:04o}"))
            .to_string()
    }
}
