use std::fmt::{Debug, Display, Formatter};
use crate::value::Marker;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(std::io::Error),
    InvalidKey,
    InvalidMarker,
    InvalidString,
    TrailingData,
    Custom(String),
    Eof,
    ExpectedLength,
    Expected(Vec<Marker>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::error::Error for Error {}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::InvalidKey => write!(f, "invalid key"),
            Error::InvalidMarker => write!(f, "invalid marker"),
            Error::InvalidString => write!(f, "invalid string"),
            Error::TrailingData => write!(f, "trailing data"),
            Error::Custom(s) => write!(f, "{}", s),
            Error::Eof => write!(f, "end of input"),
            Error::ExpectedLength => write!(f, "expected length"),
            Error::Expected(markers) => {
                write!(f, "expected markers:")?;
                for c in markers.iter().map(|m| *m as u8 as char) {
                    write!(f, " {}", c)?;
                }
                Ok(())
            }
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        let s = format!("{}", msg);
        Self::Custom(s)
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        let s = format!("{}", msg);
        Self::Custom(s)
    }
}
