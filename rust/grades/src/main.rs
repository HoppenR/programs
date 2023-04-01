mod ui;
mod uni_info;

use std::env;
use std::fs;
use std::io::Error;
use uni_info::UniInfo;

fn main() -> Result<(), Error> {
    // TODO[?]: Add keypress for toggling show_all/show_noncomplete
    //          Add bool to UniInfo
    //          Tell SerDe to ignore it
    //          Make a keypress that toggles it in UI/Key
    //          Make an impl that changes the bool UniInfo
    //          Use it in the Display for UniInfo to show all tasks
    //
    // TODO: Some way to add new entries
    //       Targeting an entry and pressing ButtonAdd means to add something inside of it
    //       An exception would be targetting a semester, because it makes no sense to add a period
    //       and there would be no way to target the "menu" entry to create a semester
    //       therefore adding on a semester entry creates a new semester

    // -- ARGS --
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args.get(1).ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: `cargo run <filepath>`",
        )
    })?;

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
