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

use crate::uni_info::cursor::Level;
use crate::uni_info::{Grade, UniInfo};
use key::Key;
use std::io;
use term::Term;

/// An object that represents I/O, data, and information about how to present
/// the data on screen.
pub(super) struct UI<'a> {
    key: Key<'a>,
    term: Term<'a>,
    uni: &'a mut UniInfo,
    offset: usize,
}

impl Drop for UI<'_> {
    fn drop(&mut self) {
        assert!(
            self.finish().is_ok(),
            "Error restoring terminal in UI::drop"
        );
    }
}

impl<'a> UI<'a> {
    /// Sets up the terminal for the user interface, and creates a `UI` instance.
    pub(super) fn new(uni: &'a mut UniInfo) -> io::Result<Self> {
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
    fn finish(&mut self) -> io::Result<()> {
        self.term.enable_line_wrap()?;
        self.term.show_cursor()?;
        self.term.clear_buffer()?;
        self.term.reset_cursor_pos()?;
        self.term.switch_primary_buffer()?;
        self.term.flush()
    }

    /// Main loop. Takes ownership of the `UI` instance, effectively dropping it.
    /// Returns whether user wants to save data to file.
    pub(super) fn main_loop(mut self) -> io::Result<bool> {
        loop {
            self.term.reset_cursor_pos()?;
            self.term.write_skip(self.uni, self.offset)?;
            self.show_keybinds()?;
            self.term.flush()?;
            self.key.read()?;
            match &self.key.as_printable_ascii() {
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
                None if self.key.is_ctrl_e() => self.offset += 1,
                None if self.key.is_ctrl_y() => self.offset = self.offset.saturating_sub(1),
                None if self.key.is_esc() => break,
                _ => {}
            }
        }
        self.save_quit()
    }

    /// Prompt the user for information regarding the creation of the currently
    /// targeted menu entry. Silently returns `Ok` on bad user input.
    fn add_entry(&mut self) -> io::Result<()> {
        match &self.uni.cursor_level() {
            Level::Semester => {
                self.uni.add_semester();
            }
            Level::Period => {
                self.prompt_line("Enter code: ")?;
                let code: String = self.read_line()?;
                self.prompt_line("Enter credits: ")?;
                let grade_opt: Option<Grade> = self.construct_grade()?;
                self.prompt_line("Enter name: ")?;
                let name: String = self.read_line()?;
                if let Some(grade) = grade_opt {
                    self.uni.add_course(code, grade, name);
                }
            }
            Level::Course => {
                self.prompt_line("Enter code: ")?;
                let code: String = self.read_line()?;
                self.prompt_line("Enter code: ")?;
                let grade_opt: Option<Grade> = self.construct_grade()?;
                self.prompt_line("Enter credits: ")?;
                let credits_str: String = self.read_line()?;
                self.prompt_line("Enter description: ")?;
                let description: String = self.read_line()?;
                if let Ok(credits) = credits_str.parse() {
                    if let Some(grade) = grade_opt {
                        if credits >= 0.0 {
                            self.uni.add_moment(code, grade, credits, description);
                        }
                    }
                }
            }
            Level::Moment => {
                self.prompt_line("Enter name: ")?;
                let name: String = self.read_line()?;
                self.uni.add_task(name);
            }
            Level::Task => {}
        }
        Ok(())
    }

    /// Prompt the user for deletion of the targeted entry.
    /// Silently returns `Ok` on bad user input.
    fn delete_entry(&mut self) -> io::Result<()> {
        self.prompt_line("Delete entry? [y]es [n]o")?;
        if self.read_confirm()? {
            self.uni.delete_entry();
        }
        Ok(())
    }

    /// Prompt the user for saving when quitting
    /// Silently returns `Ok(false)` on bad user input.
    fn save_quit(&mut self) -> io::Result<bool> {
        self.prompt_line("Save to file? [y]es [n]o")?;
        self.read_confirm()
    }

    /// Reads user input and returns whether user pressed 'y' or 'Y'.
    fn read_confirm(&mut self) -> io::Result<bool> {
        self.key.read()?;
        Ok(matches!(self.key.as_printable_ascii(), Some('y' | 'Y')))
    }

    /// Prompt the user for information regarding the creation of a new `Grade`
    /// object. Silently returns `Ok` on bad user input.
    fn construct_grade(&mut self) -> io::Result<Option<Grade>> {
        self.prompt_line("Enter type [3] [4] [5] [p]ass [f]fail [o]ngoing")?;
        self.key.read()?;
        match &self.key.as_printable_ascii() {
            Some('3'..='5') => {
                let grade: u8 = self.key.as_char_unchecked() as u8 - b'0';
                Ok(Some(Grade::Grade(grade)))
            }
            Some('p') => Ok(Some(Grade::Completed(true))),
            Some('f') => Ok(Some(Grade::Completed(false))),
            Some('o') => Ok(Some(Grade::Ongoing)),
            _ => Ok(None),
        }
    }

    /// Manipulates the currently targeted entry, whereever possible.
    /// For entries that requires more information to edit, it prompts the user.
    /// Silently returns `Ok` on bad user input.
    fn edit_entry(&mut self) -> io::Result<()> {
        match &self.uni.cursor_level() {
            Level::Semester | Level::Period => {}
            Level::Course => {
                if let Some(grade) = self.construct_grade()? {
                    self.uni.set_course_grade(grade);
                }
            }
            Level::Moment => {
                if let Some(grade) = self.construct_grade()? {
                    self.uni.set_moment_grade(grade);
                }
            }
            Level::Task => self.uni.toggle_selected_task(),
        };
        Ok(())
    }

    /// Replace the current line with the prompt in `text`.
    fn prompt_line(&mut self, text: &str) -> io::Result<()> {
        self.term.move_cursor_line_begin()?;
        self.term.write(&text)?;
        self.term.erase_to_line_end()?;
        self.term.flush()
    }

    /// Read printable utf-8 text until the user presses enter.
    fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        loop {
            self.key.read()?;
            match &self.key.as_printable_utf8() {
                Some(ch) => {
                    line.push(*ch);
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

    /// Prints the available keybinds for each cursor level in the menu.
    fn show_keybinds(&mut self) -> io::Result<()> {
        match &self.uni.cursor_level() {
            Level::Semester => self.prompt_line(
                "S> [a]dd [d]elete        {hjkl | \u{2190}\u{2193}\u{2191}\u{2192}} [q]uit",
            ),
            Level::Period => self.prompt_line(
                "P>                       {hjkl | \u{2190}\u{2193}\u{2191}\u{2192}} [q]uit",
            ),
            Level::Course => self.prompt_line(
                "C> [a]dd [d]elete [e]dit {hjkl | \u{2190}\u{2193}\u{2191}\u{2192}} [q]uit",
            ),
            Level::Moment => self.prompt_line(
                "M> [a]dd [d]elete [e]dit {hjkl | \u{2190}\u{2193}\u{2191}\u{2192}} [q]uit",
            ),
            Level::Task => self.prompt_line(
                "T>       [d]elete [e]dit {hjkl | \u{2190}\u{2193}\u{2191}\u{2192}} [q]uit",
            ),
        }
    }
}
