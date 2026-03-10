use std::fmt::Display;

use palette::{IntoColor, Oklch, Srgb};
use serde::{Deserialize, Serialize};

use crate::theme::{Color, ThemeType};

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
    /// Generates the theme based on the given color..
    ///
    /// It generates based on the given theme type. The given color will be
    /// the primary color and the other colors willbe generated based on that
    /// color.
    pub fn generate(theme_type: ThemeType, rgb: (u8, u8, u8)) -> Self {
        match theme_type {
            ThemeType::Dark => Self::dark(rgb),
            ThemeType::Light => Self::light(rgb),
        }
    }

    /// Generates dark theme based on the given color.
    ///
    /// Given color will be the primary color and the other colors will be
    /// generated based on that color.
    pub fn dark((r, g, b): (u8, u8, u8)) -> Self {
        let color: Oklch =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
                .into_color();

        let (secondary, success, warning, error) = Self::shared_colors(color);
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

    /// Generates light theme based on the given color.
    ///
    /// Given color will be the primary color and the other colors will be
    /// generated based on that color. It restrains the maximum primary color
    /// lightness in order for it to be visible.
    pub fn light((r, g, b): (u8, u8, u8)) -> Self {
        let mut color: Oklch =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
                .into_color();

        color.l = color.l.min(0.70);
        let (secondary, success, warning, error) = Self::shared_colors(color);
        Self {
            primary: Color(color),
            secondary: Color(secondary),
            background: Color(Oklch::new(0.98, 0.004, color.hue)),
            surface: Color(Oklch::new(0.94, 0.006, color.hue)),
            border: Color(Oklch::new(0.85, 0.010, color.hue)),
            foreground: Color(Oklch::new(0.15, 0.01, color.hue)),
            muted: Color(Oklch::new(0.45, 0.02, color.hue)),
            success: Color(success),
            warning: Color(warning),
            error: Color(error),
        }
    }

    fn shared_colors(color: Oklch) -> (Oklch, Oklch, Oklch, Oklch) {
        let mut secondary = color;
        secondary.hue += 35.;

        let mut success = color;
        success.hue = 140.0.into();

        let mut warning = color;
        warning.hue = 100.0.into();

        let mut error = color;
        error.hue = 25.0.into();

        (secondary, success, warning, error)
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
