use thiserror::Error;

/// Geocol error wrapper around all the possible errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Img(#[from] image::ImageError),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    MiniJinja(#[from] minijinja::Error),
    #[error("{0}")]
    Msg(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Msg(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}
