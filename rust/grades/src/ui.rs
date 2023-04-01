mod key;
mod term;

use crate::uni_info::*;
use key::Key;
use std::io::{self, Error, Write};
pub(super) use term::*;

pub(super) fn init() -> Result<libc::termios, Error> {
    write!(io::stdout(), "{}", BUF_ALT)?;
    write!(io::stdout(), "{}", BUF_CLR)?;
    write!(io::stdout(), "{}", CURS_HOME)?;
    write!(io::stdout(), "{}", CURS_INVIS)?;
    let old_termios: libc::termios = unsafe { set_raw_terminal_mode() };
    io::stdout().flush()?;
    Ok(old_termios)
}

pub(super) fn exit(old_termios: &mut libc::termios) -> Result<(), Error> {
    unsafe { set_noraw_terminal_mode(old_termios) };
    write!(io::stdout(), "{}", CURS_VIS)?;
    write!(io::stdout(), "{}", BUF_CLR)?;
    write!(io::stdout(), "{}", CURS_HOME)?;
    write!(io::stdout(), "{}", BUF_PRI)?;
    io::stdout().flush()
}

fn prompt_line(text: &str) -> Result<(), Error> {
    write!(io::stdout(), "\r{text}{end}", end = ERASE_TO_LINE_END)?;
    io::stdout().flush()
}

fn edit_selection(uni: &mut UniInfo, key: &mut Key) -> Result<(), Error> {
    match uni.cursor.level {
        CursorLevel::Semester => {}
        CursorLevel::Period => {}
        CursorLevel::Course => {
            prompt_line("Enter type [c]ompleted [g]rade [o]ngoing{end}")?;
            key.read()?;
            match key.as_char() {
                'c' => {
                    prompt_line("Enter value [p]assed [f]ailed")?;
                    key.read()?;
                    match key.as_char() {
                        'p' => uni.set_selected_course(Grade::Completed(true)),
                        'f' => uni.set_selected_course(Grade::Completed(false)),
                        _ => {}
                    }
                }
                'g' => {
                    prompt_line("Enter value [3] [4] [5]")?;
                    key.read()?;
                    if let '3'..='5' = key.as_char() {
                        let grade: usize = key.as_char().to_digit(10).unwrap() as usize;
                        uni.set_selected_course(Grade::Grade(grade))
                    }
                }
                'o' => uni.set_selected_course(Grade::Ongoing),
                _ => {}
            }
        }
        CursorLevel::Moment => uni.toggle_selected_moment(),
        CursorLevel::Task => uni.toggle_selected_task(),
    };
    Ok(())
}

pub(super) fn ui_loop(uni: &mut UniInfo) -> Result<(), Error> {
    let mut key = Key::new();
    loop {
        write!(io::stdout(), "{}", CURS_HOME)?;
        write!(io::stdout(), "{uni}")?;
        io::stdout().flush()?;
        key.read()?;
        match key {
            ref key if key.is_abort() => break,
            ref key if key.is_down() => uni.cursor_increase(),
            ref mut key if key.is_edit() => edit_selection(uni, key)?,
            ref key if key.is_enter() => uni.cursor_enter(),
            ref key if key.is_exit() => uni.cursor_exit(),
            ref key if key.is_up() => uni.cursor_decrease(),
            _ => {}
        }
    }
    Ok(())
}
