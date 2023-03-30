use super::*;
use std::io::{self, Error, Read, Write};

pub fn ui_init() -> Result<libc::termios, Error> {
    write!(io::stdout(), "{TERM_SAVE_BUFFER}")?;
    let old_termios: libc::termios;
    unsafe {
        old_termios = set_raw_terminal_mode();
    };
    Ok(old_termios)
}

pub fn ui_exit(old_termios: &mut libc::termios) -> Result<(), Error> {
    unsafe { set_noraw_terminal_mode(old_termios) };
    write!(io::stdout(), "{TERM_RESET_BUFFER}")?;
    Ok(())
}

pub fn ui_loop(uni_info: &mut UniInfo) -> Result<(), Error> {
    let mut buffer: [u8; 3] = [0; 3];
    loop {
        print!("{TERM_CLEARSCREEN}");
        print_uni_info(&uni_info, &uni_info.cursor);
        io::stdout().flush()?;
        io::stdin().read(&mut buffer[..])?;
        match buffer {
            [b'j', _, _] | [27, 91, 66] => {
                uni_info.increase_cursor();
            }
            [b'k', _, _] | [27, 91, 65] => {
                uni_info.decrease_cursor();
            }
            [b'q', _, _] | [27, 0, 0] => {
                break;
            }
            [b'l', _, _] => {
                uni_info.cursor.enter();
            }
            [b'h', _, _] => {
                uni_info.cursor.exit();
            }
            _ => {}
        }
    }
    Ok(())
}
