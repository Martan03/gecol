use std::path::Path;

use image::{DynamicImage, RgbImage, imageops::FilterType};
use imgref::Img;
use kmeans_colors::get_kmeans;
use mss_saliency::maximum_symmetric_surround_saliency;
use palette::{FromColor, Hsv, IntoColor, Lab, Srgb};

use crate::{
    Cache,
    config::Config,
    error::Error,
    extract::scores::{ScoredCluster, ScoredPixel},
};

/// Struct for extracting a color from an image.
///
/// This struct is not meant for storing, but it only stores its state while
/// extracting the color.
#[derive(Debug, Clone)]
pub struct Extractor<'a> {
    config: &'a Config,
    width: usize,
    height: usize,
}

impl<'a> Extractor<'a> {
    /// Extracts the accent color from image on the given path.
    ///
    /// When no sufficient color is found, it returns `None`.
    pub fn extract<P>(
        path: P,
        config: &'a Config,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
    {
        let cache_file =
            config.cache_dir.to_owned().unwrap_or_else(Config::file);
        let mut cache = Cache::load(&cache_file);
        let key = Cache::key(config, path.as_ref())
            .unwrap_or("fallback".to_string());

        if let Some(&color) = cache.entries.get(&key) {
            return Ok(Some(color));
        }

        let color = Self::inner_extract(path, config)?;
        if let Some(col) = color {
            cache.entries.insert(key, col);
            _ = cache.save(&cache_file);
        }

        Ok(color)
    }

    pub fn inner_extract<P>(
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

        let (sal_map, is_sal_worth) = extractor.gen_saliency(&img);
        #[cfg(debug_assertions)]
        extractor.save_saliency(&sal_map);

        let rgb_img = img.to_rgb8();

        let candidates =
            extractor.get_candidates(&rgb_img, &sal_map, is_sal_worth);
        Ok(extractor.get_best_col(candidates))
    }

    /// Resizes the image only if new dimensions are provided.
    fn prep_img(&mut self, img: DynamicImage) -> DynamicImage {
        let tw = self.config.res_w.unwrap_or(img.width());
        let th = self.config.res_h.unwrap_or(img.height());
        self.width = tw as usize;
        self.height = th as usize;

        if tw == img.width() && th == img.height() {
            return img;
        }
        img.resize_exact(tw, th, FilterType::Triangle)
    }

    /// Generates the normalized u8 saliency map
    fn gen_saliency(&self, img: &DynamicImage) -> (Vec<u8>, bool) {
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
        let is_sal_worth = avg_sal >= self.config.sal_thresh;

        (sal_map, is_sal_worth)
    }

    /// Finds the best color based on saliency and HSV.
    fn get_candidates(
        &self,
        rgb_img: &RgbImage,
        sal_map: &[u8],
        is_worth: bool,
    ) -> Vec<ScoredPixel> {
        let mut candidates = Vec::new();

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.;
            let g = pixel[1] as f32 / 255.;
            let b = pixel[2] as f32 / 255.;

            let srgb = Srgb::new(r, g, b);
            let hsv = Hsv::from_color(srgb);
            if hsv.value < self.config.val_thresh
                || hsv.saturation < self.config.sat_thresh
            {
                continue;
            }

            let mut score = hsv.saturation * hsv.value;
            if is_worth {
                let id = y as usize * self.width + x as usize;
                let sal_val = sal_map[id] as f32 / 255.;
                score *= 1.0 + sal_val * self.config.sal_bonus;
            }

            let hue = hsv.hue.into_positive_degrees();
            let warmth = 1.0 - (hue.min(360. - hue) / 180.);
            score *= 1.0 + warmth * self.config.warmth_bonus;

            let lab: Lab = srgb.into_color();
            candidates.push(ScoredPixel::new(lab, pixel, score));
        }
        candidates
    }

    /// Gets best color from the candidate pixels.
    ///
    /// It uses k-means clustering in order to find the best color, where
    /// it picks cluster with the highest average value (which suits the
    /// cluster size requirement) and picks the pixel with the highest score.
    fn get_best_col(&self, candids: Vec<ScoredPixel>) -> Option<(u8, u8, u8)> {
        let clusters = self.get_clusters(candids);
        let min_size = ((self.width * self.height) as f32 * 0.001) as usize;

        let mut best = None;
        let mut max_score = -1.;
        for cluster in clusters {
            if cluster.cnt < min_size {
                continue;
            }

            let avg_score = cluster.score / cluster.cnt as f32;
            if avg_score > max_score {
                max_score = avg_score;
                best = Some(cluster.best_rgb);
            }
        }
        best
    }

    /// Gets [`ScoredClusters`] from using k-means clustring.
    fn get_clusters(&self, candids: Vec<ScoredPixel>) -> Vec<ScoredCluster> {
        let labs: Vec<Lab> = candids.iter().map(|c| c.lab).collect();
        let k = self.config.clusters.min(labs.len());

        let res = get_kmeans(k, 20, 10.0, false, &labs, 0);

        let mut clusters = vec![ScoredCluster::default(); k];
        for (i, &cid) in res.indices.iter().enumerate() {
            clusters[cid as usize].push(&candids[i]);
        }
        clusters
    }

    #[cfg(debug_assertions)]
    fn save_saliency(&self, sal_img: &[u8]) {
        if let Some(img) = image::GrayImage::from_raw(
            self.width as u32,
            self.height as u32,
            sal_img.to_owned(),
        ) {
            img.save("debug_saliency.png")
                .expect("Failed to save debug image");
            println!("Saved debug_saliency.png to current directory.");
        }
    }
}
