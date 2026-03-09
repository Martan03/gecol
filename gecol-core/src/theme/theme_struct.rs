use std::fmt::Display;

use palette::{IntoColor, Oklch, Srgb};
use serde::{Deserialize, Serialize};

use crate::theme::Color;

/// Represents generated theme containing different colors.
///
/// It is used to generated a theme from the given color and then contain
/// the generated colors.
///
/// # Example
///
/// ```rust
/// use gecol_core::theme::Theme;
///
/// // Generates dark theme
/// let theme = Theme::dark((155, 155, 0));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,

    pub background: Color,
    pub surface: Color,
    pub border: Color,

    pub foreground: Color,
    pub muted: Color,

    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Theme {
    /// Generates dark theme based on the given color.
    ///
    /// Given color will be the primary color and the other colors will be
    /// generated based on that color.
    pub fn dark((r, g, b): (u8, u8, u8)) -> Self {
        let color: Oklch =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
                .into_color();

        let mut secondary = color;
        secondary.hue += 35.;

        let mut success = color;
        success.hue = 140.0.into();

        let mut warning = color;
        warning.hue = 100.0.into();

        let mut error = color;
        error.hue = 25.0.into();

        Self {
            primary: Color(color),
            secondary: Color(secondary),
            background: Color(Oklch::new(0.20, 0.006, color.hue)),
            surface: Color(Oklch::new(0.25, 0.008, color.hue)),
            border: Color(Oklch::new(0.32, 0.010, color.hue)),
            foreground: Color(Oklch::new(0.9, 0.01, color.hue)),
            muted: Color(Oklch::new(0.65, 0.02, color.hue)),
            success: Color(success),
            warning: Color(warning),
            error: Color(error),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Primary:    {}  \x1b[0m", self.primary)?;
        writeln!(f, "Secondary:  {}  \x1b[0m", self.secondary)?;
        writeln!(f, "Background: {}  \x1b[0m", self.background)?;
        writeln!(f, "Surface:    {}  \x1b[0m", self.surface)?;
        writeln!(f, "Border:     {}  \x1b[0m", self.border)?;
        writeln!(f, "Foreground: {}  \x1b[0m", self.foreground)?;
        writeln!(f, "Muted:      {}  \x1b[0m", self.muted)?;
        writeln!(f, "Success:    {}  \x1b[0m", self.success)?;
        writeln!(f, "Warning:    {}  \x1b[0m", self.warning)?;
        write!(f, "Error:      {}  \x1b[0m", self.error)
    }
}
