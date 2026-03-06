use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Error, template::template::Template};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "resize_width", default = "default_resize")]
    pub res_w: Option<u32>,
    #[serde(rename = "resize_height", default = "default_resize")]
    pub res_h: Option<u32>,
    #[serde(rename = "saturation_threshold", default = "default_sat_thresh")]
    pub sat_thresh: f32,
    #[serde(rename = "value_threshold", default = "default_val_thresh")]
    pub val_thresh: f32,
    #[serde(rename = "saliency_threshold", default)]
    pub sal_thresh: f32,
    #[serde(rename = "saliency_bonus", default = "default_sal_bonus")]
    pub sal_bonus: f32,
    #[serde(default = "default_warmth_bonus")]
    pub warmth_bonus: f32,
    #[serde(default = "default_clusters")]
    pub clusters: usize,

    #[serde(rename = "template", default)]
    pub templates: Vec<Template>,
}

impl Config {
    pub fn load_default() -> Self {
        Self::load(Self::file()).unwrap_or_default()
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save_default(&self) -> Result<(), Error> {
        self.save(Self::file())
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("gecol")
    }

    pub fn file() -> PathBuf {
        Self::dir().join("config.toml")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            res_w: default_resize(),
            res_h: default_resize(),
            sat_thresh: default_sat_thresh(),
            val_thresh: default_val_thresh(),
            sal_thresh: Default::default(),
            sal_bonus: default_sal_bonus(),
            warmth_bonus: default_warmth_bonus(),
            clusters: default_clusters(),
            templates: Default::default(),
        }
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
    5.
}

fn default_warmth_bonus() -> f32 {
    1.5
}

fn default_clusters() -> usize {
    16
}
