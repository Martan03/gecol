mod color;
mod theme_struct;

use std::str::FromStr;

pub use color::Color;
use serde::{Deserialize, Serialize};
pub use theme_struct::Theme;

/// Represents the theme type.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default,
)]
pub enum ThemeType {
    #[default]
    Dark,
    Light,
}

impl FromStr for ThemeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(ThemeType::Dark),
            "light" => Ok(ThemeType::Light),
            _ => Err(format!("Invalid theme type.")),
        }
    }
}
