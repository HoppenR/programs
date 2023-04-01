mod key;
pub(super) mod term;

use crate::uni_info::cursor::CursorLevel;
use crate::uni_info::{Grade, UniInfo};
use key::Key;
use std::io::{stdout, Error, Write};
use term::{
    BUF_ALT, BUF_CLR, BUF_PRI, CURS_HOME, CURS_INVIS, CURS_LEFT, CURS_VIS, ERASE_TO_LINE_END,
};

pub(super) fn init() -> Result<libc::termios, Error> {
    write!(stdout(), "{}", BUF_ALT)?;
    write!(stdout(), "{}", BUF_CLR)?;
    write!(stdout(), "{}", CURS_HOME)?;
    write!(stdout(), "{}", CURS_INVIS)?;
    let old_termios: libc::termios = unsafe { term::set_raw() };
    stdout().flush()?;
    Ok(old_termios)
}

pub(super) fn exit(old_termios: &mut libc::termios) -> Result<(), Error> {
    unsafe { term::set_noraw(old_termios) };
    write!(stdout(), "{}", CURS_VIS)?;
    write!(stdout(), "{}", BUF_CLR)?;
    write!(stdout(), "{}", CURS_HOME)?;
    write!(stdout(), "{}", BUF_PRI)?;
    stdout().flush()
}

fn prompt_line(text: &str) -> Result<(), Error> {
    write!(stdout(), "\r{text}{end}", end = ERASE_TO_LINE_END)?;
    stdout().flush()
}

fn read_line(key: &mut Key) -> Result<String, Error> {
    let mut line = String::new();
    loop {
        key.read()?;
        match key.as_char() {
            Some('!'..='~') => {
                line.push(key.as_char_unchecked());
                write!(stdout(), "{}", key.as_char_unchecked())?;
            }
            None if key.is_space() => {
                line.push(' ');
                write!(stdout(), " ")?;
            }
            None if key.is_backspace() => {
                if line.pop().is_none() {
                    continue;
                }
                write!(stdout(), "{}{end}", CURS_LEFT, end = ERASE_TO_LINE_END)?;
            }
            None if key.is_enter() => break,
            _ => {}
        }
        stdout().flush()?;
    }
    Ok(line)
}

fn construct_grade(key: &mut Key) -> Result<Option<Grade>, Error> {
    prompt_line("Enter type [c]ompleted [g]rade [o]ngoing")?;
    key.read()?;
    match key.as_char() {
        Some('c') => {
            prompt_line("Enter value [p]assed [f]ailed")?;
            key.read()?;
            match key.as_char() {
                Some('p') => Ok(Some(Grade::Completed(true))),
                Some('f') => Ok(Some(Grade::Completed(false))),
                _ => Ok(None),
            }
        }
        Some('g') => {
            prompt_line("Enter value [3] [4] [5]")?;
            key.read()?;
            if matches!(&key.as_char(), Some('3'..='5')) {
                let grade: usize = key.as_char_unchecked().to_digit(10).unwrap() as usize;
                return Ok(Some(Grade::Grade(grade)));
            }
            Ok(None)
        }
        Some('o') => Ok(Some(Grade::Ongoing)),
        _ => Ok(None),
    }
}

fn add_entry(uni: &mut UniInfo, key: &mut Key) -> Result<(), Error> {
    match uni.cursor_level() {
        CursorLevel::Semester => {
            // Add semester + periods
            uni.add_semester();
        }
        CursorLevel::Period => {
            // Add course
            prompt_line("Enter code: ")?;
            let code: String = read_line(key)?;
            prompt_line("Enter credits: ")?;
            let grade_option: Option<Grade> = construct_grade(key)?;
            prompt_line("Enter name: ")?;
            let name: String = read_line(key)?;
            if let Some(grade) = grade_option {
                uni.add_course(code, grade, name);
            }
        }
        CursorLevel::Course => {
            // Add moment
            prompt_line("Enter code: ")?;
            let code: String = read_line(key)?;
            prompt_line("Enter credits: ")?;
            let credits_str: String = read_line(key)?;
            prompt_line("Enter description: ")?;
            let description: String = read_line(key)?;
            if let Ok(credits) = credits_str.parse() {
                uni.add_moment(code, credits, description);
            }
        }
        CursorLevel::Moment => {
            // Add task
            prompt_line("Enter name: ")?;
            let name: String = read_line(key)?;
            uni.add_task(name);
        }
        CursorLevel::Task => {}
    }
    Ok(())
}

fn edit_entry(uni: &mut UniInfo, key: &mut Key) -> Result<(), Error> {
    match uni.cursor_level() {
        CursorLevel::Semester => {}
        CursorLevel::Period => {}
        CursorLevel::Course => {
            if let Some(grade) = construct_grade(key)? {
                uni.set_selected_course(grade);
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
        write!(stdout(), "{}", CURS_HOME)?;
        write!(stdout(), "{uni}")?;
        stdout().flush()?;
        key.read()?;
        match key.as_char() {
            Some('a') => add_entry(uni, &mut key)?,
            Some('d') => uni.delete_entry(),
            Some('e') => edit_entry(uni, &mut key)?,
            Some('h') => uni.cursor_exit(),
            Some('j') => uni.cursor_increase(),
            Some('k') => uni.cursor_decrease(),
            Some('l') => uni.cursor_enter(),
            Some('q') => break,
            None if key.is_enter() => edit_entry(uni, &mut key)?,
            None if key.is_left() => uni.cursor_exit(),
            None if key.is_down() => uni.cursor_increase(),
            None if key.is_up() => uni.cursor_decrease(),
            None if key.is_right() => uni.cursor_enter(),
            None if key.is_esc() => break,
            _ => {}
        }
    }
    Ok(())
}
