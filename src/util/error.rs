use crate::{commands::aoc_cli::AocClientError, AocDate};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    AocClientError(AocClientError),
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    InvalidDate(AocDate),
    StripPrefix(std::path::StripPrefixError),
    Unknown(String),
}

impl From<AocClientError> for Error {
    fn from(e: AocClientError) -> Self {
        Self::AocClientError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::TomlDeserialize(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self::TomlSerialize(e)
    }
}

impl From<AocDate> for Error {
    fn from(e: AocDate) -> Self {
        Self::InvalidDate(e)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Self {
        Self::StripPrefix(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.to_string())
    }
}
