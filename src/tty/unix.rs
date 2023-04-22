use std::{io, mem::MaybeUninit, os::fd::AsRawFd};

/// Attempts to get the current width of the tty's window. Returns `None` if error.
pub(super) unsafe fn win_width() -> Option<usize> {
    let mut winsize: MaybeUninit<libc::winsize> = MaybeUninit::uninit();
    let tty_fd: libc::c_int = io::stdout().as_raw_fd();

    if libc::ioctl(tty_fd, libc::TIOCGWINSZ, winsize.as_mut_ptr()) != 0 {
        return None;
    }

    let libc::winsize { ws_col, .. } = winsize.assume_init();

    Some(usize::from(ws_col))
}
