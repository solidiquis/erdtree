use crate::hash;
use ansi_term::Color;
use error::Error;
use lscolors::LsColors;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

/// Errors for this module.
pub mod error;

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
static LS_COLORS: OnceCell<LsColors> = OnceCell::new();

/// Runtime evaluated static that contains ANSI-colored box drawing characters used for the
/// printing of [super::tree::Tree]'s branches.
static TREE_THEME: OnceCell<ThemesMap> = OnceCell::new();

/// Runtime evaluated static that contains ANSI-colored box drawing characters used for the
/// printing of [super::tree::Tree]'s branches for descendents of symlinks.
static LINK_THEME: OnceCell<ThemesMap> = OnceCell::new();

/// Runtime evaluated static that contains styles for disk usage output.
static DU_THEME: OnceCell<HashMap<&'static str, Color>> = OnceCell::new();

/// Map of the names box-drawing elements to their styled strings.
pub type ThemesMap = HashMap<&'static str, String>;

/// Initializes both [LS_COLORS] and all themes. If `plain` argument is `true` then plain colorless
/// themes are used and [LS_COLORS] won't be initialized.
pub fn init(plain: bool) {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    if plain {
        init_plain();
    } else {
        init_ls_colors();
        init_themes();
    }
}

/// Getter for [LS_COLORS]. Returns an error if not initialized.
pub fn get_ls_colors() -> Result<&'static LsColors, Error<'static>> {
    LS_COLORS.get().ok_or(Error::Uninitialized("LS_COLORS"))
}

/// Getter for [DU_THEME]. Returns an error if not initialized.
pub fn get_du_theme() -> Result<&'static HashMap<&'static str, Color>, Error<'static>> {
    DU_THEME.get().ok_or(Error::Uninitialized("DU_THEME"))
}

/// Getter for [TREE_THEME]. Returns an error if not initialized.
pub fn get_tree_theme() -> Result<&'static ThemesMap, Error<'static>> {
    TREE_THEME.get().ok_or(Error::Uninitialized("TREE_THEME"))
}

/// Getter for [LINK_THEME]. Returns an error if not initialized.
pub fn get_link_theme() -> Result<&'static ThemesMap, Error<'static>> {
    LINK_THEME.get().ok_or(Error::Uninitialized("LINK_THEME"))
}

/// Initializes [LS_COLORS] by reading in the `LS_COLORS` environment variable. If it isn't set, a
/// default determined by `lscolors` crate will be used.
fn init_ls_colors() {
    LS_COLORS
        .set(LsColors::from_env().unwrap_or_default())
        .unwrap();
}

/// Colorless themes
fn init_plain() {
    let theme = hash! {
        "vt" => VT.to_owned(),
        "uprt" => UPRT.to_owned(),
        "vtrt" => VTRT.to_owned()
    };

    TREE_THEME.set(theme).unwrap();

    let link_theme = hash! {
        "vt" => VT.to_owned(),
        "uprt" => UPRT.to_owned(),
        "vtrt" => VTRT.to_owned()
    };

    LINK_THEME.set(link_theme).unwrap();
}

/// Initializes all color themes.
fn init_themes() {
    let theme = hash! {
        "vt" => format!("{}", Color::Purple.paint(VT)),
        "uprt" => format!("{}", Color::Purple.paint(UPRT)),
        "vtrt" => format!("{}", Color::Purple.paint(VTRT))
    };

    TREE_THEME.set(theme).unwrap();

    let link_theme = hash! {
        "vt" => format!("{}", Color::Red.paint(VT)),
        "uprt" => format!("{}", Color::Red.paint(UPRT)),
        "vtrt" => format!("{}", Color::Red.paint(VTRT))
    };

    LINK_THEME.set(link_theme).unwrap();

    let du_theme = hash! {
        "B" => Color::Cyan,
        "KB" | "KiB" => Color::Yellow,
        "MB" | "MiB" => Color::Green,
        "GB" | "GiB" => Color::Red,
        "TB" | "TiB" => Color::Blue
    };

    DU_THEME.set(du_theme).unwrap();
}
