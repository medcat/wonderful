use std::error::Error as TraitError;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::num::ParseIntError;
use std::io::Error as IoError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;
use discord::Error as DiscordError;
use redis::RedisError;
use std::convert::From;

#[derive(Debug)]
pub struct Error(String, Box<TraitError + Send + Sync>);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> { write!(f, "GenericError({})", self.0) }
}

impl TraitError for Error {
    fn description(&self) -> &str { &self.0 }
    fn cause(&self) -> Option<&TraitError> { Some(self.1.as_ref()) }
}

macro_rules! error_from {
    ($x:ty) => (impl From<$x> for Error { fn from(err: $x) -> Error { Error(err.description().to_owned(), Box::new(err)) } });
    ($x:ty, $($y:ty),+) => (
        impl From<$x> for Error { fn from(err: $x) -> Error { Error(err.description().to_owned(), Box::new(err)) } }
        $(impl From<$y> for Error { fn from(err: $y) -> Error { Error(err.description().to_owned(), Box::new(err)) } })*
    );
}

error_from!(ParseIntError, IoError, TomlDeError, TomlSerError, DiscordError, RedisError);
