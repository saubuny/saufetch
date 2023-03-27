use clap::ValueEnum;
use colored::{ColoredString, Colorize};

#[derive(Debug, Clone, ValueEnum)]
pub enum AsciiArtSize {
    Large,
    Small,
}

#[cfg(target_os = "macos")]
pub fn get_ascii_art(size: AsciiArtSize) -> Vec<ColoredString> {
    match size {
        AsciiArtSize::Large => {
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
        AsciiArtSize::Small => vec![
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "        .:'".green(),
            "    __ :'__".green(),
            " .'`  `-'  ``.".yellow(),
            ":          .-'".yellow(),
            ":         :".red(),
            " :         `-;".magenta(),
            "  `.__.-.__.'".blue(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
        ],
    }
}

#[cfg(target_os = "windows")]
pub fn get_ascii_art(size: AsciiArtSize) -> Vec<&'static str> {
    vec!["skill issue"]
}

pub fn longest_str(vec: &[ColoredString]) -> usize {
    let mut len = 0;

    for line in vec.iter() {
        if line.len() > len {
            len = line.len();
        }
    }

    len
}
