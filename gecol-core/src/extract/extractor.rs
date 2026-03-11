use std::path::Path;

use image::{DynamicImage, RgbImage, imageops::FilterType};
use imgref::Img;
use kmeans_colors::get_kmeans;
use mss_saliency::maximum_symmetric_surround_saliency;
use palette::{FromColor, Hsv, IntoColor, Lab, Srgb};

use crate::{
    Cache,
    error::Error,
    extract::{
        ExtractionConfig, ExtractStep,
        scores::{ScoredCluster, ScoredPixel},
    },
};

/// A utility for extracting accent color from an image.
///
/// This structs acts as a temporary state container during the extraction
/// process and provides static methods for executing the extraction.
///
/// There are multiple extraction variants, mainly with
/// ([`Extractor::extract`]) and without ([`Extractor::extract_cached`]) using
/// the [`Cache`]. It is highly recommended using the [`Cache`]. High
/// resolution images can take a while to just open, so thanks to the [`Cache`]
/// the result for repeated extraction will be pretty much instant.
///
/// You can also use variant with progress reporting
/// ([`Extractor::extract_with_progress`] and
/// [`Extractor::extract_cached_with_progress`]), which is useful for having
/// a loading screen, for example.
#[derive(Debug, Clone)]
pub struct Extractor<'a> {
    config: &'a ExtractionConfig,
    width: usize,
    height: usize,
}

impl<'a> Extractor<'a> {
    /// Extracts the accent color from the image at the given path.
    ///
    /// When no sufficient color is found, it returns `None`.
    pub fn extract<P>(
        path: P,
        config: &'a ExtractionConfig,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
    {
        Self::inner_extract(path, config, |_| {})
    }

    /// Extracts the accent color from the image at the given path and uses
    /// the cache.
    ///
    /// It checks if the cache already contains the color for the given image,
    /// otherwise it saves the extracted color into the cache.
    ///
    /// When no sufficient color is found, it returns `None`.
    pub fn extract_cached<P>(
        path: P,
        config: &'a ExtractionConfig,
        cache_path: Option<&Path>,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
    {
        Self::extract_cached_with_progress(path, config, cache_path, |_| {})
    }

    /// Extracts the accent color from the image at the given path with the
    /// progress reporting.
    ///
    /// When no sufficient color is found, it returns `None`.
    pub fn extract_with_progress<P, F>(
        path: P,
        config: &'a ExtractionConfig,
        progress_callback: F,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
        F: FnMut(ExtractStep),
    {
        Self::inner_extract(path, config, progress_callback)
    }

    /// Extracts the accent color from the image at the given path with the
    /// progress reporting and uses the cache.
    ///
    /// It checks if the cache already contains the color for the given image,
    /// otherwise it saves the extracted color into the cache.
    ///
    /// When no sufficient color is found, it returns `None`.
    pub fn extract_cached_with_progress<P, F>(
        path: P,
        config: &'a ExtractionConfig,
        cache_path: Option<&Path>,
        mut progress_callback: F,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
        F: FnMut(ExtractStep),
    {
        progress_callback(ExtractStep::CheckingCache);
        let cache_file =
            cache_path.map(|v| v.to_owned()).unwrap_or_else(Cache::file);
        let mut cache = Cache::load(&cache_file);
        let key = Cache::key(config, path.as_ref())
            .unwrap_or("fallback".to_string());

        if let Some(&color) = cache.entries.get(&key) {
            progress_callback(ExtractStep::FinishedWithCache);
            return Ok(Some(color));
        }

        let color = Self::inner_extract(path, config, progress_callback)?;
        if let Some(col) = color {
            cache.entries.insert(key, col);
            _ = cache.save(&cache_file);
        }

        Ok(color)
    }

    fn inner_extract<P, F>(
        path: P,
        config: &'a ExtractionConfig,
        mut progress_callback: F,
    ) -> Result<Option<(u8, u8, u8)>, Error>
    where
        P: AsRef<Path>,
        F: FnMut(ExtractStep),
    {
        let mut extractor = Self {
            config,
            width: 0,
            height: 0,
        };

        progress_callback(ExtractStep::OpeningImage);
        let img = image::open(path)?;
        progress_callback(ExtractStep::ResizingImage);
        let img = extractor.prep_img(img);

        progress_callback(ExtractStep::ExtractingColors);
        let (sal_map, is_sal_worth) = extractor.gen_saliency(&img);
        #[cfg(debug_assertions)]
        extractor.save_saliency(&sal_map);

        let rgb_img = img.to_rgb8();

        let candids =
            extractor.get_candidates(&rgb_img, &sal_map, is_sal_worth);

        progress_callback(ExtractStep::Clustering);
        let col = extractor.get_best_col(candids);
        progress_callback(ExtractStep::Finished);
        Ok(col)
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
        let max_cnt = clusters.iter().map(|c| c.cnt).max().unwrap_or(1) as f32;

        let mut best = None;
        let mut max_score = -1.;
        for cluster in clusters {
            if cluster.cnt < min_size {
                continue;
            }

            let avg_score = cluster.score / cluster.cnt as f32;
            let mass_score = (cluster.cnt as f32 / max_cnt).sqrt();
            let mut score = avg_score * mass_score * self.config.dom_bonus;

            let sab = cluster.best_lab.a.powi(2) + cluster.best_lab.b.powi(2);
            let mut vibr_score = sab / 10000.;

            let r = cluster.best_rgb.0 as f32 / 255.;
            let g = cluster.best_rgb.1 as f32 / 255.;
            let b = cluster.best_rgb.2 as f32 / 255.;
            vibr_score *= r.max(g.max(b));

            score += vibr_score * self.config.vibr_bonus;
            if score > max_score {
                max_score = score;
                best = Some(cluster.best_rgb);
            }
        }
        best
    }

    /// Gets [`ScoredCluster`]s from using k-means clustring.
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
        }
    }
}
