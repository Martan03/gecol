use std::path::Path;

use image::{DynamicImage, RgbImage, imageops::FilterType};
use imgref::Img;
use mss_saliency::maximum_symmetric_surround_saliency;
use palette::{FromColor, Hsv, Srgb};

use crate::{config::Config, error::Error};

#[derive(Debug, Clone)]
pub struct Extractor<'a> {
    config: &'a Config,
    width: usize,
    height: usize,
}

impl<'a> Extractor<'a> {
    /// Extracts the accent color from image on the given path.
    ///
    /// `path` is any type convertible into `PathBuf`.
    pub fn extract<P>(
        path: P,
        config: &'a Config,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
    {
        let mut extractor = Self {
            config,
            width: 0,
            height: 0,
        };
        let img = image::open(path)?;
        let img = extractor.prep_img(img);

        let (sal_map, is_sal_worth) = extractor.gen_salience(&img);
        let rgb_img = img.to_rgb8();

        Ok(extractor.find_best_col(&rgb_img, &sal_map, is_sal_worth))
    }

    /// Resizes the image only if new dimensions are provided.
    fn prep_img(&mut self, img: DynamicImage) -> DynamicImage {
        let tw = self.config.resize_width.unwrap_or(img.width());
        let th = self.config.resize_height.unwrap_or(img.height());
        self.width = tw as usize;
        self.height = th as usize;

        if tw == img.width() && th == img.height() {
            return img;
        }
        img.resize_exact(tw, th, FilterType::Nearest)
    }

    /// Generates the normalized u8 saliency map
    fn gen_salience(&self, img: &DynamicImage) -> (Vec<u8>, bool) {
        let luma = img.to_luma8();
        let luma_img =
            Img::new(luma.as_raw().as_slice(), self.width, self.height);
        let sal_map = maximum_symmetric_surround_saliency(luma_img);

        let max_sal = *sal_map.buf().iter().max().unwrap_or(&1);
        let sal_map: Vec<u8> = sal_map
            .buf()
            .iter()
            .map(|&v| ((v as f32 / max_sal as f32) * 255.) as u8)
            .collect();

        let total_sal: u32 = sal_map.iter().map(|&p| p as u32).sum();
        let avg_sal = total_sal as f32 / (self.width * self.height) as f32;
        let is_sal_worth = avg_sal >= self.config.sal_threshold;

        (sal_map, is_sal_worth)
    }

    /// Finds the best color based on saliency and HSV.
    fn find_best_col(
        &self,
        rgb_img: &RgbImage,
        sal_map: &[u8],
        is_worth: bool,
    ) -> Option<(u8, u8, u8)> {
        let mut best_col = None;
        let mut max_score = 0.0;

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.;
            let g = pixel[1] as f32 / 255.;
            let b = pixel[2] as f32 / 255.;

            let hsv = Hsv::from_color(Srgb::new(r, g, b));
            if hsv.value < self.config.val_threshold
                || hsv.saturation < self.config.sat_threshold
            {
                continue;
            }

            let mut score = hsv.saturation * hsv.value;
            if is_worth {
                let id = y as usize * self.width + x as usize;
                let sal_val = sal_map[id] as f32 / 255.;
                score *= 1.0 + (sal_val * 5.0);
            }

            if score > max_score {
                max_score = score;
                best_col = Some((pixel[0], pixel[1], pixel[2]));
            }
        }
        best_col
    }
}
