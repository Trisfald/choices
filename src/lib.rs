//! Easy HTTP configuration library.
//!
//! `choices` is a library that lets you expose your application's configuration
//! over HTTP with a simple struct!
//!
//! ## Examples
//!
//! Given the following code:
//!
//! ```no_run
//! use choices::Choices;
//! use lazy_static::lazy_static;
//!
//! #[derive(Choices)]
//! struct Config {
//!     debug: bool,
//!     id: Option<i32>,
//!     log_file: String,
//! }
//!
//! lazy_static! {
//!     static ref CONFIG: Config = {
//!         Config {
//!             debug: false,
//!             id: Some(3),
//!             log_file: "log.txt".to_string()
//!         }
//!     };
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     CONFIG.run(([127, 0, 0, 1], 8081)).await;
//! }
//! ```
//!
//! You can see all configuration fields at `localhost:8081/config`
//! and the individual fields' values at `localhost:8081/config/<field name>`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

#[doc(hidden)]
pub use choices_derive::*;

pub mod serializer;
pub use serializer::ChoicesOutput;

use async_trait::async_trait;
use std::net::SocketAddr;

/// A trait to manage the http server responsible for the configuration.
#[async_trait]
pub trait Choices {
    /// Starts the configuration http server on the chosen address.
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T);
}
