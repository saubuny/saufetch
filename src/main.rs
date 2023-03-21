use colored::{ColoredString, Colorize};
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    let ascii_art = get_ascii_art();

    println!();

    println!("{:<40}{}", ascii_art[0], sys.host_name().unwrap());
    println!(
        "{:<35}OS:    {:>6}",
        ascii_art[1],
        sys.long_os_version().unwrap()
    );

    for i in 2..ascii_art.len() {
        println!("{:<35}TMP:    {:>6}", ascii_art[i], "temp");
    }

    println!();
}

// Append output to each line of this
#[cfg(target_os = "macos")]
fn get_ascii_art() -> Vec<ColoredString> {
    vec![
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
fn get_ascii_art() -> Vec<&'static str> {
    vec!["skill issue"]
}
