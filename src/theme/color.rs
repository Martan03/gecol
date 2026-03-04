use std::fmt::Display;

use palette::{IntoColor, Oklch, Srgb};

/// Color representation using oklch color format.
///
/// It is used internally in the [`Theme`](crate::theme::Theme) for easier
/// color manipulation, such as lightening and darkening the color.
///
/// # Example
/// ```rust
/// use geocol::theme::Color;
///
/// let color = Color::from_rgb(67, 69, 42);
///
/// let lighter_color = color.lighten(0.2);
/// let darker_color = color.darken(0.3);
/// ```
#[derive(Debug, Clone)]
pub struct Color(pub Oklch);

impl Color {
    /// Converts given RGB components to the [`Color`] represented using
    /// [`Oklch`](palette::Oklch) color format
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let srgb =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        Self(srgb.into_color())
    }

    /// Lightens the color by adding a value to it.
    ///
    /// It adds `amount` (e.g. 0.1 for a 10% shift) to the current lightness
    /// channel.
    pub fn lighten(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.l = (color.l + amount).clamp(0., 1.);
        Self(color)
    }

    /// Brightens the color relatively by a given multiplier.
    ///
    /// It multiplies the current lightness by 1 + `amount` percent (e.g. 0.2
    /// for a 20% brighter color)
    pub fn brighten(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.l = (color.l * (1.0 + amount)).clamp(0., 1.0);
        Self(color)
    }

    /// Darkens the color by adding a value to it.
    ///
    /// It subtracts `amount` (e.g. 0.1 for a 10% shift) from the current
    /// lightness channel.
    pub fn darken(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.l = (color.l - amount).clamp(0., 1.);
        Self(color)
    }

    /// Dims the color relatively by a given multiplier.
    ///
    /// It reduces the current lightness by 1 - `amount` percent (e.g. 0.2
    /// for a 20% dimmer color)
    pub fn dim(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.l = (color.l * (1.0 - amount)).clamp(0., 1.0);
        Self(color)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let srgb: Srgb = self.0.into_color();

        let r = (srgb.red.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (srgb.green.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (srgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
        write!(f, "\x1b[48;2;{r};{g};{b}m",)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use palette::Oklch;

    use crate::theme::Color;

    #[test]
    fn lighten() {
        let color = Color(Oklch::new(0.4, 0.6, 0.9)).lighten(0.2);
        assert_eq!(color.0.l, 0.6);
        let color = color.lighten(0.5);
        assert_eq!(color.0.l, 1.0);
        let color = color.lighten(-1.5);
        assert_eq!(color.0.l, 0.);
    }

    #[test]
    fn brighten() {
        let color = Color(Oklch::new(0.4, 0.6, 0.9)).brighten(0.5);
        assert_eq!(color.0.l, 0.6);
        let color = color.brighten(0.8);
        assert_eq!(color.0.l, 1.0);
        let color = color.brighten(-1.5);
        assert_eq!(color.0.l, 0.);
    }

    #[test]
    fn darken() {
        let color = Color(Oklch::new(0.4, 0.6, 0.9)).darken(0.2);
        assert_eq!(color.0.l, 0.2);
        let color = color.darken(0.5);
        assert_eq!(color.0.l, 0.0);
        let color = color.darken(-1.5);
        assert_eq!(color.0.l, 1.);
    }

    #[test]
    fn dim() {
        let color = Color(Oklch::new(0.4, 0.6, 0.9)).dim(-2.0);
        assert_eq!(color.0.l, 1.);
        let color = color.dim(0.5);
        assert_eq!(color.0.l, 0.5);
        let color = color.dim(1.5);
        assert_eq!(color.0.l, 0.0);
    }
}
