use std::mem;

pub(super) const TERM_BUF_ALT: &str = "\x1b[?1049h";
pub(super) const TERM_BUF_CLR: &str = "\x1b2J";
pub(super) const TERM_BUF_PRI: &str = "\x1b[?1049l";
pub(super) const TERM_CURS_HOME: &str = "\x1b[H";
pub(super) const TERM_CURS_INVIS: &str = "\x1b[?25l";
pub(super) const TERM_CURS_VIS: &str = "\x1b[?25h";

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
