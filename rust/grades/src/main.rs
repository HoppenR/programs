mod ui;
mod uni_info;

use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use ui::UI;
use uni_info::UniInfo;

macro_rules! err_usage {
    () => {
        Error::new(ErrorKind::InvalidInput, "usage: `cargo run <filepath>`")
    };
}

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
// TODO[?]: Function up_or_exit for after deleting something?

fn main() -> Result<(), Error> {
    // -- ARGS --
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args.get(1).ok_or_else(|| err_usage!())?;

    // -- LOAD --
    let json_data: String = fs::read_to_string(file_path)?;
    let mut uni: UniInfo = serde_json::from_str(&json_data)?;

    // --- UI ---
    let mut ui = UI::new(&mut uni)?;
    ui.main_loop()?;
    drop(ui);

    // -- SAVE --
    let json_data: String = serde_json::to_string_pretty(&uni)?;
    fs::write(file_path, json_data)?;

    Ok(())
}
