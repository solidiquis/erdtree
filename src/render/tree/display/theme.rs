use crate::render::{
    styles::{self, ThemesMap},
    tree::Node,
};

/// Returns a closure that retrieves the regular theme.
pub fn regular_theme_getter() -> Box<dyn FnMut(&Node) -> &'static ThemesMap> {
    Box::new(|_node: &Node| styles::get_tree_theme().unwrap())
}

/// Returns a closure that can smartly determine when a symlink is being followed and when it is
/// not being followed. When a symlink is being followed, all of its descendents should have tree
/// branches that are colored differently.
pub fn link_theme_getter() -> Box<dyn FnMut(&Node) -> &'static ThemesMap> {
    let mut link_depth = None;

    Box::new(move |node: &Node| {
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
