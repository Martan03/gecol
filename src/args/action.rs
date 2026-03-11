use clap::{Parser, Subcommand};
use gecol_core::theme::ThemeType;

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum Action {
    Run(Run),
    List,
    Config,
    ClearCache,
}

#[derive(Debug, Clone, PartialEq, Default, Parser)]
pub struct Run {
    /// The target image path or a hex color (e.g. "#3acbaf")
    pub target: String,

    /// Build only specific template. Can be used multiplie times.
    #[arg(short, long = "template")]
    pub templates: Vec<String>,
    /// Override the theme type for this run
    #[arg(short = 'T', long)]
    pub theme: Option<ThemeType>,

    /// Skip building the templates - prints theme in the terminal.
    #[arg(long, conflicts_with_all = ["templates", "extract_only"])]
    pub skip_build: bool,
    /// Only extract color from image
    #[arg(long, conflicts_with_all = ["skip_build", "templates", "theme"])]
    pub extract_only: bool,
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
