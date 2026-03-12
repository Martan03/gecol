use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// Holds all the extraction configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionConfig {
    // The width image to extract color from resizes to. If set to `None`, it
    // keeps the original width.
    #[serde(rename = "resize_width", default = "default_resize")]
    pub res_w: Option<u32>,
    // The height image to extract color from resizes to. If set to `None`, it
    // keeps the original height.
    #[serde(rename = "resize_height", default = "default_resize")]
    pub res_h: Option<u32>,

    // Saturation threshold for a pixel to be consider for the extraction.
    // If the pixels saturation is lower than this value, it's skipped.
    #[serde(rename = "saturation_threshold", default = "default_sat_thresh")]
    pub sat_thresh: f32,
    // Value threshold for a pixel to be consider for the extraction.
    // If the pixels value is lower than this value, it's skipped.
    #[serde(rename = "value_threshold", default = "default_val_thresh")]
    pub val_thresh: f32,

    // Threshold for the saliency to be used. If the average pixel saliency
    // is lower than this value, saliency is not used.
    #[serde(rename = "saliency_threshold", default)]
    pub sal_thresh: f32,
    // By how much the pixels saliency is multiplied by for its score.
    #[serde(rename = "saliency_bonus", default = "default_sal_bonus")]
    pub sal_bonus: f32,
    // By how much the pixels warmth factor is multiplied by for its score.
    #[serde(default = "default_warmth_bonus")]
    pub warmth_bonus: f32,

    // Number of cluster used in the k-means clustering of the pixels:
    #[serde(default = "default_clusters")]
    pub clusters: usize,

    // By how much the final cluster's vibrancy (chroma) is multiplied.
    #[serde(rename = "vibrancy_bonus", default = "default_vibr_bonus")]
    pub vibr_bonus: f32,
    // By how much the final cluster's dominance (pixel mass) is multiplied.
    #[serde(rename = "dominance_bonus", default = "default_dom_bonus")]
    pub dom_bonus: f32,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            res_w: default_resize(),
            res_h: default_resize(),
            sat_thresh: default_sat_thresh(),
            val_thresh: default_val_thresh(),
            sal_thresh: Default::default(),
            sal_bonus: default_sal_bonus(),
            warmth_bonus: default_warmth_bonus(),
            vibr_bonus: default_vibr_bonus(),
            dom_bonus: default_dom_bonus(),
            clusters: default_clusters(),
        }
    }
}

impl Hash for ExtractionConfig {
    /// Hashes the config state.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.res_w.hash(state);
        self.res_h.hash(state);
        self.sat_thresh.to_bits().hash(state);
        self.val_thresh.to_bits().hash(state);
        self.sal_thresh.to_bits().hash(state);
        self.sal_bonus.to_bits().hash(state);
        self.warmth_bonus.to_bits().hash(state);
        self.clusters.hash(state);
        self.vibr_bonus.to_bits().hash(state);
        self.dom_bonus.to_bits().hash(state);
    }
}

fn default_resize() -> Option<u32> {
    Some(255)
}

fn default_sat_thresh() -> f32 {
    0.2
}

fn default_val_thresh() -> f32 {
    0.15
}

fn default_sal_bonus() -> f32 {
    1.5
}

fn default_warmth_bonus() -> f32 {
    0.1
}

fn default_clusters() -> usize {
    16
}

fn default_vibr_bonus() -> f32 {
    2.5
}

fn default_dom_bonus() -> f32 {
    1.
}
