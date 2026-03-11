use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use gecol_core::{
    prelude::{ExtractionConfig, Template},
    theme::ThemeType,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(flatten)]
    pub extraction: ExtractionConfig,

    // Fallback color when extraction fails to extract a color.
    #[serde(default)]
    pub fallback_color: Option<String>,
    // Which theme type to use. Visit [`ThemeType`] to see all the variants.
    #[serde(default)]
    pub theme_type: ThemeType,

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

    /// Gets the fallback color as RGB tuple. Returns `None` if conversion
    /// fails or fallback color was not set.
    pub fn fallback_color(&self) -> Option<(u8, u8, u8)> {
        let hex = self.fallback_color.as_ref()?.trim_start_matches('#');
        if hex.len() == 6
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            )
        {
            return Some((r, g, b));
        }

        None
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
