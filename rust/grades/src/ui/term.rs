//! Functions and escape sequences for interacting with a terminal
//!
//! The Term struct is indended to abstract all terminal output for the duration
//! of its lifetime. It provides bindings for setting the terminal into a raw
//! mode that implies a more explicit cursor movement, and offers more immediate
//! data for key events. As well as offering automatic reset of the raw mode
//! when the object goes out of scope.
//!
//! The module (and its child module `attributes` also contain some attribute
//! strings, as well as constants to mark what the terminal should clear. These are
//! meant to be used to format strings with before passing them on to `term.write`.
//!
//! # Usage
//!
//! An example of the usage is:
//!
//! ```
//! {
//!     let mut term = Term::new();
//!     term.set_raw();
//!     term.switch_alternate_buffer()?;
//!     term.write(&format!("immediate key events now!{ERASE_TO_LINE_END}"))?;
//!     term.switch_primary_buffer()?;
//!     term.write(&"Back in normal buffer!{ERASE_TO_DISP_END}")?;
//! }
//! println!("Terminal mode restored after drop. Input is back to what it was prior!");
//! ```

use libc::{c_int, c_ushort, termios, STDOUT_FILENO, TCSANOW, TIOCGWINSZ};
use std::fmt::Display;
use std::io::{self, StdoutLock, Write};
use std::mem;

/// Terminal codes to change the attributes of text.
/// This is a convenience module, intended to be imported as `term::attributes::*;`
/// so that you can directly include attribute constants in formatting strings.
///
/// # Usage
///
/// An example usage of this is:
///
/// ```
/// use term::attributes::*;
/// println!("{RED}{BLD}Red bold text here!{RST} Normal text now");
/// ```
pub(crate) mod attributes {
    /// Reset the color and attributes.
    pub(crate) const RST: &str = "\x1b[0m";
    /// Set attribute `bold`.
    pub(crate) const BLD: &str = "\x1b[1m";
    /// Set attribute `cursive`.
    pub(crate) const CUR: &str = "\x1b[3m";
    /// Set attribute `underline`.
    pub(crate) const UDL: &str = "\x1b[4m";
    /// Set attribute `strikethrough`.
    pub(crate) const STK: &str = "\x1b[9m";
    /// Set color `red`.
    pub(crate) const RED: &str = "\x1b[91m";
    /// Set color `green`.
    pub(crate) const GRN: &str = "\x1b[92m";
    /// Set color `yellow`.
    pub(crate) const YLW: &str = "\x1b[93m";
    /// Set color `blue`.
    pub(crate) const BLU: &str = "\x1b[94m";
    /// Set color `cyan`.
    pub(crate) const CYN: &str = "\x1b[96m";
}

/// Clear the terminal contents from the terminal cursor to the end of the line.
pub(crate) const ERASE_TO_LINE_END: &str = "\x1b[0K";
/// Clear the terminal contents from the terminal cursor to the end of the display.
pub(crate) const ERASE_TO_DISP_END: &str = "\x1b[J";
/// Switch to the alternate terminal buffer.
const BUFFER_ALTERNATE: &str = "\x1b[?1049h";
/// Clear the current buffer.
const BUFFER_CLEAR: &str = "\x1b2J";
/// Switch to the primary terminal buffer.
const BUFFER_PRIMARY: &str = "\x1b[?1049l";
/// Move the cursor to upper left corner `(0, 0)`.
const CURSOR_HOME: &str = "\x1b[H";
/// Hide the terminal cursor.
const CURSOR_INVIS: &str = "\x1b[?25l";
/// Show the terminal cursor.
const CURSOR_VISIBLE: &str = "\x1b[?25h";
/// Move the cursor one step to the left.
const CURSOR_LEFT: &str = "\x1b[D";
/// Disable line wrap
const LINEWRAP_DISABLE: &str = "\x1b[?7l";
/// Enable line wrap
const LINEWRAP_ENABLE: &str = "\x1b[?7h";

pub(super) struct Term<'a> {
    old_termios: termios,
    os: StdoutLock<'a>,
    is_raw: bool,
    size: TermSize,
}

