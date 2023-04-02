mod ui;
mod uni_info;

use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use uni_info::UniInfo;

fn main() -> Result<(), Error> {
    // TODO[?]: Add keypress for toggling show_all/show_noncomplete
    //          - Add bool to UniInfo
    //          - Tell SerDe to ignore it
    //          - Make a keypress that toggles it in UI/Key
    //          - Make an impl that changes the bool UniInfo
    //          - Use it in the Display for UniInfo to show all tasks
    //
    // TODO[ ]: Better movement / handling when deleting the last of something
    //          also update cursor position
    //
    // TODO[ ]: Only format/print the area of uni_info around the cursor.
    //          Maybe use some sort of variable to track offset?

    // -- ARGS --
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args
        .get(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "usage: `cargo run <filepath>`"))?;

    // -- DATA --
    let json_data: String = fs::read_to_string(file_path)?;
    let mut uni: UniInfo = serde_json::from_str(&json_data)?;

    // --- UI ---
    let mut old_termios: libc::termios = ui::init()?;
    let ret: Result<(), Error> = ui::ui_loop(&mut uni);
    if ret.is_ok() {
        let json_data: String = serde_json::to_string_pretty(&uni)?;
        fs::write(file_path, json_data)?;
    }
    ui::exit(&mut old_termios)?;
    ret
}
