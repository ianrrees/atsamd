//! Error handling types for atsamd-rs/atsamd management program

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Git error: {0}")]
    Git(git2::Error),
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("TOML deserializing error: {0}")]
    Toml(toml::de::Error),
    #[error("Handlebars rendering error: {0}")]
    HBRender(handlebars::RenderError),
    #[error("Handlebars template error: {0}")]
    HBTemplate(handlebars::TemplateError),
    #[error("Failure aleady logged to stderr")]
    Logged,
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Error::HBRender(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Error::HBTemplate(err)
    }
}

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Error::Git(value)
    }
}
