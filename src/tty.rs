use is_terminal::IsTerminal;
use once_cell::sync::OnceCell;
use std::io::{stdin, stdout};

/// Is stdin connected to a tty? Should be `false` if `erdtree` is on the receiving end of a
/// pipeline.
static STDIN_IS_TTY: OnceCell<bool> = OnceCell::new();

/// Is stdout connected to a tty? Should be `false` if output is redirected to a file.
static STDOUT_IS_TTY: OnceCell<bool> = OnceCell::new();

/// Initializes statics [STDIN_IS_TTY] and [STDOUT_IS_TTY].
pub fn init_is_tty() {
    let stdin_is_terminal = stdin().is_terminal();
    STDIN_IS_TTY.set(stdin_is_terminal).unwrap();

    let stdout_is_terminal = stdout().is_terminal();
    STDOUT_IS_TTY.set(stdout_is_terminal).unwrap();
}

/// See [STDIN_IS_TTY].
pub fn stdin_is_tty() -> bool {
    STDIN_IS_TTY.get().map(bool::clone).expect("Failed to initialize STDIN_IS_TTY")
}

/// See [STDOUT_IS_TTY].
pub fn stdout_is_tty() -> bool {
    STDOUT_IS_TTY.get().map(bool::clone).expect("Failed to initialize STDOUT_IS_TTY")
}
