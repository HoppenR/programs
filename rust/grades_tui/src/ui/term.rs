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

pub(crate) mod attributes {
    //! Terminal codes to change the attributes of text.
    //! This is a convenience module, intended to be imported as `term::attributes::*;`
    //! so that you can directly include attribute constants in formatting strings.
    //!
    //! # Usage
    //!
    //! An example usage of this is:
    //!
    //! ```
    //! use term::attributes::*;
    //! println!("{RED}{BLD}Red bold text here!{RST} Normal text now");
    //! ```
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
const BUFFER_ALTERNATE: &str = "\x1b[?1049h";
const BUFFER_CLEAR: &str = "\x1b2J";
const BUFFER_PRIMARY: &str = "\x1b[?1049l";
const CURSOR_HOME: &str = "\x1b[H";
const CURSOR_INVIS: &str = "\x1b[?25l";
const CURSOR_VISIBLE: &str = "\x1b[?25h";
const CURSOR_LEFT: &str = "\x1b[D";
const LINEWRAP_DISABLE: &str = "\x1b[?7l";
const LINEWRAP_ENABLE: &str = "\x1b[?7h";

/// A struct representing information and functions regarding sending
/// information to a terminal.
pub(super) struct Term<'a> {
    old_termios: termios,
    os: StdoutLock<'a>,
    is_raw: bool,
    size: TermSize,
}

/// Compatibility libc struct for passing to libc functions.
#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    x_pixsz: c_ushort,
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
    /// Creates a `Term` object with all fields zero-initialized.
    /// Creates a standard output lock.
    pub(super) fn new() -> Self {
        Term {
            old_termios: unsafe { mem::zeroed() },
            is_raw: false,
            os: io::stdout().lock(),
            size: unsafe { mem::zeroed() },
        }
    }

    /// Sets the terminal into raw-mode. Saves the terminal I/O interfaces settings
    /// that are used to restore the terminal after it is finished being used raw.
    pub(super) fn set_raw(&mut self) -> io::Result<()> {
        if self.is_raw {
            return Ok(());
        }
        unsafe {
            cvt_err(libc::tcgetattr(STDOUT_FILENO, &mut self.old_termios))?;
            let mut raw_termios: termios = self.old_termios;
            libc::cfmakeraw(&mut raw_termios);
            cvt_err(libc::tcsetattr(STDOUT_FILENO, TCSANOW, &raw_termios))?;
        };
        self.is_raw = true;
        Ok(())
    }

    /// Format and write the contents to stdout, skipping the first `offset` lines.
    pub(super) fn write_skip<T>(&mut self, contents: &T, offset: usize) -> io::Result<()>
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

    /// Updates information about the size of the terminal.
    fn update_size(&mut self) -> io::Result<()> {
        unsafe { cvt_err(libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut self.size)) }
    }

    /// Format and write the contents to stdout.
    pub(super) fn write<T>(&mut self, contents: &T) -> io::Result<()>
    where
        T: Display,
    {
        write!(self.os, "{contents}")
    }

    /// Switch to the alternate terminal buffer.
    pub(super) fn switch_alternate_buffer(&mut self) -> io::Result<()> {
        self.os.write_all(BUFFER_ALTERNATE.as_bytes())
    }

    /// Clear the current terminal buffer.
    pub(super) fn clear_buffer(&mut self) -> io::Result<()> {
        self.os.write_all(BUFFER_CLEAR.as_bytes())
    }

    /// Switch to the primary terminal buffer.
    pub(super) fn switch_primary_buffer(&mut self) -> io::Result<()> {
        self.os.write_all(BUFFER_PRIMARY.as_bytes())
    }

    /// Move the cursor to upper left corner `(0, 0)`.
    pub(super) fn reset_cursor_pos(&mut self) -> io::Result<()> {
        self.os.write_all(CURSOR_HOME.as_bytes())
    }

    /// Hide the terminal cursor.
    pub(super) fn hide_cursor(&mut self) -> io::Result<()> {
        self.os.write_all(CURSOR_INVIS.as_bytes())
    }

    /// Show the terminal cursor.
    pub(super) fn show_cursor(&mut self) -> io::Result<()> {
        self.os.write_all(CURSOR_VISIBLE.as_bytes())
    }

    /// Move the cursor one step to the left.
    pub(super) fn move_cursor_left(&mut self) -> io::Result<()> {
        self.os.write_all(CURSOR_LEFT.as_bytes())
    }

    /// Moves the cursor to the beginning of the current line.
    pub(super) fn move_cursor_line_begin(&mut self) -> io::Result<()> {
        self.os.write_all(b"\r")
    }

    /// Clear the terminal contents from the terminal cursor to the end of the line.
    pub(super) fn erase_to_line_end(&mut self) -> io::Result<()> {
        self.os.write_all(ERASE_TO_LINE_END.as_bytes())
    }

    /// Disable line wrap, making writes off the edge of the screen appear on the next.
    pub(super) fn disable_line_wrap(&mut self) -> io::Result<()> {
        self.os.write_all(LINEWRAP_DISABLE.as_bytes())
    }

    /// Enable line wrap, making writes off the edge of the screen disappear.
    pub(super) fn enable_line_wrap(&mut self) -> io::Result<()> {
        self.os.write_all(LINEWRAP_ENABLE.as_bytes())
    }

    /// Flush the terminal buffer. Making sure all characters rech their destination.
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
        };
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
