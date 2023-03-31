mod ui;
mod uni_info;

use std::env;
use std::fs;
use std::io::Error;
use uni_info::UniInfo;

fn main() -> Result<(), Error> {
    // TODO: Better args parsing, maybe --all should disable hiding
    //       moments of non-ongoing courses
    // TODO: Ability to edit the JSON through the program
    // TODO: Add type CompletedExpanded, GradeExpanded to be able to expand non-visible fields,
    //       these would save as their non-expanded counterparts

    // -- ARGS --
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args.get(1).ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: `cargo run <filepath>`",
        )
    })?;

    // -- DATA --
    let json_data = fs::read_to_string(file_path)?;
    let mut uni: UniInfo = serde_json::from_str(&json_data)?;

    // --- UI ---
    let mut old_termios: libc::termios = ui::init()?;
    let ret: Result<(), Error> = ui::ui_loop(&mut uni);
    ui::exit(&mut old_termios)?;
    ret
}
