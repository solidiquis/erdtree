use crate::hash;
use ansi_term::Color;
use lscolors::LsColors;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

/// Used for padding between tree branches.
pub const SEP: &str = "   ";

/// The `│` box drawing character.
pub const VT: &str = "\u{2502}  ";

/// The `└─` box drawing characters.
pub const UPRT: &str = "\u{2514}\u{2500} ";

/// The `├─` box drawing characters.
pub const VTRT: &str = "\u{251C}\u{2500} ";

/// A runtime evaluated static. [LS_COLORS] the `LS_COLORS` environment variable to determine what
/// ANSI colors to use when printing the names of files. If `LS_COLORS` is not set it will fallback
/// to a default defined in the `lscolors` crate.
///
/// **Note for MacOS**: MacOS uses the `LSCOLORS` environment variable which is in a format not
/// supported by the `lscolors` crate. Mac users can either set their own `LS_COLORS` environment
/// variable to customize output color or rely on the default.
pub static LS_COLORS: OnceCell<LsColors> = OnceCell::new();

/// Runtime evaluated static that contains ANSI-colored box drawing characters used for the
/// printing of [super::Tree]'s branches.
pub static THEME: OnceCell<ThemesMap> = OnceCell::new();

/// Runtime evaluated static that contains ANSI-colored box drawing characters used for the
/// printing of [super::Tree]'s branches for descdendents of symlinks.
pub static LINK_THEME: OnceCell<ThemesMap> = OnceCell::new();

/// Runtime evaluated static that contains styles for disk usage output.
pub static DU_THEME: OnceCell<HashMap<&'static str, Color>> = OnceCell::new();

/// Map of the names box-drawing elements to their styled strings.
pub type ThemesMap = HashMap<&'static str, String>;

/// Initializes both [LS_COLORS] and [THEME].
pub fn init() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support();

    init_ls_colors();
    init_themes();
}

/// Getter for [LS_COLORS]. Panics if not initialized.
pub fn get_ls_colors() -> &'static LsColors {
    LS_COLORS.get().expect("LS_COLORS not initialized")
}

/// Getter for [DU_THEME]. Panics if not initialized.
pub fn get_du_theme() -> &'static HashMap<&'static str, Color> {
    DU_THEME.get().expect("DU_THEME not initialized")
}

/// Getter for [THEME]. Panics if not initialized.
pub fn get_theme() -> &'static ThemesMap {
    THEME.get().expect("THEME not initialized")
}

/// Getter for [LINK_THEME]. Panics if not initialized.
pub fn get_link_theme() -> &'static ThemesMap {
    LINK_THEME.get().expect("THEME not initialized")
}

/// Initializes [LS_COLORS] by reading in the `LS_COLORS` environment variable. If it isn't set, a
/// default determined by `lscolors` crate will be used.
fn init_ls_colors() {
    LS_COLORS
        .set(LsColors::from_env().unwrap_or_default())
        .unwrap();
}

/// Initializes [THEME].
fn init_themes() {
    let theme = hash! {
        "vt" => format!("{}", Color::Purple.paint(VT)),
        "uprt" => format!("{}", Color::Purple.paint(UPRT)),
        "vtrt" => format!("{}", Color::Purple.paint(VTRT))
    };

    THEME.set(theme).unwrap();

    let link_theme = hash! {
        "vt" => format!("{}", Color::Red.paint(VT)),
        "uprt" => format!("{}", Color::Red.paint(UPRT)),
        "vtrt" => format!("{}", Color::Red.paint(VTRT))
    };

    LINK_THEME.set(link_theme).unwrap();

    let du_theme = hash! {
        "B" => Color::Cyan,
        "KB" => Color::Yellow,
        "KiB" => Color::Yellow,
        "MB" => Color::Green,
        "MiB" => Color::Green,
        "GB" => Color::Red,
        "MiB" => Color::Red,
        "TB" => Color::Blue,
        "TiB" => Color::Blue
    };

    DU_THEME.set(du_theme).unwrap();
}
