//! Functions and escape sequences for interacting with a terminal
//!
//! This module contains some basic predefined strings indended to communicate
//! with most, if not all terminals.
//! The `set_raw` and `set_noraw` functions use the `libc` predefined functions
//! `tcgetattr`, `cfmakeraw`, and `tcsetattr` to set the terminal into a raw
//! mode that implies a more explicit cursor movement, and offers more immediate
//! data for key events.
//!
//! # Usage
//!
//! An example of the usage is:
//!
//! ```
//! let old_ios = unsafe { set_raw() };
//! print!("{BUF_ALT}");
//! println!("{RED}Red text in alt terminal buffer{RST}");
//! print!("{BUF_PRI}");
//! unsafe { set_noraw(&old_ios) };
//! println!("Back in normal buffer!");
//! ```

use std::mem;

/// Reset the color and attributes.
pub(crate) const RST: &str = "\x1b[0m";
/// Set attribute `bold`.
pub(crate) const BLD: &str = "\x1b[1m";
/// Set attribute `cursive`.
pub(crate) const CUR: &str = "\x1b[3m";
/// Set attribute `underline`.
pub(crate) const UDL: &str = "\x1b[4m";
/// Set attribute `strikethrough`.
pub(crate) const STK: &str = "\x1b[9m";
/// Set color `red`.
pub(crate) const RED: &str = "\x1b[91m";
/// Set color `green`.
pub(crate) const GRN: &str = "\x1b[92m";
/// Set color `yellow`.
pub(crate) const YLW: &str = "\x1b[93m";
/// Set color `blue`.
pub(crate) const BLU: &str = "\x1b[94m";
/// Set color `cyan`.
pub(crate) const CYN: &str = "\x1b[96m";
/// Clear the terminal contents from the terminal cursor to the end of the line.
pub(crate) const ERASE_TO_LINE_END: &str = "\x1b[0K";
/// Clear the terminal contents from the terminal cursor to the end of the display.
pub(crate) const ERASE_TO_DISP_END: &str = "\x1b[J";
/// Switch to the alternate terminal buffer.
pub(super) const BUF_ALT: &str = "\x1b[?1049h";
/// Clear the current buffer.
pub(super) const BUF_CLR: &str = "\x1b2J";
/// Switch to the primary terminal buffer.
pub(super) const BUF_PRI: &str = "\x1b[?1049l";
/// Move the cursor to upper left corner `(0, 0)`.
pub(super) const CURS_HOME: &str = "\x1b[H";
/// Hide the terminal cursor.
pub(super) const CURS_INVIS: &str = "\x1b[?25l";
/// Show the terminal cursor.
pub(super) const CURS_VIS: &str = "\x1b[?25h";
/// Move the cursor one step to the left.
pub(super) const CURS_LEFT: &str = "\x1b[D";

/// Resets the terminal to the terminal I/O interfaces settings in `termios`.
pub(super) unsafe fn set_noraw(old_termios: &libc::termios) {
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, old_termios);
}

/// Sets the terminal into raw-mode. Returns the terminal I/O interfaces settings
/// that can be used to restore the terminal after it is finished being used raw.
pub(super) unsafe fn set_raw() -> libc::termios {
    let mut old_termios: libc::termios = mem::zeroed();
    libc::tcgetattr(libc::STDOUT_FILENO, &mut old_termios);
    let mut raw_termios: libc::termios = old_termios;
    libc::cfmakeraw(&mut raw_termios);
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, &raw_termios);
    old_termios
}
