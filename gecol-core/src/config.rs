use std::{
    collections::HashMap,
    hash::Hash,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, template::Template};

/// Holds all the gecol configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
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

    // List of templates to be built.
    #[serde(default)]
    pub templates: HashMap<String, Template>,
    // Path to the directory containing the templates.
    // `~/.config/gecol/templates` by default on linux.
    #[serde(default)]
    pub templates_dir: Option<PathBuf>,

    /// Path to the directory containing cached colors for faster repeated
    /// extraction.
    #[serde(default)]
    pub cache_dir: Option<PathBuf>,
    /// Whether no color cache should be used.
    #[serde(default)]
    pub no_cache: bool,
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
        let mut config: Config = toml::from_str(&content)?;

        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        if let Some(path) = &mut config.templates_dir {
            Self::expand_tilde(path, &home_dir);
        }
        config.resolve_paths(&home_dir);

        Ok(config)
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
    /// It is `gecol` folder inside of the config directory
    /// (e.g. `.config` on linux)
    pub fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("gecol")
    }

    /// Gets the templates directory.
    ///
    /// If the templates directory is not set, it uses `templates` directory
    /// inside the default config directory.
    pub fn templates_dir(&self) -> PathBuf {
        match &self.templates_dir {
            Some(dir) => dir.to_owned(),
            None => Self::dir().join("templates"),
        }
    }

    /// Gets the default config file path.
    ///
    /// It uses the [`Config::dir`] to get the config directory, followed
    /// by the `config.toml`.
    pub fn file() -> PathBuf {
        Self::dir().join("config.toml")
    }

    /// Resolves the template paths.
    ///
    /// If the source path is not absolute, it is in the templates directory,
    /// and if the target is not absolute, it is in the home directory.
    fn resolve_paths(&mut self, home_dir: &Path) {
        let dir = self.templates_dir();

        for template in self.templates.values_mut() {
            Self::expand_tilde(&mut template.source, home_dir);
            Self::expand_tilde(&mut template.target, home_dir);

            if !template.source.is_absolute() {
                template.source = dir.join(&template.source);
            }

            if !template.target.is_absolute() {
                template.target = home_dir.join(&template.target);
            }
        }
    }

    fn expand_tilde(path: &mut PathBuf, home_dir: &Path) {
        if let Ok(stripped) = path.strip_prefix("~") {
            *path = home_dir.join(stripped);
        }
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
            vibr_bonus: default_vibr_bonus(),
            dom_bonus: default_dom_bonus(),
            clusters: default_clusters(),
            templates: Default::default(),
            templates_dir: Default::default(),
            cache_dir: Default::default(),
            no_cache: Default::default(),
        }
    }
}

impl Hash for Config {
    /// Hashes the config state. It doesn't include the templates configuration
    /// since it doesn't effect the actual extraction.
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
