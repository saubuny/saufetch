use ascii_art::{get_ascii_art, longest_str};
use clap::Parser;
use colored::Colorize;
use sysinfo::{System, SystemExt, UserExt};

mod ascii_art;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(default_value_t = String::from(""))]
    ascii_art_name: String,
}

fn main() {
    let _args = Cli::parse();

    // Eventually change this to just the things we need, and not everything
    let mut sys = System::new_all();

    sys.refresh_all();

    // All art should be 16 lines long
    let _ascii_art_len = 16;

    let ascii_art = get_ascii_art();
    let width = longest_str(&ascii_art) + 5;

    println!();

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

    println!();
    println!();
}
