use std::io::{self, Error, Read};

pub(super) struct Key {
    data: [u8; 3],
}

impl Key {
    pub(super) fn new() -> Self {
        Key { data: [0; 3] }
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

    pub(super) fn as_char(&self) -> char {
        self.data[0] as char
    }

    pub(super) fn is_abort(&self) -> bool {
        matches!(self.data, [b'q', _, _] | [27, 0, 0])
    }

    pub(super) fn is_down(&self) -> bool {
        matches!(self.data, [b'j', _, _] | [27, 91, 66])
    }

    pub(super) fn is_edit(&self) -> bool {
        matches!(self.data, [b'e', _, _] | [b' ', _, _])
    }

    pub(super) fn is_enter(&self) -> bool {
        matches!(self.data, [b'l', _, _] | [27, 91, 67])
    }

    pub(super) fn is_exit(&self) -> bool {
        matches!(self.data, [b'h', _, _] | [27, 91, 68])
    }

    pub(super) fn is_up(&self) -> bool {
        matches!(self.data, [b'k', _, _] | [27, 91, 65])
    }
}
