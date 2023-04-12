#![allow(clippy::module_name_repetitions)]
use is_terminal::IsTerminal;
use std::io::{stdin, stdout};

/// Is stdin connected to a tty? Should be `false` if `erdtree` is on the receiving end of a
/// pipeline.
pub fn stdin_is_tty() -> bool {
    stdin().is_terminal()
}

/// Is stdout connected to a tty? Should be `false` if output is redirected to a file.
pub fn stdout_is_tty() -> bool {
    stdout().is_terminal()
}
