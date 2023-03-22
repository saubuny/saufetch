use colored::{ColoredString, Colorize};
use sysinfo::{System, SystemExt, UserExt};

fn longest_str(vec: &Vec<ColoredString>) -> usize {
    let mut len = 0;

    for line in vec.iter() {
        if line.len() > len {
            len = line.len();
        }
    }

    len
}

fn main() {
    // Eventually change this to just the things we need, and not everything
    let mut sys = System::new_all();

    sys.refresh_all();

    let ascii_art = get_ascii_art();
    let width = longest_str(&ascii_art) + 5;

    print!("\n");

    let name = format!(
        "{}@{}",
        sys.users().get(2).unwrap().name().green().bold(),
        sys.host_name().unwrap().green().bold()
    );

    println!("{:<width$}{}", ascii_art[0], name);

    println!("{:<width$}{}", ascii_art[1], "-".repeat(name.len()));

    println!(
        "{:<width$}{:<10}{:>10}",
        ascii_art[2],
        "OS:".yellow().bold(),
        sys.long_os_version().unwrap()
    );

    println!(
        "{:<width$}{:<10}{:>10}",
        ascii_art[3],
        "Kernel:".yellow().bold(),
        sys.kernel_version().unwrap()
    );

    for i in 4..ascii_art.len() {
        println!(
            "{:<width$}{:<10}{:>10}",
            ascii_art[i],
            "TODO:".yellow().bold(),
            "TODO"
        );
    }

    print!("\n\n");
}

#[cfg(target_os = "macos")]
fn get_ascii_art() -> Vec<ColoredString> {
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
fn get_ascii_art() -> Vec<&'static str> {
    vec!["skill issue"]
}
