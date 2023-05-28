use crate::{
    styles::{self, ThemesMap},
    tree::node::Node,
};
use ansi_term::{Color, Style};
use std::borrow::Cow;

type Theme = Box<dyn FnMut(&Node) -> &'static ThemesMap>;

/// Returns a closure that retrieves the regular theme.
pub fn regular_theme_getter() -> Theme {
    Box::new(|_node| styles::get_tree_theme().unwrap())
}

/// Returns a closure that can smartly determine when a symlink is being followed and when it is
/// not being followed. When a symlink is being followed, all of its descendents should have tree
/// branches that are colored differently.
pub fn link_theme_getter() -> Theme {
    let mut link_depth = None;

    Box::new(move |node| {
        let current_depth = node.depth();

        if let Some(ldepth) = link_depth {
            if current_depth == ldepth {
                link_depth = None;
            }
        }

        if link_depth.is_some() || node.is_symlink() {
            if node.is_dir() && link_depth.is_none() {
                link_depth = Some(current_depth);
            }
            styles::get_link_theme().unwrap()
        } else {
            styles::get_tree_theme().unwrap()
        }
    })
}

/// Stylizes the input `file_name` with the provided `style`. If `None` is provided then the
/// underlying `String` is returned unmodified as a [Cow]. If the provided [Node] is a symlink then
/// it will be styled accordingly.
pub fn stylize_file_name(node: &Node) -> Cow<'_, str> {
    let name = node.file_name();
    let style = node.style();

    let Some(target_name) = node.symlink_target_file_name() else {
        if let Some(Style {foreground: Some(ref fg), .. }) = style {
            let file_name = name.to_string_lossy();
            let styled_name = fg.bold().paint(file_name).to_string();
            return Cow::from(styled_name);
        }

        return name.to_string_lossy();
    };

    if let Some(color) = style {
        let styled_name = color.paint(name.to_string_lossy());
        let target_name = Color::Red.paint(format!("\u{2192} {}", target_name.to_string_lossy()));

        return Cow::from(format!("{styled_name} {target_name}"));
    }

    let link = name.to_string_lossy();
    let target = target_name.to_string_lossy();
    Cow::from(format!("{link} \u{2192} {target}"))
}

/// Styles the symbolic notation of file permissions.
#[cfg(unix)]
pub fn style_sym_permissions(node: &Node) -> String {
    use crate::fs::permissions::FileModeXAttrs;

    let perms = node.mode().expect("Expected permissions to be initialized");

    let symb = if node.has_xattrs() {
        let perm_xattr = FileModeXAttrs(&perms);
        format!("{perm_xattr}")
    } else {
        // extra whitespace to align with permissions with extended attrs
        format!("{perms} ")
    };

    if let Ok(theme) = styles::get_permissions_theme() {
        symb.chars()
            .filter_map(|ch| {
                theme.get(&ch).map(|color| {
                    let chstr = ch.to_string();
                    color.paint(chstr).to_string()
                })
            })
            .collect()
    } else {
        symb
    }
}

/// Styles the octal notation of file permissions.
#[cfg(unix)]
pub fn style_oct_permissions(node: &Node) -> String {
    let perms = node.mode().expect("Expected permissions to be initialized");
    let oct = format!("{perms:04o}");

    if let Ok(style) = styles::get_octal_permissions_style() {
        style.paint(oct).to_string()
    } else {
        oct
    }
}
