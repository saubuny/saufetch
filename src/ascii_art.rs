use colored::{ColoredString, Colorize};
use std::collections::HashMap;

// Update this to be able to choose based on cli input
#[cfg(target_os = "macos")]
pub fn get_ascii_art() -> Vec<ColoredString> {
    vec![
        "                    'c.".green(),
        "                 ,MMMM.".green(),
        "               .MMMMMM".green(),
        "               MMMMM,".green(),
        "     .;MMMMM:' MMMMMMMMMM;.".yellow(),
        "   MMMMMMMMMMMMNWMMMMMMMMMMM:".yellow(),
        " .MMMMMMMMMMMMMMMMMMMMMMMMWM.".yellow(),
        " MMMMMMMMMMMMMMMMMMMMMMMMM.".red(),
        ";MMMMMMMMMMMMMMMMMMMMMMMM:".red(),
        ":MMMMMMMMMMMMMMMMMMMMMMMM:".red(),
        ".MMMMMMMMMMMMMMMMMMMMMMMMM.".magenta(),
        " MMMMMMMMMMMMMMMMMMMMMMMMMMM.".magenta(),
        "  .MMMMMMMMMMMMMMMMMMMMMMMMMM.".magenta(),
        "    MMMMMMMMMMMMMMMMMMMMMMMM".blue(),
        "     ;MMMMMMMMMMMMMMMMMMMM.".blue(),
        "       .MMMM,.    .MMMM,.".blue(),
    ]
}

#[cfg(target_os = "windows")]
pub fn get_ascii_art() -> Vec<&'static str> {
    vec!["skill issue"]
}

pub fn longest_str(vec: &Vec<ColoredString>) -> usize {
    let mut len = 0;

    for line in vec.iter() {
        if line.len() > len {
            len = line.len();
        }
    }

    len
}
