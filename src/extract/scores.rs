use image::Rgb;
use palette::Lab;

/// Represents a scored pixel extracted from the image.
#[derive(Debug, Clone)]
pub(crate) struct ScoredPixel {
    pub lab: Lab,
    pub rgb: (u8, u8, u8),
    pub score: f32,
}

impl ScoredPixel {
    pub fn new(lab: Lab, rgb: &Rgb<u8>, score: f32) -> Self {
        Self {
            lab,
            rgb: (rgb[0], rgb[1], rgb[2]),
            score,
        }
    }
}

/// Represents a scored cluster.
#[derive(Debug, Clone)]
pub(crate) struct ScoredCluster {
    pub cnt: usize,
    pub score: f32,
    pub max_score: f32,
    pub best_rgb: (u8, u8, u8),
}

impl ScoredCluster {
    pub fn push(&mut self, pixel: &ScoredPixel) {
        self.cnt += 1;
        self.score += pixel.score;

        if pixel.score > self.max_score {
            self.max_score = pixel.score;
            self.best_rgb = pixel.rgb;
        }
    }
}

impl Default for ScoredCluster {
    fn default() -> Self {
        Self {
            cnt: Default::default(),
            score: Default::default(),
            max_score: -1.,
            best_rgb: Default::default(),
        }
    }
}
