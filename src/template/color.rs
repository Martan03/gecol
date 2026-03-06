use minijinja::{Value, value::Object};

use crate::theme::Color;

impl Object for Color {
    fn get_value(
        self: &std::sync::Arc<Self>,
        key: &minijinja::Value,
    ) -> Option<minijinja::Value> {
        match key.as_str()? {
            "hex" => Some(self.hex().into()),
            "rgb" => Some(self.rgb().into()),
            "strip" => Some(self.strip().into()),
            "r" => Some(self.to_rgb().0.into()),
            "g" => Some(self.to_rgb().1.into()),
            "b" => Some(self.to_rgb().2.into()),
            _ => None,
        }
    }

    fn call_method(
        self: &std::sync::Arc<Self>,
        _state: &minijinja::State<'_, '_>,
        method: &str,
        args: &[minijinja::Value],
    ) -> Result<minijinja::Value, minijinja::Error> {
        let amount = args
            .first()
            .and_then(|v| f32::try_from(v.clone()).ok())
            .unwrap_or(0.0);

        match method {
            "hexa" => Ok(Value::from(self.hexa(amount))),
            "rgba" => Ok(Value::from(self.rgba(amount))),

            "lighten" => Ok(Value::from_object(self.lighten(amount))),
            "brighten" => Ok(Value::from_object(self.brighten(amount))),
            "darken" => Ok(Value::from_object(self.darken(amount))),
            "dim" => Ok(Value::from_object(self.dim(amount))),

            "saturate" => Ok(Value::from_object(self.saturate(amount))),
            "desaturate" => Ok(Value::from_object(self.desaturate(amount))),

            _ => Err(minijinja::Error::from(
                minijinja::ErrorKind::UnknownMethod,
            )),
        }
    }

    fn render(
        self: &std::sync::Arc<Self>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        Self: Sized + 'static,
    {
        write!(f, "{}", self.hex())
    }
}

impl Color {
    /// Converts the given color to the rgb string in format `r,g,b`.
    pub fn rgb(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("{r},{g},{b}")
    }

    /// Converts the given color to the rgba string in format `r,g,b,a`, where
    /// `a` is the given `alpha` value.
    ///
    /// `alpha` is value in the range of 0 to 1.
    pub fn rgba(&self, alpha: f32) -> String {
        format!("{},{}", self.rgb(), alpha.clamp(0., 1.))
    }

    /// Converts the given color to the hex string in format `#rrggbb`.
    pub fn hex(&self) -> String {
        format!("#{}", self.strip())
    }

    /// Converts the given color to the hexa string in format `#rrggbbaa`,
    /// where `aa` is the given `alpha` in hex.
    ///
    /// `alpha` is value in the range of 0 to 1.
    pub fn hexa(&self, alpha: f32) -> String {
        let alpha = (alpha.clamp(0., 1.) * 255.).round() as u8;
        format!("#{}{:02x}", self.strip(), alpha)
    }

    /// Converts the given color to the hex string in format `rrggbb`.
    ///
    /// This compared [`Color::hex`] doesn't have the leading `#` character.
    pub fn strip(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("{r:02x}{g:02x}{b:02x}")
    }
}
