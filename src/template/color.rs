use crate::theme::Color;

impl Color {
    pub fn rgb(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("{r},{g},{b}")
    }

    pub fn hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}
