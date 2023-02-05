use lscolors::LsColors;
use once_cell::sync::OnceCell;

pub mod node;
pub mod order;
pub mod tree;

pub static LS_COLORS: OnceCell<LsColors> = OnceCell::new();

pub fn init_ls_colors() {
    LS_COLORS.set(LsColors::from_env().unwrap_or_default()).unwrap();
}

pub fn get_ls_colors() -> &'static LsColors {
    LS_COLORS.get().expect("LS_COLORS not initialized")
}

