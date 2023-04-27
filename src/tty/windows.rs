use winapi::um::{
    processenv::GetStdHandle,
    winbase::STD_OUTPUT_HANDLE,
    wincon::GetConsoleScreenBufferInfo,
    wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT},
};

/// Attempts to get the current width of the tty's window. Returns `None` if error.
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
        .then_some(console_data.srWindow.Right - console_data.srWindow.Left + 1)
        .map(usize::try_from)
        .and_then(Result::ok)
}
