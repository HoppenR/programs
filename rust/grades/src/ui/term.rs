use std::mem;

pub(crate) const RST: &str = "\x1b[0m"; // Reset
pub(crate) const BLD: &str = "\x1b[1m"; // Bold
pub(crate) const CUR: &str = "\x1b[3m"; // Cursive
pub(crate) const UDL: &str = "\x1b[4m"; // Underline
pub(crate) const STK: &str = "\x1b[9m"; // Strikethrough
pub(crate) const RED: &str = "\x1b[91m"; // Red
pub(crate) const GRN: &str = "\x1b[92m"; // Green
pub(crate) const YLW: &str = "\x1b[93m"; // Yellow
pub(crate) const BLU: &str = "\x1b[94m"; // Blue
pub(crate) const CYN: &str = "\x1b[96m"; // Cyan

pub(crate) const ERASE_TO_LINE_END: &str = "\x1b[0K";
pub(crate) const ERASE_TO_DISP_END: &str = "\x1b[J";

pub(super) const BUF_ALT: &str = "\x1b[?1049h";
pub(super) const BUF_CLR: &str = "\x1b2J";
pub(super) const BUF_PRI: &str = "\x1b[?1049l";
pub(super) const CURS_HOME: &str = "\x1b[H";
pub(super) const CURS_INVIS: &str = "\x1b[?25l";
pub(super) const CURS_VIS: &str = "\x1b[?25h";

pub(super) unsafe fn set_noraw_terminal_mode(old_termios: &mut libc::termios) {
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, old_termios);
}

pub(super) unsafe fn set_raw_terminal_mode() -> libc::termios {
    let mut old_termios: libc::termios = mem::zeroed();
    libc::tcgetattr(libc::STDOUT_FILENO, &mut old_termios);
    let mut raw_termios: libc::termios = old_termios;
    libc::cfmakeraw(&mut raw_termios);
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, &raw_termios);
    old_termios
}
