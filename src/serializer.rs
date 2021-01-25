//! Serializes fields' value.

use std::fmt::Display;

/// Traits to serialize types.
pub trait ChoicesOutput {
    /// Serializes `self` into a String.
    fn body_string(&self) -> String;
}

/// Implements the default impl of `ChoicesOutput`for the specified types.
macro_rules! default_impl {
    ($($ty:ident),*) => {
        $(impl ChoicesOutput for $ty {
            fn body_string(&self) -> String {
                format!("{}", &self)
            }
        })*
    };
}

default_impl! {bool, char, i128, i16, i32, i64, i8, u128, u16, u32, u64, u8, f32, f64, usize, isize}

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
            Some(v) => format!("Some({})", v),
            None => "None".to_string(),
        }
    }
}
