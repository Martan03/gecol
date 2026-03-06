use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Error, template::template::Template};

/// Holds all the geocol configuration.
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
    /// Loads the config from the default config file path.
    ///
    /// It returns default config when the config file is not found.
    ///
    /// Default config file path is given by [`Config::file`].
    pub fn load_default() -> Self {
        Self::load(Self::file()).unwrap_or_default()
    }

    /// Loads the config from the given path.
    ///
    /// Config is required to be in TOML format.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    /// Saves the current config to the default config file path.
    ///
    /// Default config file path is given by [`Config::file`]. If the folders
    /// don't exists, it creates them.
    pub fn save_default(&self) -> Result<(), Error> {
        self.save(Self::file())
    }

    /// Saves the current config to the given file path.
    ///
    /// If the folder on the path don't exist, it creates them.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Gets the default config directory.
    ///
    /// It is `geocol` folder inside of the config directory
    /// (e.g. `.config` on linux)
    pub fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("gecol")
    }

    /// Gets the default config file path.
    ///
    /// It uses the [`Config::dir`] to get the config directory, followed
    /// by the `config.toml`.
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
