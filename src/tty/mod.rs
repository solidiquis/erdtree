#![allow(clippy::module_name_repetitions)]
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdin, stdout, IsTerminal};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

/// Is stdin connected to a tty? Should be `false` if `erdtree` is on the receiving end of a
/// pipeline.
pub fn stdin_is_tty() -> bool {
    stdin().is_terminal()
}

/// Is stdout connected to a tty? Should be `false` if output is redirected to a file for example.
pub fn stdout_is_tty() -> bool {
    stdout().is_terminal()
}

/// Restore terminal settings.
pub fn restore_tty() {
    stdout().execute(cursor::Show).unwrap();
}

/// Attempts to get the current size of the tty's window. Returns `None` if stdout isn't tty or if
/// failed to get width.
pub fn get_window_width(stdout_is_tty: bool) -> Option<usize> {
    if !stdout_is_tty {
        return None;
    }

    #[cfg(windows)]
    return unsafe { windows::win_width() };

    #[cfg(unix)]
    return unsafe { unix::win_width() };
}
