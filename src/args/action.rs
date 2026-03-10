use std::path::PathBuf;

use clap::{Parser, Subcommand};
use gecol_core::theme::ThemeType;

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum Action {
    Run(Run),
    Extract(Extract),
    Build(Build),
    Preview(Preview),
    List,
    Config,
    ClearCache,
}

#[derive(Debug, Clone, PartialEq, Default, Parser)]
pub struct Run {
    pub img: PathBuf,
    #[arg(short, long = "template")]
    pub templates: Vec<String>,
    #[arg(short = 'T', long)]
    pub theme: Option<ThemeType>,
}

#[derive(Debug, Clone, PartialEq, Default, Parser)]
pub struct Extract {
    pub img: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Default, Parser)]
pub struct Build {
    #[arg(value_parser = parse_hex_col)]
    pub color: (u8, u8, u8),
    #[arg(short, long = "template")]
    pub templates: Vec<String>,
    #[arg(short = 'T', long)]
    pub theme: Option<ThemeType>,
}

#[derive(Debug, Clone, PartialEq, Default, Parser)]
pub struct Preview {
    /// This can be either a hex code or an image path.
    pub target: String,
    #[arg(short = 'T', long)]
    pub theme: Option<ThemeType>,
}

pub fn parse_hex_col(val: &str) -> Result<(u8, u8, u8), String> {
    let hex = val.trim_start_matches('#');
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Ok((r, g, b));
        }
    }

    Err(format!(
        "Invalid color '{}'. Expected format is '#rrggbb'.",
        val
    ))
}
