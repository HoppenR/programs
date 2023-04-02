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

use std::io::{self, Error, Read, StdinLock};
use std::str;

const ESC: u8 = 27;

/// Maps key names to patterns to match common terminal representations.
/// The terminal representations will differ between terminals.
/// For example `alacritty` and `xterm` will send `0x7f` and `0x08`
/// respectively when pressing backspace.
macro_rules! keycode {
    ("backspace") => {
        [8] | [127]
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
}

/// `Key` represents data from a keypress.
pub(super) struct Key<'a> {
    data: [u8; 6],
    is: StdinLock<'a>,
    rd: usize,
}

impl<'a> Key<'a> {
    pub(super) fn new() -> Self {
        let is: StdinLock = io::stdin().lock();
        Key {
            data: [0; 6],
            is,
            rd: 0,
        }
    }

    /// Blocking read a keypress from standard input.
    pub(super) fn read(&mut self) -> Result<(), Error> {
        self.rd = self.is.read(&mut self.data[..])?;
        if self.rd == 0 {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "invalid input, 0 bytes read",
            ));
        }
        Ok(())
    }

    /// Returns an optional character if the last keypress is representable
    /// as a printable ascii-character.
    pub(super) fn as_printable_ascii(&self) -> Option<char> {
        if self.rd == 1 {
            if matches!(self.data[0], b' '..=b'~') {
                return Some(self.data[0] as char);
            }
        }
        None
    }

    /// Returns an optional character if the last keypress is representable
    /// as a printable utf8-character.
    pub(super) fn as_printable_utf8(&self) -> Option<char> {
        match str::from_utf8(&self.data[..self.rd]) {
            Ok(data_str) => match data_str.chars().next() {
                Some('\u{00}'..='\u{1f}') => None,
                Some('\u{7f}'..='\u{9f}') => None,
                opt_ch => opt_ch,
            },
            Err(_) => None,
        }
    }

    /// Reinterprets the first byte of the keypress as a char.
    pub(super) fn as_char_unchecked(&self) -> char {
        self.data[0] as char
    }

    /// Checks whether the last keypress is the backspace or delete character.
    pub(super) fn is_backspace(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("backspace"))
    }

    /// Checks whether the last keypress is the enter (return) key.
    pub(super) fn is_enter(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("enter"))
    }

    pub(super) fn is_esc(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("esc"))
    }

    /// Checks whether the last keypress is the up-arrow.
    pub(super) fn is_up(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("up"))
    }

    /// Checks whether the last keypress is the down-arrow.
    pub(super) fn is_down(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("down"))
    }

    /// Checks whether the last keypress is the right-arrow.
    pub(super) fn is_right(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("right"))
    }

    /// Checks whether the last keypress is the left-arrow.
    pub(super) fn is_left(&self) -> bool {
        matches!(self.data[..self.rd], keycode!("left"))
    }
}