struct TermSize {
    row: c_ushort,
    #[allow(dead_code)]
    col: c_ushort,
    #[allow(dead_code)]
    x_pixsz: c_ushort,
    #[allow(dead_code)]
    y_pxsz: c_ushort,
}

impl Drop for Term<'_> {
    fn drop(&mut self) {
        assert!(
            self.set_old_termios().is_ok(),
            "Error restoring terminal in Term::drop"
        );
    }
}

impl Term<'_> {
    /// Creates a `Term` object that simplifies interacting with a terminal
    /// that may be put into raw mode.
    pub(super) fn new() -> Self {
        Term {
            old_termios: unsafe { mem::zeroed() },
            is_raw: false,
            os: io::stdout().lock(),
            size: unsafe { mem::zeroed() },
        }
    }

    /// Sets the terminal into raw-mode. Saves the terminal I/O interfaces settings
    /// that can be used to restore the terminal after it is finished being used raw.
    pub(super) fn set_raw(&mut self) -> io::Result<()> {
        if self.is_raw {
            return Ok(());
        }
        unsafe {
            cvt_err(libc::tcgetattr(STDOUT_FILENO, &mut self.old_termios))?;
            let mut raw_termios: termios = self.old_termios;
            libc::cfmakeraw(&mut raw_termios);
            cvt_err(libc::tcsetattr(STDOUT_FILENO, TCSANOW, &raw_termios))?;
        }
        self.is_raw = true;
        Ok(())
    }

    pub(super) fn write_offset<T>(&mut self, contents: &T, offset: usize) -> io::Result<()>
    where
        T: Display,
    {
        self.update_size()?;
        let output: String = format!("{contents}")
            .lines()
            .skip(offset)
            .take(self.size.row.into())
            .collect::<Vec<_>>()
            .join("\n");
        self.write(&output)
    }

    fn update_size(&mut self) -> io::Result<()> {
        unsafe { cvt_err(libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut self.size)) }
    }

    pub(super) fn write<T>(&mut self, contents: &T) -> io::Result<()>
    where
        T: Display,
    {
        write!(self.os, "{contents}")
    }

    pub(super) fn switch_alternate_buffer(&mut self) -> io::Result<()> {
        write!(self.os, "{BUFFER_ALTERNATE}")
    }

    pub(super) fn switch_primary_buffer(&mut self) -> io::Result<()> {
        write!(self.os, "{BUFFER_PRIMARY}")
    }

    pub(super) fn clear_buffer(&mut self) -> io::Result<()> {
        write!(self.os, "{BUFFER_CLEAR}")
    }

    pub(super) fn reset_cursor_pos(&mut self) -> io::Result<()> {
        write!(self.os, "{CURSOR_HOME}")
    }

    pub(super) fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.os, "{CURSOR_INVIS}")
    }

    pub(super) fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.os, "{CURSOR_VISIBLE}")
    }

    pub(super) fn move_cursor_left(&mut self) -> io::Result<()> {
        write!(self.os, "{CURSOR_LEFT}")
    }

    pub(super) fn move_cursor_line_begin(&mut self) -> io::Result<()> {
        write!(self.os, "\r")
    }

    pub(super) fn erase_to_line_end(&mut self) -> io::Result<()> {
        write!(self.os, "{ERASE_TO_LINE_END}")
    }

    pub(super) fn disable_line_wrap(&mut self) -> io::Result<()> {
        write!(self.os, "{LINEWRAP_DISABLE}")
    }

    pub(super) fn enable_line_wrap(&mut self) -> io::Result<()> {
        write!(self.os, "{LINEWRAP_ENABLE}")
    }

    pub(super) fn flush(&mut self) -> io::Result<()> {
        self.os.flush()
    }

    /// Resets the terminal to the terminal I/O interfaces settings in `termios`
    /// if it is currently in raw mode.
    fn set_old_termios(&mut self) -> io::Result<()> {
        if !self.is_raw {
            return Ok(());
        }
        unsafe {
            cvt_err(libc::tcsetattr(STDOUT_FILENO, TCSANOW, &self.old_termios))?;
        }
        self.is_raw = false;
        Ok(())
    }
}

/// Converts a return value from a libc syscall to an error if the value
/// is -1 and indicates an os error.
fn cvt_err(value: c_int) -> io::Result<()> {
    if value == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
