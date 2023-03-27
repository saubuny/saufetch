use ascii_art::{get_ascii_art, longest_str, AsciiArtSize};
use clap::Parser;
use colored::Colorize;
use libmacchina::traits::GeneralReadout as _;
use libmacchina::GeneralReadout;
use sysinfo::{CpuExt, System, SystemExt, UserExt};

pub mod ascii_art;

// big TODO: Switch to Libmacchina

// TODO: Have less info shown with small option
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum, default_value_t = AsciiArtSize::Large)]
    ascii_art_size: AsciiArtSize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    let general_readout = GeneralReadout::new();

    // NOTE: All large art should be 16 lines long
    let ascii_art = get_ascii_art(args.ascii_art_size);
    let width = longest_str(&ascii_art) + 5;

    println!();

    // Colored strings add invisible length to the string, but we need
    // the actual length later
    let name1 = sys.users().get(2).unwrap().name();
    let name2 = sys.host_name().unwrap();
    let name = format!("{}@{}", name1.green().bold(), name2.green().bold());

    println!("{:<width$}{}", ascii_art[0], name);

    println!(
        "{:<width$}{}",
        ascii_art[1],
        "-".repeat(name1.len() + name2.len() + 1)
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[2],
        "OS:".yellow().bold(),
        general_readout.os_name().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[3],
        "Uptime:".yellow().bold(),
        general_readout.uptime().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[4],
        "Terminal:".yellow().bold(),
        general_readout.terminal().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[5],
        "Resolution:".yellow().bold(),
        general_readout.resolution().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[6],
        "DE:".yellow().bold(),
        general_readout.desktop_environment().unwrap()
    );

    for item in ascii_art.iter().skip(7) {
        println!(
            "{:<width$}{} {}",
            item,
            "CPU:".yellow().bold(),
            general_readout.cpu_model_name().unwrap()
        );
    }

    println!();
    println!();

    Ok(())
}
