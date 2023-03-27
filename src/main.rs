use ascii_art::{get_ascii_art, longest_str, AsciiArtSize};
use clap::Parser;
use colored::Colorize;
use libmacchina::traits::{GeneralReadout as _, MemoryReadout as _, PackageReadout as _};
use libmacchina::{GeneralReadout, MemoryReadout, PackageReadout};

pub mod ascii_art;

// TODO: Have less info shown with small option
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum, default_value_t = AsciiArtSize::Large)]
    ascii_art_size: AsciiArtSize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let general_readout = GeneralReadout::new();
    let memory_readout = MemoryReadout::new();
    let package_readout = PackageReadout::new();

    // NOTE: All large art should be 16 lines long
    let ascii_art = get_ascii_art(args.ascii_art_size);
    let width = longest_str(&ascii_art) + 5;

    // TODO: Do this better than just a bunch of printlns

    println!();

    // Colored strings add invisible length to the string, but we need
    // the actual length later
    let name1 = general_readout.username().unwrap();
    let name2 = general_readout.hostname().unwrap();
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
        "Host:".yellow().bold(),
        general_readout.machine().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[4],
        "Uptime:".yellow().bold(),
        general_readout.uptime().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[5],
        "Terminal:".yellow().bold(),
        general_readout.terminal().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[6],
        "Resolution:".yellow().bold(),
        general_readout.resolution().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[7],
        "DE:".yellow().bold(),
        general_readout.desktop_environment().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[8],
        "WM:".yellow().bold(),
        general_readout.window_manager().unwrap()
    );

    println!(
        "{:<width$}{} {}%",
        ascii_art[9],
        "Brightness:".yellow().bold(),
        general_readout.backlight().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[10],
        "CPU:".yellow().bold(),
        general_readout.cpu_model_name().unwrap()
    );

    println!(
        "{:<width$}{} {}%",
        ascii_art[11],
        "CPU Load:".yellow().bold(),
        general_readout.cpu_usage().unwrap()
    );

    // Ew
    let mem1 = memory_readout.used().unwrap() as f32 / 100000_f32;
    let mem2 = memory_readout.total().unwrap() as f32 / 100000_f32;

    println!(
        "{:<width$}{} {} GB/{} GB",
        ascii_art[12],
        "Memory:".yellow().bold(),
        mem1.round() / 10_f32,
        mem2.round() / 10_f32
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[13],
        "Packages:".yellow().bold(),
        package_readout
            .count_pkgs()
            .iter()
            .map(|n| format!("{} ({})", n.1, n.0.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    );

    for item in ascii_art.iter().skip(14) {
        println!("{:<width$}", item,);
    }

    println!();
    println!();

    Ok(())
}
