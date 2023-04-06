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
use std::io;
use ui::UI;
use uni_info::UniInfo;

macro_rules! err_usage {
    () => {
        io::Error::new(io::ErrorKind::InvalidInput, "usage: `cargo run <filepath>`")
    };
}

// TODO: Dont prompt for saving if no changes?

/// Run the editing program, reading the file at command line arg 1.
/// Saves the JSON data back to disk if no errors occur.
fn main() -> io::Result<()> {
    // -- ARGS --
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args.get(1).ok_or_else(|| err_usage!())?;

    // -- LOAD --
    let load_json_data: String = fs::read_to_string(file_path)?;
    let mut uni: UniInfo = serde_json::from_str(&load_json_data)?;

    // --- UI ---
    let ui = UI::new(&mut uni)?;
    let should_save: bool = ui.main_loop()?;

    // -- SAVE --
    if should_save {
        let save_json_data: String = serde_json::to_string_pretty(&uni)?;
        fs::write(file_path, save_json_data)?;
    }

    Ok(())
}