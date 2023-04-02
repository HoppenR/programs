use std::io::{self, Error, Read};
use std::str;

pub(super) struct Key {
    data: [u8; 6],
}

impl Key {
    pub(super) fn new() -> Self {
        Key { data: [0; 6] }
    }

    pub(super) fn read(&mut self) -> Result<(), Error> {
        let bytes_read: usize = io::stdin().read(&mut self.data[..])?;
        if bytes_read == 0 {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "invalid input, 0 bytes read",
            ));
        }
        Ok(())
    }

    pub(super) fn as_printable_ascii(&self) -> Option<char> {
        if matches!(self.data, [b' '..=b'~', ..]) {
            return Some(self.data[0] as char);
        }
        None
    }

    pub(super) fn as_printable_utf8(&self) -> Option<char> {
        match str::from_utf8(&self.data[..]) {
            Ok(data_str) => match data_str.chars().next() {
                Some('\u{00}'..='\u{1f}') => None,
                Some('\u{7f}'..='\u{9f}') => None,
                opt_ch => opt_ch,
            },
            Err(_) => None,
        }
    }

    pub(super) fn as_char_unchecked(&self) -> char {
        self.data[0] as char
    }

    pub(super) fn is_backspace(&self) -> bool {
        matches!(self.data, [127, ..])
    }

    pub(super) fn is_enter(&self) -> bool {
        matches!(self.data, [13, ..])
    }

    pub(super) fn is_esc(&self) -> bool {
        matches!(self.data, [27, b'\0', ..])
    }

    pub(super) fn is_up(&self) -> bool {
        matches!(self.data, [27, b'[', b'A', ..])
    }

    pub(super) fn is_down(&self) -> bool {
        matches!(self.data, [27, b'[', b'B', ..])
    }

    pub(super) fn is_right(&self) -> bool {
        matches!(self.data, [27, b'[', b'C', ..])
    }

    pub(super) fn is_left(&self) -> bool {
        matches!(self.data, [27, b'[', b'D', ..])
    }
}
