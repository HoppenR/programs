mod term;
mod ui;
mod uni_info;
use term::*;
use ui::*;
use uni_info::*;

use libc;
use serde_json;
use std::env;
use std::fs;
use std::io::Error;

fn indent(indent_level: usize) -> String {
    return INDENT.repeat(indent_level);
}

fn main() -> Result<(), Error> {
    // TODO: Better args parsing, maybe --all should disable hiding
    //       moments of non-ongoing courses
    // TODO: Ability to edit the JSON through the program
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args.get(1).ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: `cargo run <filepath>`",
        )
    })?;
    let json_data = fs::read_to_string(file_path)?;
    let mut uni_info: UniInfo = serde_json::from_str(&json_data)?;
    // --- UI ---
    let mut old_termios: libc::termios = ui_init()?;
    ui_loop(&mut uni_info)?;
    ui_exit(&mut old_termios)?;
    Ok(())
}
