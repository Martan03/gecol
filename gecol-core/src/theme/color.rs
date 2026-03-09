use std::fmt::Display;

use palette::{IntoColor, OklabHue, Oklch, Srgb};
use serde::{Deserialize, Serialize};

/// Color representation using oklch color format.
///
/// It is used internally in the [`Theme`](crate::theme::Theme) for easier
/// color manipulation, such as lightening and darkening the color.
///
/// # Example
///
/// ```rust
/// use gecol_core::theme::Color;
///
/// let color = Color::from_rgb(67, 69, 42);
///
/// let lighter_color = color.lighten(0.2);
/// let darker_color = color.darken(0.3);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color(pub Oklch);

impl Color {
    /// Gets the lighness component of the oklch color.
    pub fn l(&self) -> f32 {
        self.0.l
    }

    /// Gets the chroma component of the oklch color.
    pub fn chroma(&self) -> f32 {
        self.0.chroma
    }

    /// Gets the hue component of the oklch color.
    pub fn hue(&self) -> OklabHue<f32> {
        self.0.hue
    }

    /// Converts given RGB components to the [`Color`] represented using
    /// [`Oklch`](palette::Oklch) color format
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let srgb =
            Srgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        Self(srgb.into_color())
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let value: Srgb = self.0.into_color();
        (
            f32_to_u8(value.red),
            f32_to_u8(value.green),
            f32_to_u8(value.blue),
        )
    }

    /// Lightens the color by adding a value to it.
    ///
    /// It adds `amount` (e.g. 0.1 for a 10% shift) to the current lightness
    /// channel.
    pub fn lighten(&self, amount: f32) -> Self {
        let mut color = self.0;
        let old_w_dist = 1. - color.l;

        color.l = (color.l + amount).clamp(0., 1.);
        let new_w_dist = 1. - color.l;

        if old_w_dist > 0. {
            color.chroma *= new_w_dist / old_w_dist;
        }
        Self(color)
    }

    /// Brightens the color relatively by a given multiplier.
    ///
    /// It multiplies the current lightness by 1 + `amount` percent (e.g. 0.2
    /// for a 20% brighter color)
    pub fn brighten(&self, amount: f32) -> Self {
        let color = self.0;
        let target = (color.l * (1.0 + amount)).clamp(0., 1.0);
        self.lighten(target - color.l)
    }

    /// Darkens the color by adding a value to it.
    ///
    /// It subtracts `amount` (e.g. 0.1 for a 10% shift) from the current
    /// lightness channel.
    pub fn darken(&self, amount: f32) -> Self {
        let mut color = self.0;
        let old_l = color.l;

        color.l = (color.l - amount).clamp(0., 1.);
        if old_l > 0.0 {
            color.chroma *= color.l / old_l;
        }
        Self(color)
    }

    /// Dims the color relatively by a given multiplier.
    ///
    /// It reduces the current lightness by 1 - `amount` percent (e.g. 0.2
    /// for a 20% dimmer color)
    pub fn dim(&self, amount: f32) -> Self {
        if amount < 0. {
            return self.brighten(-amount);
        }

        let mut color = self.0;
        let factor = (1. - amount).max(0.);

        color.l = (color.l * factor).clamp(0., 1.);
        color.chroma = (color.chroma * factor).max(0.);
        Self(color)
    }

    /// Increases the saturation (chroma component) of the color.
    pub fn saturate(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.chroma = (color.chroma + amount).max(0.);
        Self(color)
    }

    /// Descreases the saturation (chroma component) of the color.
    pub fn desaturate(&self, amount: f32) -> Self {
        let mut color = self.0;
        color.chroma = (color.chroma - amount).max(0.);
        Self(color)
    }
}

fn f32_to_u8(value: f32) -> u8 {
    (value.clamp(0., 1.) * 255.).round() as u8
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.to_rgb();
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
        let color = Color(Oklch::new(0.4, 0.6, 0.9)).dim(-3.0);
        assert_eq!(color.0.l, 1.);
        let color = color.dim(0.5);
        assert_eq!(color.0.l, 0.5);
        let color = color.dim(1.5);
        assert_eq!(color.0.l, 0.0);
    }
}
