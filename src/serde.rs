//! Serializes and deserializes fields' value.

use crate::bytes::Bytes;
use crate::ChoicesResult;
use std::fmt::Display;
use std::str;

/// Traits to serialize values into character sequences.
pub trait ChoicesOutput {
    /// Serializes `self` into a String.
    fn body_string(&self) -> String;
}

/// Implements the default impl of `ChoicesOutput`for the specified types.
macro_rules! default_choices_output_impl {
    ($($ty:ident),*) => {
        $(impl ChoicesOutput for $ty {
            fn body_string(&self) -> String {
                format!("{}", &self)
            }
        })*
    };
}

default_choices_output_impl! {bool, char, i128, i16, i32, i64, i8, u128, u16, u32, u64, u8, f32, f64, usize, isize}

impl ChoicesOutput for &str {
    fn body_string(&self) -> String {
        self.to_string()
    }
}

impl ChoicesOutput for String {
    fn body_string(&self) -> String {
        self.clone()
    }
}

impl<T: Display> ChoicesOutput for Option<T> {
    fn body_string(&self) -> String {
        match self {
            Some(v) => format!("{}", v),
            None => "".to_string(),
        }
    }
}

/// Trait to retrieve values from character sequences.
pub trait ChoicesInput<'a> {
    /// Parses an instance of `Self` from a buffer of contiguous characters.
    ///
    /// Returns an error if the operation failed.
    fn from_chars(bytes: &'a Bytes) -> ChoicesResult<Self>
    where
        Self: Sized;
}

/// Implements the default impl of `ChoicesInput`for the specified types.
macro_rules! default_choices_input_impl {
    ($($ty:ident),*) => {
        $(impl ChoicesInput<'_> for $ty {
            fn from_chars(bytes: &Bytes) -> ChoicesResult<Self> {
                let chars = str::from_utf8(&bytes)?;
                Ok(chars.parse::<Self>()?)
            }
        })*
    };
}

default_choices_input_impl! {bool, char, i128, i16, i32, i64, i8, u128, u16, u32, u64, u8, f32, f64, usize, isize}

impl<'a> ChoicesInput<'a> for &'a str {
    fn from_chars(bytes: &'a Bytes) -> ChoicesResult<Self> {
        Ok(str::from_utf8(&bytes)?)
    }
}

impl ChoicesInput<'_> for String {
    fn from_chars(bytes: &Bytes) -> ChoicesResult<Self> {
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

impl<'a, T: Display + ChoicesInput<'a>> ChoicesInput<'a> for Option<T> {
    fn from_chars(bytes: &'a Bytes) -> ChoicesResult<Self> {
        if bytes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::from_chars(bytes)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{BufMut, BytesMut};

    #[test]
    fn generic_number_from_chars() {
        let i: u16 = 56;
        let mut buf = BytesMut::with_capacity(16);
        buf.put_slice(i.to_string().as_bytes());
        let buf = buf.freeze();
        assert_eq!(u16::from_chars(&buf), Ok(i));
    }

    #[test]
    fn option_from_chars() {
        let i: u16 = 56;
        let mut buf = BytesMut::with_capacity(16);
        buf.put_slice(i.to_string().as_bytes());
        let buf = buf.freeze();
        assert_eq!(Option::<u16>::from_chars(&buf), Ok(Some(i)));
        let buf = BytesMut::with_capacity(16);
        assert_eq!(Option::<u16>::from_chars(&buf.freeze()), Ok(None));
    }
}
