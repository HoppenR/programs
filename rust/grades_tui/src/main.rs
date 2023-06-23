//! A program to navigate, edit, and keep track of university grades
//!
//! This terminal program aims to provide easy, safe, and intuitive
//! manipulation, and record keeping of university grades, sub-grades and tasks.

// Clippy lint groups {{{
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    //clippy::deprecated,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    //clippy::restriction,
    clippy::style,
    clippy::suspicious,
)]
#![allow(clippy::redundant_pub_crate)]
// }}}

mod ui;
mod uni_info;

use std::env;
use std::fs;
use std::io::{self, BufReader, Seek, Write};
use ui::UI;
use uni_info::UniInfo;

macro_rules! err_usage {
    () => {
        io::Error::new(io::ErrorKind::InvalidInput, "usage: `cargo run <filepath>`")
    };
}

/// Run the editing program, reading the file at command line arg 1.
/// Creates the file it it does not exist.
/// Saves the JSON data back to disk if no errors occur and the user confirms it.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        return Err(err_usage!());
    }
    let file_path: &str = args.get(1).ok_or_else(|| err_usage!())?;

    let mut file: fs::File = fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    if file.metadata().map_or(0, |md| md.len()) == 0 {
        file.write_all(b"[]")?;
        file.rewind()?;
    }

    let mut uni: UniInfo = serde_json::from_reader(BufReader::new(&file))?;

    let should_save: bool = UI::new(&mut uni)?.main_loop()?;

    if should_save {
        file.rewind()?;
        serde_json::to_writer_pretty(&file, &uni)?;
        let new_len: u64 = file.stream_position()?;
        file.set_len(new_len)?;
    }

    file.sync_all()?;
    Ok(())
}
