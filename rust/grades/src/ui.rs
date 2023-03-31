mod term;

use crate::uni_info::UniInfo;
use std::io::{self, Error, Read, Write};
use term::{
    TERM_BUF_ALT, TERM_BUF_CLR, TERM_BUF_PRI, TERM_CURS_HOME, TERM_CURS_INVIS, TERM_CURS_VIS,
};

enum Key {
    Exit,
    Down,
    Up,
    Enter,
    Edit,
    Abort,
    Bad,
}

impl Key {
    fn from_arr(key: [u8; 3]) -> Self {
        match key {
            [b'h', _, _] | [27, 91, 68] => Key::Exit,
            [b'j', _, _] | [27, 91, 66] => Key::Down,
            [b'k', _, _] | [27, 91, 65] => Key::Up,
            [b'l', _, _] | [27, 91, 67] => Key::Enter,
            [b'e', _, _] | [b' ', _, _] => Key::Edit,
            [b'q', _, _] | [27, 0, 0] => Key::Abort,
            _ => Key::Bad,
        }
    }
}

pub(super) fn init() -> Result<libc::termios, Error> {
    write!(io::stdout(), "{TERM_BUF_ALT}")?;
    write!(io::stdout(), "{TERM_BUF_CLR}")?;
    write!(io::stdout(), "{TERM_CURS_HOME}")?;
    write!(io::stdout(), "{TERM_CURS_INVIS}")?;
    let old_termios: libc::termios = unsafe { term::set_raw_terminal_mode() };
    io::stdout().flush()?;
    Ok(old_termios)
}

pub(super) fn exit(old_termios: &mut libc::termios) -> Result<(), Error> {
    unsafe { term::set_noraw_terminal_mode(old_termios) };
    write!(io::stdout(), "{TERM_CURS_VIS}")?;
    write!(io::stdout(), "{TERM_BUF_CLR}")?;
    write!(io::stdout(), "{TERM_CURS_HOME}")?;
    write!(io::stdout(), "{TERM_BUF_PRI}")?;
    io::stdout().flush()
}

pub(super) fn ui_loop(uni: &mut UniInfo) -> Result<(), Error> {
    let mut key: [u8; 3] = [0; 3];
    loop {
        write!(io::stdout(), "{TERM_CURS_HOME}")?;
        write!(io::stdout(), "{uni}")?;
        let bytes_read: usize = io::stdin().read(&mut key[..])?;
        if bytes_read == 0 {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data on input",
            ));
        }
        match Key::from_arr(key) {
            Key::Exit => uni.cursor_exit(),
            Key::Down => uni.cursor_increase(),
            Key::Up => uni.cursor_decrease(),
            Key::Enter => uni.cursor_enter(),
            Key::Edit => uni.edit_selection(),
            Key::Abort => break,
            Key::Bad => {}
        }
    }
    Ok(())
}
