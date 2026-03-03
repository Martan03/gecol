use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_resize")]
    pub resize_width: Option<u32>,
    #[serde(default = "default_resize")]
    pub resize_height: Option<u32>,
    #[serde(default)]
    pub sat_threshold: f32,
    #[serde(default)]
    pub val_threshold: f32,
    #[serde(default)]
    pub sal_threshold: f32,
}

impl Config {
    pub fn from_default_json() -> Self {
        Self::from_json(Self::file()).unwrap_or_default()
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<Self, Error> {
        let f = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(f)?)
    }

    pub fn to_default_json(&self) -> Result<(), Error> {
        self.to_json(Self::file())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let f = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }

    pub fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("gecol")
    }

    pub fn file() -> PathBuf {
        Self::dir().join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resize_width: default_resize(),
            resize_height: default_resize(),
            sat_threshold: Default::default(),
            val_threshold: Default::default(),
            sal_threshold: default_sal_threshold(),
        }
    }
}

fn default_resize() -> Option<u32> {
    Some(256)
}

fn default_sal_threshold() -> f32 {
    20.
}
