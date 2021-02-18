//! Collection of various constants.

pub(crate) const CONTENT_TYPE_HEADER: &str = "Content-Type";
pub(crate) const CONTENT_TYPE_TEXT: &str = "text/plain; charset=utf-8";
#[cfg(feature = "json")]
pub(crate) const CONTENT_TYPE_JSON: &str = "application/json";
