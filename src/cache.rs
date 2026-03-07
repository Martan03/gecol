use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{Config, Error};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cache {
    pub entries: HashMap<String, (u8, u8, u8)>,
}

impl Cache {
    /// Loads the cache from the default cache file path.
    ///
    /// Default cache file path is given by the [`Cache::file`].
    ///
    /// If it fails to load or find the cache file, it returns default cache.
    pub fn load_default() -> Self {
        Self::load(Self::file())
    }

    /// Loads the cache from the given file path.
    ///
    /// If it fails to load or find the cache file, it returns default cache.
    pub fn load<P>(file: P) -> Self
    where
        P: AsRef<Path>,
    {
        match std::fs::read_to_string(file) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Saves the current cache to the default cache file path.
    ///
    /// Default cache file path is given by the [`Cache::file`].
    pub fn save_default(&self) -> Result<(), Error> {
        self.save(Self::file())
    }

    /// Saves the current cache to the given file path.
    pub fn save<P>(&self, file: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file, toml::to_string(self)?)?;
        Ok(())
    }

    /// Generates the cache key for the given config and image.
    ///
    /// This is done so that any change in config, that would effect the
    /// extracted color, is detected. It also can detect the image being
    /// modified.
    pub fn key<P>(config: &Config, image: P) -> Result<String, Error>
    where
        P: AsRef<Path>,
    {
        let mut hasher = DefaultHasher::new();

        let image = std::fs::canonicalize(image)?;
        image.hash(&mut hasher);

        if let Ok(Ok(modifier)) =
            std::fs::metadata(image).map(|v| v.modified())
        {
            modifier.hash(&mut hasher);
        }

        config.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Gets the default cache directory.
    pub fn dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| ".".into())
            .join("gecol")
    }

    /// Gets the default cache file path.
    ///
    /// It uses the [`Cache::dir`] to get the cache directory, followed by the
    /// `colors.toml`.
    pub fn file() -> PathBuf {
        Self::dir().join("color.toml")
    }
}
