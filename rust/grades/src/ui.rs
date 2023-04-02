mod key;
pub(super) mod term;

use crate::uni_info::cursor::CursorLevel;
use crate::uni_info::{Grade, UniInfo};
use key::Key;
use std::io::{self, Error, StdoutLock, Write};
use term::{
    BUF_ALT, BUF_CLR, BUF_PRI, CURS_HOME, CURS_INVIS, CURS_LEFT, CURS_VIS, ERASE_TO_LINE_END,
};

pub(super) struct UI<'a> {
    key: Key,
    old_termios: libc::termios,
    os: StdoutLock<'a>,
    uni: &'a mut UniInfo,
}

impl<'a> Drop for UI<'a> {
    fn drop(&mut self) {
        unsafe { term::set_noraw(&mut self.old_termios) };
        write!(self.os, "{}", CURS_VIS).unwrap();
        write!(self.os, "{}", BUF_CLR).unwrap();
        write!(self.os, "{}", CURS_HOME).unwrap();
        write!(self.os, "{}", BUF_PRI).unwrap();
        self.os.flush().unwrap();
    }
}

impl<'a> UI<'a> {
    pub(super) fn new(uni: &'a mut UniInfo) -> Result<Self, Error> {
        let mut os: StdoutLock = io::stdout().lock();
        write!(os, "{}", BUF_ALT)?;
        write!(os, "{}", BUF_CLR)?;
        write!(os, "{}", CURS_HOME)?;
        write!(os, "{}", CURS_INVIS)?;
        let old_termios: libc::termios = unsafe { term::set_raw() };
        os.flush()?;
        Ok(UI {
            key: Key::new(),
            old_termios,
            os,
            uni,
        })
    }

    pub(super) fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            write!(self.os, "{}", CURS_HOME)?;
            write!(self.os, "{}", self.uni)?;
            self.show_keybinds()?;
            self.os.flush()?;
            self.key.read()?;
            match self.key.as_printable_ascii() {
                Some('a') => self.add_entry()?,
                Some('d') => self.uni.delete_entry(),
                Some('e') => self.edit_entry()?,
                Some('h') => self.uni.cursor_exit(),
                Some('j') => self.uni.cursor_increase(),
                Some('k') => self.uni.cursor_decrease(),
                Some('l') => self.uni.cursor_enter(),
                Some('q') => break,
                None if self.key.is_enter() => self.edit_entry()?,
                None if self.key.is_left() => self.uni.cursor_exit(),
                None if self.key.is_down() => self.uni.cursor_increase(),
                None if self.key.is_up() => self.uni.cursor_decrease(),
                None if self.key.is_right() => self.uni.cursor_enter(),
                None if self.key.is_esc() => break,
                _ => {}
            }
        }
        Ok(())
    }

    fn add_entry(&mut self) -> Result<(), Error> {
        match self.uni.cursor_level() {
            CursorLevel::Semester => {
                // Add semester + periods
                self.uni.add_semester();
            }
            CursorLevel::Period => {
                // Add course
                self.prompt_line("Enter code: ")?;
                let code: String = self.read_line()?;
                self.prompt_line("Enter credits: ")?;
                let grade_option: Option<Grade> = self.construct_grade()?;
                self.prompt_line("Enter name: ")?;
                let name: String = self.read_line()?;
                if let Some(grade) = grade_option {
                    self.uni.add_course(code, grade, name);
                }
            }
            CursorLevel::Course => {
                // Add moment
                self.prompt_line("Enter code: ")?;
                let code: String = self.read_line()?;
                self.prompt_line("Enter credits: ")?;
                let credits_str: String = self.read_line()?;
                self.prompt_line("Enter description: ")?;
                let description: String = self.read_line()?;
                if let Ok(credits) = credits_str.parse() {
                    self.uni.add_moment(code, credits, description);
                }
            }
            CursorLevel::Moment => {
                // Add task
                self.prompt_line("Enter name: ")?;
                let name: String = self.read_line()?;
                self.uni.add_task(name);
            }
            CursorLevel::Task => {}
        }
        Ok(())
    }

    fn construct_grade(&mut self) -> Result<Option<Grade>, Error> {
        self.prompt_line("Enter type [c]ompleted [g]rade [o]ngoing")?;
        self.key.read()?;
        match self.key.as_printable_ascii() {
            Some('c') => {
                self.prompt_line("Enter value [p]assed [f]ailed")?;
                self.key.read()?;
                match self.key.as_printable_ascii() {
                    Some('p') => Ok(Some(Grade::Completed(true))),
                    Some('f') => Ok(Some(Grade::Completed(false))),
                    _ => Ok(None),
                }
            }
            Some('g') => {
                self.prompt_line("Enter value [3] [4] [5]")?;
                self.key.read()?;
                if matches!(self.key.as_printable_ascii(), Some('3'..='5')) {
                    let grade: usize = self.key.as_char_unchecked().to_digit(10).unwrap() as usize;
                    return Ok(Some(Grade::Grade(grade)));
                }
                Ok(None)
            }
            Some('o') => Ok(Some(Grade::Ongoing)),
            _ => Ok(None),
        }
    }

    fn edit_entry(&mut self) -> Result<(), Error> {
        match self.uni.cursor_level() {
            CursorLevel::Semester => {}
            CursorLevel::Period => {}
            CursorLevel::Course => {
                if let Some(grade) = self.construct_grade()? {
                    self.uni.set_selected_course(grade);
                }
            }
            CursorLevel::Moment => self.uni.toggle_selected_moment(),
            CursorLevel::Task => self.uni.toggle_selected_task(),
        };
        Ok(())
    }

    fn prompt_line(&mut self, text: &str) -> Result<(), Error> {
        write!(self.os, "\r{text}{end}", end = ERASE_TO_LINE_END)?;
        self.os.flush()
    }

    fn read_line(&mut self) -> Result<String, Error> {
        let mut line = String::new();
        loop {
            self.key.read()?;
            match self.key.as_printable_utf8() {
                Some(ch) => {
                    line.push(ch);
                    write!(self.os, "{}", ch)?;
                }
                None if self.key.is_backspace() => {
                    if line.pop().is_none() {
                        continue;
                    }
                    write!(self.os, "{}{end}", CURS_LEFT, end = ERASE_TO_LINE_END)?;
                }
                None if self.key.is_enter() => break,
                _ => {}
            }
            self.os.flush()?;
        }
        Ok(line)
    }

    fn show_keybinds(&mut self) -> Result<(), Error> {
        match self.uni.cursor_level() {
            CursorLevel::Semester => self.prompt_line("[a]dd [d]elete        {hjkl | ←↓↑→} [q]uit"),
            CursorLevel::Period => self.prompt_line("                      {hjkl | ←↓↑→} [q]uit"),
            CursorLevel::Course => self.prompt_line("[a]dd [d]elete [e]dit {hjkl | ←↓↑→} [q]uit"),
            CursorLevel::Moment => self.prompt_line("[a]dd [d]elete [e]dit {hjkl | ←↓↑→} [q]uit"),
            CursorLevel::Task => self.prompt_line("      [d]elete [e]dit {hjkl | ←↓↑→} [q]uit"),
        }
    }
}
