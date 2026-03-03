use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "resize_width", default = "default_resize")]
    pub res_w: Option<u32>,
    #[serde(rename = "resize_height", default = "default_resize")]
    pub res_h: Option<u32>,
    #[serde(rename = "saturation_threshold", default)]
    pub sat_thresh: f32,
    #[serde(rename = "value_threshold", default)]
    pub val_thresh: f32,
    #[serde(rename = "saliency_threshold", default)]
    pub sal_thresh: f32,
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
            res_w: default_resize(),
            res_h: default_resize(),
            sat_thresh: Default::default(),
            val_thresh: Default::default(),
            sal_thresh: Default::default(),
        }
    }
}

fn default_resize() -> Option<u32> {
    Some(256)
}
