//! An object that provides a terminal user interface input and output
//!
//! This module contains a struct that is meant to fully own the terminal
//! output and input for the duration of its lifetime.
//! It is intended to print a `UniInfo` object and provide keybindings for
//! navigating and interacting with it in a user friendly manner.
//! It also integrates well with the interface of the `uni_info` module.
//!
//! The design goal is to be fully error safe, and tries to always restore the
//! terminal to a normal state whenever the `UI` object goes out of scope through
//! the `Drop` trait. Which is why the `main_loop` function takes ownership of,
//! and drops the object when it exits.
//!
//! # Usage
//!
//! An example of the intended usage is:
//!
//! ```
//! let ui = UI::new(&mut uni).unwrap();
//! ui.main_loop().unwrap();
//! println!("Normal terminal is back!");
//! ```

mod key;
pub(super) mod term;

use crate::uni_info::cursor::CursorLevel;
use crate::uni_info::{Grade, UniInfo};
use key::Key;
use std::io::Error;
use term::Term;

pub(super) struct UI<'a> {
    key: Key<'a>,
    term: Term<'a>,
    uni: &'a mut UniInfo,
    offset: usize,
}

impl<'a> Drop for UI<'a> {
    fn drop(&mut self) {
        if self.finish().is_err() {
            panic!("Error restoring terminal in UI::drop");
        }
    }
}

impl<'a> UI<'a> {
    /// Sets up the terminal for the user interface, and creates a `UI` instance.
    pub(super) fn new(uni: &'a mut UniInfo) -> Result<Self, Error> {
        let mut term = Term::new();
        term.set_raw()?;
        term.switch_alternate_buffer()?;
        term.clear_buffer()?;
        term.reset_cursor_pos()?;
        term.hide_cursor()?;
        term.disable_line_wrap()?;
        term.flush()?;
        Ok(UI {
            key: Key::new(),
            term,
            uni,
            offset: 0,
        })
    }

    /// Resets the terminal to the state prior to the `UI` instance creation.
    fn finish(&mut self) -> Result<(), Error> {
        self.term.enable_line_wrap()?;
        self.term.show_cursor()?;
        self.term.clear_buffer()?;
        self.term.reset_cursor_pos()?;
        self.term.switch_primary_buffer()?;
        self.term.flush()
    }

    /// Main loop. Takes ownership of the `UI` instance, effectively dropping it.
    pub(super) fn main_loop(mut self) -> Result<(), Error> {
        loop {
            self.term.reset_cursor_pos()?;
            self.term.write_offset(self.uni, self.offset)?;
            self.show_keybinds()?;
            self.term.flush()?;
            self.key.read()?;
            match self.key.as_printable_ascii() {
                Some(' ') => {
                    self.edit_entry()?;
                    self.uni.cursor_down();
                }
                Some('a') => self.add_entry()?,
                Some('d') => self.delete_entry()?,
                Some('e') => self.edit_entry()?,
                Some('h') => self.uni.cursor_exit(),
                Some('j') => self.uni.cursor_down(),
                Some('k') => self.uni.cursor_up(),
                Some('l') => self.uni.cursor_enter(),
                Some('q') => break,
                None if self.key.is_enter() => self.edit_entry()?,
                None if self.key.is_left() => self.uni.cursor_exit(),
                None if self.key.is_down() => self.uni.cursor_down(),
                None if self.key.is_up() => self.uni.cursor_up(),
                None if self.key.is_right() => self.uni.cursor_enter(),
                None if self.key.is_ctrl_e() => self.offset = self.offset.saturating_sub(1),
                None if self.key.is_ctrl_y() => self.offset += 1,
                None if self.key.is_esc() => break,
                _ => {}
            }
        }
        Ok(())
    }

    /// Prompt the user for information regarding the creation of the currently
    /// targeted menu entry. Silently returns `Ok` on bad user input.
    fn add_entry(&mut self) -> Result<(), Error> {
        match self.uni.cursor_level() {
            CursorLevel::Semester => {
                self.uni.add_semester();
            }
            CursorLevel::Period => {
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
                self.prompt_line("Enter code: ")?;
                let code: String = self.read_line()?;
                self.prompt_line("Enter credits: ")?;
                let credits_str: String = self.read_line()?;
                self.prompt_line("Enter description: ")?;
                let description: String = self.read_line()?;
                if let Ok(credits) = credits_str.parse() {
                    if credits >= 0.0 {
                        self.uni.add_moment(code, credits, description);
                    }
                }
            }
            CursorLevel::Moment => {
                self.prompt_line("Enter name: ")?;
                let name: String = self.read_line()?;
                self.uni.add_task(name);
            }
            CursorLevel::Task => {}
        }
        Ok(())
    }

    /// Prompt the user for deletion of the targeted entry.
    /// Waits for a valid y/n input.
    fn delete_entry(&mut self) -> Result<(), Error> {
        self.prompt_line("Delete entry? [y]es [n]o")?;
        self.key.read()?;
        loop {
            match self.key.as_printable_ascii() {
                Some('y') | Some('Y') => {
                    self.uni.delete_entry();
                    break;
                }
                Some('n') | Some('N') => break,
                _ => {}
            }
        }
        Ok(())
    }

    /// Prompt the user for information regarding the creation of a new `Grade`
    /// object. Silently returns `Ok` on bad user input.
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
                    let grade: usize = self.key.as_char_unchecked() as usize - '0' as usize;
                    return Ok(Some(Grade::Grade(grade)));
                }
                Ok(None)
            }
            Some('o') => Ok(Some(Grade::Ongoing)),
            _ => Ok(None),
        }
    }

    /// Manipulates the currently targeted entry, whereever possible.
    /// For entries that requires more information to edit, it prompts the user.
    /// Silently returns `Ok` on bad user input.
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

    /// Replace the current line with the prompt in `text`.
    fn prompt_line(&mut self, text: &str) -> Result<(), Error> {
        self.term.move_cursor_line_begin()?;
        self.term.write(&text)?;
        self.term.erase_to_line_end()?;
        self.term.flush()
    }

    /// Read printable utf-8 text until the user presses enter.
    fn read_line(&mut self) -> Result<String, Error> {
        let mut line = String::new();
        loop {
            self.key.read()?;
            match self.key.as_printable_utf8() {
                Some(ch) => {
                    line.push(ch);
                    self.term.write(&ch)?;
                }
                None if self.key.is_backspace() => {
                    if line.pop().is_none() {
                        continue;
                    }
                    self.term.move_cursor_left()?;
                    self.term.erase_to_line_end()?;
                }
                None if self.key.is_enter() => break,
                _ => {}
            }
            self.term.flush()?;
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
