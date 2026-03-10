mod color;
mod theme_struct;

pub use color::Color;
use serde::{Deserialize, Serialize};
pub use theme_struct::Theme;

/// Represents the theme type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeType {
    #[default]
    Dark,
    Light,
}
