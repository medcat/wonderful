use std::error::Error as TraitError;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::num::ParseIntError;
use std::str::ParseBoolError;
use std::io::Error as IoError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;
use discord::Error as DiscordError;
use redis::RedisError;
use std::convert::From;
use hyper;

#[derive(Debug)]
pub enum Error {
    ParseIntError(ParseIntError),
    ParseBoolError(ParseBoolError),
    IoError(IoError),
    TomlDeError(TomlDeError),
    TomlSerError(TomlSerError),
    DiscordError(DiscordError),
    RedisError(RedisError),
}

impl Error {
    pub fn is_recoverable(&self) -> bool {
        match self {
            &Error::DiscordError(DiscordError::Status(s, _)) =>
                s == hyper::status::StatusCode::Forbidden ||
                s == hyper::status::StatusCode::NotFound,
            &Error::ParseIntError(_) => true,
            &Error::ParseBoolError(_) => true,
            _ => false
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

impl TraitError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ParseIntError(ref e) => e.description(),
            &Error::ParseBoolError(ref e) => e.description(),
            &Error::IoError(ref e) => e.description(),
            &Error::TomlDeError(ref e) => e.description(),
            &Error::TomlSerError(ref e) => e.description(),
            &Error::DiscordError(ref e) => e.description(),
            &Error::RedisError(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&TraitError> {
        match self {
            &Error::ParseIntError(ref e) => Some(e),
            &Error::ParseBoolError(ref e) => Some(e),
            &Error::IoError(ref e) => Some(e),
            &Error::TomlDeError(ref e) => Some(e),
            &Error::TomlSerError(ref e) => Some(e),
            &Error::DiscordError(ref e) => Some(e),
            &Error::RedisError(ref e) => Some(e),
        }
    }
}

macro_rules! error_from {
    ($e:ty => $v:ident) => (
        impl From<$e> for Error { fn from(err: $e) -> Error { Error::$v(err) } }
    );
    ($e:ident => $v:ident, $($ei:ty => $vi:ident),+) => (
        impl From<$e> for Error { fn from(err: $e) -> Error { Error::$v(err) } }
        $(impl From<$ei> for Error { fn from(err: $ei) -> Error { Error::$vi(err) } })*
    );
}

error_from!(
    ParseIntError => ParseIntError,
    ParseBoolError => ParseBoolError,
    IoError => IoError,
    TomlDeError => TomlDeError,
    TomlSerError => TomlSerError,
    DiscordError => DiscordError,
    RedisError => RedisError);
