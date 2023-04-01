use std::{
    fmt::Display,
    io::{self, Error, Read},
};

pub(super) struct Key {
    data: [u8; 5],
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.data[0], self.data[1], self.data[2], self.data[3], self.data[4]
        )
    }
}

impl Key {
    pub(super) fn new() -> Self {
        Key { data: [0; 5] }
    }

    pub(super) fn read(&mut self) -> Result<(), Error> {
        self.data.fill(0);
        let bytes_read: usize = io::stdin().read(&mut self.data[..])?;
        if bytes_read == 0 {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "invalid input, 0 bytes read",
            ));
        }
        Ok(())
    }

    pub(super) fn is_printable(&self) -> bool {
        matches!(self.data, [b'!'..=b'~', 0, 0, 0, 0])
    }

    pub(super) fn as_char(&self) -> Option<char> {
        if self.is_printable() {
            return Some(self.data[0] as char);
        }
        None
    }

    pub(super) fn as_char_unchecked(&self) -> char {
        self.data[0] as char
    }

    pub(super) fn is_space(&self) -> bool {
        matches!(self.data, [32, 0, 0, 0, 0])
    }

    pub(super) fn is_backspace(&self) -> bool {
        matches!(self.data, [127, 0, 0, 0, 0])
    }

    pub(super) fn is_enter(&self) -> bool {
        matches!(self.data, [13, 0, 0, 0, 0])
    }

    pub(super) fn is_esc(&self) -> bool {
        matches!(self.data, [27, 0, 0, 0, 0])
    }

    pub(super) fn is_up(&self) -> bool {
        matches!(self.data, [27, 91, 65, 0, 0])
    }

    pub(super) fn is_down(&self) -> bool {
        matches!(self.data, [27, 91, 66, 0, 0])
    }

    pub(super) fn is_right(&self) -> bool {
        matches!(self.data, [27, 91, 67, 0, 0])
    }

    pub(super) fn is_left(&self) -> bool {
        matches!(self.data, [27, 91, 68, 0, 0])
    }
}
