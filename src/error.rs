//! Handles errors.

use std::char::ParseCharError;
use std::num::{ParseFloatError, ParseIntError};
use std::result::Result;
use std::str::{ParseBoolError, Utf8Error};
use std::{error, fmt, fmt::Debug, fmt::Display, string::FromUtf8Error};

/// Alias for a `Result` returning a `ChoicesError`.
pub type ChoicesResult<T> = Result<T, ChoicesError>;

/// Error type for all kind of errors generated by `choices`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChoicesError {
    /// Generic parsing error.
    ParseError(String),
    /// Error when attempting to interpret a sequence of u8 as a string.
    Utf8Error(Utf8Error),
    /// Error when converting a String from a UTF-8 byte vector.
    FromUtf8Error(FromUtf8Error),
    /// Error when a conversion from slice to array fails.
    TryFromSliceError(usize, usize),
    /// Error when parsing an integer.
    ParseIntError(ParseIntError),
    /// Error when parsing a float.
    ParseFloatError(ParseFloatError),
    /// Error when parsing a bool.
    ParseBoolError(ParseBoolError),
    /// Error when parsing a char.
    ParseCharError(ParseCharError),
    /// Error when validating a field.
    ValidationError(String),
}

impl Display for ChoicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ChoicesError::*;
        match self {
            ParseError(err) => write!(f, "ParseError: {}", err),
            Utf8Error(err) => write!(f, "Utf8Error: {}", err),
            FromUtf8Error(err) => write!(f, "FromUtf8Error: {}", err),
            TryFromSliceError(len_provided, len_requested) => write!(
                f,
                "could not convert slice of len {} into array of len {}",
                len_provided, len_requested
            ),
            ParseIntError(err) => write!(f, "ParseIntError: {}", err),
            ParseFloatError(err) => write!(f, "ParseFloatError: {}", err),
            ParseBoolError(err) => write!(f, "ParseBoolError: {}", err),
            ParseCharError(err) => write!(f, "ParseCharError: {}", err),
            ValidationError(err) => write!(f, "ValidationError: {}", err),
        }
    }
}

impl error::Error for ChoicesError {}

macro_rules! impl_trivial_from_error {
    ($($ty:ident),*) => {
        $(impl From<$ty> for ChoicesError {
            fn from(err: $ty) -> Self {
                Self::$ty(err)
            }
        })*
    };
}

impl_trivial_from_error! {Utf8Error, FromUtf8Error, ParseIntError, ParseFloatError, ParseBoolError, ParseCharError}
