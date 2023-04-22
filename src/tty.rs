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

#[cfg(windows)]
mod windows {
    use winapi::um::{
        processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE,
        wincon::GetConsoleScreenBufferInfo,
        wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT},
    };

    pub(super) unsafe fn win_width() -> Option<usize> {
        let null_coord = COORD { X: 0, Y: 0 };
        let null_smallrect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        };

        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut console_data = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: null_coord,
            dwCursorPosition: null_coord,
            wAttributes: 0,
            srWindow: null_smallrect,
            dwMaximumWindowSize: null_coord,
        };

        (GetConsoleScreenBufferInfo(stdout_handle, &mut console_data) != 0)
            .then(|| console_data.srWindow.Right - console_data.srWindow.Left + 1)
            .map(usize::try_from)
            .map(Result::ok)
            .flatten()
    }
}

#[cfg(unix)]
mod unix {
    use std::{io, mem::MaybeUninit, os::fd::AsRawFd};

    pub(super) unsafe fn win_width() -> Option<usize> {
        let mut winsize: MaybeUninit<libc::winsize> = MaybeUninit::uninit();
        let tty_fd: libc::c_int = io::stdout().as_raw_fd();

        if libc::ioctl(tty_fd, libc::TIOCGWINSZ, winsize.as_mut_ptr()) != 0 {
            return None;
        }

        let libc::winsize { ws_col, .. } = winsize.assume_init();

        Some(usize::from(ws_col))
    }
}
