//! Functions and data for representing a user keypress
//!
//! This module is intended to fully abstract away the idea of a terminal keypress
//! and provides multiple functions for interpreting the keypress depending
//! on the usage.
//!
//! # Usage
//!
//! An example of the usage is:
//!
//! ```
//! let mut key = Key::new();
//! key.read().unwrap();
//! match key.as_printable_utf8() {
//!     Some('ы') => println!("Key ы pressed!"),
//!     None if key.is_esc() => println!("user wants to exit"),
//!     _ => {},
//! }
//! ```

use std::io::{self, Read, StdinLock};
use std::str;

const ESC: u8 = 27;

/// Maps key names to patterns to match common terminal representations.
/// The terminal representations will differ between terminals.
/// For example `alacritty` and `xterm` will send `0x7f` and `0x08`
/// respectively when pressing backspace.
macro_rules! keycode {
    ("backspace") => {
        [8 | 127]
    };
    ("down") => {
        [ESC, b'[', b'B']
    };
    ("enter") => {
        [b'\r']
    };
    ("esc") => {
        [ESC]
    };
    ("left") => {
        [ESC, b'[', b'D']
    };
    ("right") => {
        [ESC, b'[', b'C']
    };
    ("up") => {
        [ESC, b'[', b'A']
    };
    ("ctrl-e") => {
        [5]
    };
    ("ctrl-y") => {
        [25]
    };
}

/// A struct representing information and functions regarding recieving
/// data from a terminal, especially keypress events.
pub(super) struct Key<'a> {
    data: [u8; 6],
    is: StdinLock<'a>,
    rd: usize,
}

impl Key<'_> {
    /// Returns a new instance of `Key` with all fields default initialized.
    /// Creates a standard input lock.
    pub(super) fn new() -> Self {
        Key {
            data: [0; 6],
            is: io::stdin().lock(),
            rd: 0,
        }
    }

    /// Blocking read a keypress from standard input.
    pub(super) fn read(&mut self) -> io::Result<()> {
        self.rd = self.is.read(&mut self.data[..])?;
        if self.rd == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid input, 0 bytes read",
            ));
        }
        Ok(())
    }

    /// Returns an optional character if the last keypress is representable
    /// as a printable ascii-character.
    pub(super) const fn as_printable_ascii(&self) -> Option<char> {
        if self.rd == 1 && matches!(self.data[0], b' '..=b'~') {
            return Some(self.data[0] as char);
        }
        None
    }

    /// Returns an optional character if the last keypress is representable
    /// as a printable utf8-character.
    pub(super) fn as_printable_utf8(&self) -> Option<char> {
        str::from_utf8(self.data.get(..self.rd).unwrap_or(&[])).map_or(None, |data_str| {
            match data_str.chars().next() {
                Some('\u{00}'..='\u{1f}' | '\u{7f}'..='\u{9f}') => None,
                opt_ch => opt_ch,
            }
        })
    }

    /// Reinterprets the first byte of the keypress as a char.
    pub(super) const fn as_char_unchecked(&self) -> char {
        self.data[0] as char
    }

    /// Checks whether the last keypress is the backspace or delete character.
    pub(super) fn is_backspace(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("backspace")))
    }

    /// Checks whether the last keypress is the enter (return) key.
    pub(super) fn is_enter(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("enter")))
    }

    /// Checks whether the last keypress is the escape key.
    pub(super) fn is_esc(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("esc")))
    }

    /// Checks whether the last keypress is the up-arrow.
    pub(super) fn is_up(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("up")))
    }

    /// Checks whether the last keypress is the down-arrow.
    pub(super) fn is_down(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("down")))
    }

    /// Checks whether the last keypress is the right-arrow.
    pub(super) fn is_right(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("right")))
    }

    /// Checks whether the last keypress is the left-arrow.
    pub(super) fn is_left(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("left")))
    }

    /// Checks whether the last keypress is CTRL-e.
    pub(super) fn is_ctrl_e(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("ctrl-e")))
    }

    /// Checks whether the last keypress is CTRL-y.
    pub(super) fn is_ctrl_y(&self) -> bool {
        matches!(self.data.get(..self.rd), Some(keycode!("ctrl-y")))
    }
}
