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
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, LockResult, Mutex, MutexGuard, RwLock};

/// Trait for a generic read/write lock.
///
/// Access to both read and write can be shared or exclusive, depending
/// on the concrete implementation.
pub trait Lock<'a>: Send + Sync {
    /// The type wrapped by the lock.
    type Value: ?Sized;
    /// Type of lock guard for read access.
    type ReadGuard: Deref<Target = Self::Value>;
    /// Type of lock guard for write access.
    type WriteGuard: Deref<Target = Self::Value> + DerefMut<Target = Self::Value>;

    /// Locks with read access.
    fn read(&'a self) -> LockResult<Self::ReadGuard>;
    /// Locks with write access.
    fn write(&'a self) -> LockResult<Self::WriteGuard>;
}

impl<'a, T: ?Sized + Send + 'a> Lock<'a> for Mutex<T> {
    type Value = T;
    type ReadGuard = MutexGuard<'a, T>;
    type WriteGuard = MutexGuard<'a, T>;

    fn read(&'a self) -> LockResult<Self::ReadGuard> {
        self.lock()
    }
    fn write(&'a self) -> LockResult<Self::WriteGuard> {
        self.lock()
    }
}

// impl Lock<T: ?Sized>for RwLock<T> {

// }

/// A trait to manage the http server responsible for the configuration.
#[self::async_trait]
pub trait Choices {
    /// Starts the configuration http server on the chosen address.
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T);

    #[doc(hidden)]
    async fn run_mutable<'a, T: Into<SocketAddr> + Send, U>(choices: Arc<U>, addr: T)
    where
        U: Lock<'a, Value = Self>,
        <U as Lock<'a>>::ReadGuard: Deref<Target = Self>,
        <U as Lock<'a>>::WriteGuard: Deref<Target = Self> + DerefMut<Target = Self>;
}

#[self::async_trait]
impl<L> Choices for Arc<L>
where
    for<'a> <L as Lock<'a>>::Value: Choices + Send,
    for<'a> L: Lock<'a>,
{
    async fn run<T: Into<SocketAddr> + Send>(&'static self, addr: T) {
        <<L as Lock>::Value as Choices>::run_mutable(self.clone(), addr).await;
    }

    #[doc(hidden)]
    async fn run_mutable<'b, T: Into<SocketAddr> + Send, U>(_: Arc<U>, _: T)
    where
        U: Lock<'b, Value = Self>,
    {
        panic!("do not call run_mutable() when T=Arc<Lock>, use run() instead")
    }
}
