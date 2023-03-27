use ascii_art::{get_ascii_art, longest_str, AsciiArtSize};
use clap::Parser;
use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt, UserExt};

pub mod ascii_art;

// TODO: Have less info shown with small option
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum, default_value_t = AsciiArtSize::Large)]
    ascii_art_size: AsciiArtSize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // TODO: Change this to just the things we need, and not everything
    let mut sys = System::new_all();

    sys.refresh_all();

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
        sys.long_os_version().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[3],
        "Kernel:".yellow().bold(),
        sys.kernel_version().unwrap()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[4],
        "Uptime:".yellow().bold(),
        sys.uptime()
    );

    println!(
        "{:<width$}{} {}",
        ascii_art[5],
        "CPU:".yellow().bold(),
        sys.global_cpu_info().brand()
    );

    for i in 6..ascii_art.len() {
        println!(
            "{:<width$}{} {}",
            ascii_art[i],
            "TODO:".yellow().bold(),
            "TODO"
        );
    }

    println!();
    println!();

    Ok(())
}
