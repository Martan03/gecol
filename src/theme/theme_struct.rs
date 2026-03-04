use std::fmt::Display;

use palette::{IntoColor, Oklch, Srgb};

use crate::theme::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub surface: Color,
    pub foreground: Color,
}

impl Theme {
    pub fn dark((r, g, b): (u8, u8, u8)) -> Self {
        let color: Oklch =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
                .into_color();

        let mut secondary = color;
        secondary.hue += 35.0;

        Self {
            primary: Color(color),
            secondary: Color(secondary),
            background: Color(Oklch::new(0.15, 0.02, color.hue)),
            surface: Color(Oklch::new(0.2, 0.02, color.hue)),
            foreground: Color(Oklch::new(0.9, 0.01, color.hue)),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Primary:    {}  \x1b[0m", self.primary)?;
        writeln!(f, "Secondary:  {}  \x1b[0m", self.secondary)?;
        writeln!(f, "Background: {}  \x1b[0m", self.background)?;
        writeln!(f, "Surface:    {}  \x1b[0m", self.surface)?;
        write!(f, "Foreground: {}  \x1b[0m", self.foreground)
    }
}
