use std::mem;
pub const INDENT: &str = "    ";
pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
pub const CUR: &str = "\x1b[3m";
pub const UDL: &str = "\x1b[4m";
pub const STK: &str = "\x1b[9m";
pub const RED: &str = "\x1b[91m";
pub const GRN: &str = "\x1b[92m";
pub const YLW: &str = "\x1b[93m";
pub const BLU: &str = "\x1b[94m";
pub const CYN: &str = "\x1b[96m";
pub const TERM_SAVE_BUFFER: &str = "\x1b[?1049h";
pub const TERM_RESET_BUFFER: &str = "\x1b[?1049l";
pub const TERM_CLEARSCREEN: &str = "\x1bc";


pub unsafe fn set_noraw_terminal_mode(old_termios: &mut libc::termios) {
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, old_termios);
}

pub unsafe fn set_raw_terminal_mode() -> libc::termios {
    let mut old_termios: libc::termios = mem::zeroed();
    libc::tcgetattr(libc::STDOUT_FILENO, &mut old_termios);
    let mut raw_termios = old_termios.clone();
    libc::cfmakeraw(&mut raw_termios);
    libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, &mut raw_termios);
    return old_termios;
}
