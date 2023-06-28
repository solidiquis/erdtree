use ansi_term::{ANSIGenericString, Style};
use ignore::DirEntry;
use std::{borrow::Cow, path::Path};

/// Computes a plain, colorless icon with given parameters.
///
/// The precedent from highest to lowest in terms of which parameters determine the icon used
/// is as followed: file-type, file-extension, and then file-name. If an icon cannot be
/// computed the fall-back default icon is used.
///
/// If a directory entry is a link and the link target is provided, the link target will be
/// used to determine the icon.
pub fn compute(entry: &DirEntry, link_target: Option<&Path>) -> Cow<'static, str> {
    let icon = entry
        .file_type()
        .and_then(super::icon_from_file_type)
        .map(Cow::from);

    if let Some(i) = icon {
        return i;
    }

    let ext = match link_target {
        Some(target) if entry.path_is_symlink() => target.extension(),
        _ => entry.path().extension(),
    };

    let icon = ext
        .and_then(super::icon_from_ext)
        .map(|(_, i)| Cow::from(i));

    if let Some(i) = icon {
        return i;
    }

    let icon = super::icon_from_file_name(entry.file_name()).map(Cow::from);

    if let Some(i) = icon {
        return i;
    }

    Cow::from(super::get_default_icon().1)
}

/// Computes a plain, colored icon with given parameters. See [compute] for more details.
pub fn compute_with_color(
    entry: &DirEntry,
    link_target: Option<&Path>,
    style: Option<Style>,
) -> Cow<'static, str> {
    let icon = entry
        .file_type()
        .and_then(super::icon_from_file_type)
        .map(Cow::from);

    let paint_icon = |icon| match style {
        Some(Style {
            foreground: Some(fg),
            ..
        }) => {
            let ansi_string: ANSIGenericString<str> = fg.bold().paint(icon);
            let styled_icon = ansi_string.to_string();
            Cow::from(styled_icon)
        },
        _ => icon,
    };

    if let Some(icon) = icon {
        return paint_icon(icon);
    }

    let ext = match link_target {
        Some(target) if entry.path_is_symlink() => target.extension(),
        _ => entry.path().extension(),
    };

    let icon = ext
        .and_then(super::icon_from_ext)
        .map(|attrs| Cow::from(super::col(attrs.0, attrs.1)));

    if let Some(i) = icon {
        return i;
    }

    let icon = super::icon_from_file_name(entry.file_name())
        .map(Cow::from)
        .map(paint_icon);

    if let Some(i) = icon {
        return i;
    }

    let (code, icon) = super::get_default_icon();
    Cow::from(super::col(code, icon))
}
