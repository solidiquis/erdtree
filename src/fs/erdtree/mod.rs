use lscolors::LsColors;
use once_cell::sync::OnceCell;

/// Contains components of the `Tree` data structure that derive from [ignore::DirEntry].
pub mod node;

/// Ordering operations for printing.
pub mod order;

/// Encapsulates everything related to the in-memory representation of the root directory and its
/// contents.
pub mod tree;

/// A runtime evaluated static. `LS_COLORS` the `LS_COLORS` environment variable to determine what
/// ANSI colors to use when printing the names of files. If `LS_COLORS` is not set it will fallback
/// to a default defined in the `lscolors` crate.
///
/// **Note for MacOS**: MacOS uses the `LSCOLORS` environment variable which is in a format not
/// supported by the `lscolors` crate. Mac users can either set their own `LS_COLORS` environment
/// variable to customize output color or rely on the default.
pub static LS_COLORS: OnceCell<LsColors> = OnceCell::new();

/// Initializes `LS_COLORS` by reading in the `LS_COLORS` environment variable. If it isn't set, a
/// default determined by `lscolors` crate will be used.
pub fn init_ls_colors() {
    LS_COLORS.set(LsColors::from_env().unwrap_or_default()).unwrap();
}

/// Grabs a reference to `LS_COLORS`. Panics if not initialized.
pub fn get_ls_colors() -> &'static LsColors {
    LS_COLORS.get().expect("LS_COLORS not initialized")
}

