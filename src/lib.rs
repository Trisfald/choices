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
//! use std::sync::{Arc, Mutex};
//!
//! #[derive(Choices)]
//! struct Config {
//!     debug: bool,
//!     id: Option<i32>,
//!     log_file: String,
//! }
//!
//! lazy_static! {
//!     static ref CONFIG: Arc<Mutex<Config>> = {
//!         Arc::new(Mutex::new(Config {
//!             debug: false,
//!             id: Some(3),
//!             log_file: "log.txt".to_string()
//!         }))
//!     };
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;
//! }
//! ```
//!
//! You can see all configuration fields at `localhost:8081/config`
//! and the individual fields' values at `localhost:8081/config/<field name>`\
//! A field's value can be changed with a `PUT`, for instance
//! `curl -X PUT localhost:8081/config/debug -d "true"`.
//!
//! More examples on [github](https://github.com/Trisfald/choices/blob/master/examples/).
//!
//! ## Documentation
//! Check out the documentation on
//! [github](https://github.com/Trisfald/choices/blob/master/documentation.md).

#![forbid(unsafe_code)]
#![deny(missing_docs)]

#[doc(hidden)]
pub use choices_derive::*;

/// Re-export of `bytes`
pub mod bytes {
    pub use bytes::*;
}

/// Re-export of `warp`
pub mod warp {
    pub use warp::*;
}

#[cfg(feature = "json")]
/// Re-export of `serde_json`
pub mod serde_json {
    pub use serde_json::*;
}

#[doc(hidden)]
pub use async_trait::*;

pub mod error;
pub use crate::error::{ChoicesError, ChoicesResult};

pub mod serde;
pub use crate::serde::{ChoicesInput, ChoicesOutput};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

/// A trait to manage the http server responsible for the configuration.
#[self::async_trait]
pub trait Choices {
    /// Starts the configuration http server on the chosen address.
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T);

    #[doc(hidden)]
    async fn run_mutable<T: Into<SocketAddr> + Send>(_: Arc<Mutex<Self>>, _: T) {
        unimplemented!()
    }

    #[doc(hidden)]
    async fn run_mutable_rw<T: Into<SocketAddr> + Send>(_: Arc<RwLock<Self>>, _: T)
    where
        Self: Sync,
    {
        unimplemented!()
    }
}

#[self::async_trait]
impl<C: Choices + Send> Choices for Arc<Mutex<C>> {
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T) {
        C::run_mutable(self.clone(), addr).await;
    }
}

#[self::async_trait]
impl<C: Choices + Send + Sync> Choices for Arc<RwLock<C>> {
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T) {
        C::run_mutable_rw(self.clone(), addr).await;
    }
}
