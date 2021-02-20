//! Utilities for the tests.

pub mod configs;
pub use configs::*;

pub mod constants;
pub use constants::*;

pub mod test_cases;
pub use test_cases::*;

/// Retries the given operation 50 times, waiting 1ms in between.
/// `op` must return an awaitable future.
#[macro_export]
macro_rules! retry_await {
    ($op:expr) => {{
        let mut i = 0;
        let mut result = $op.await;
        loop {
            i += 1;
            if result.is_ok() || i >= 50 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            result = $op.await;
        }
        result
    }};
}

/// Returns a free tcp port.
#[macro_export]
macro_rules! get_free_port {
    () => {{
        let port: u16 = util::portpicker::pick_unused_port().expect("no free port");
        port
    }};
}

#[macro_export]
macro_rules! check_get {
    ($port:expr, $path:expr, $expected:expr, $content_type:expr) => {
        let response = retry_await!(reqwest::get(&format!(
            "http://127.0.0.1:{}/{}",
            $port, $path
        )))
        .unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers()[reqwest::header::CONTENT_TYPE],
            $content_type
        );
        let body = response.text().await.unwrap();
        assert_eq!(body, $expected);
    };
}

/// Performs a GET on a server running on localhost on port
/// `port` and checks the body matches the `expected` text.
#[macro_export]
macro_rules! check_get_text {
    ($port:expr, $path:expr, $expected:expr) => {
        check_get!($port, $path, $expected, CONTENT_TYPE_TEXT)
    };
}

/// Performs a GET on a server running on localhost on port
/// `port` and checks the body matches the `expected` json.
#[macro_export]
macro_rules! check_get_json {
    ($port:expr, $path:expr, $expected:expr) => {
        check_get!($port, $path, $expected, CONTENT_TYPE_JSON)
    };
}

/// Performs a GET for the field `name` on a server running on localhost on port
/// `port` and checks the body matches the `expected` text.
#[macro_export]
macro_rules! check_get_field_text {
    ($port:expr, $name:expr, $expected:expr) => {
        check_get_text!($port, concat!("config/", stringify!($name)), $expected)
    };
}

/// Performs a GET for the field `name` on a server running on localhost on port
/// `port` and checks the body matches the `expected` json.
#[macro_export]
macro_rules! check_get_field_json {
    ($port:expr, $name:expr, $expected:expr) => {
        check_get_json!($port, concat!("config/", stringify!($name)), $expected)
    };
}

/// Performs a PUT for the field `name` on a server running on localhost on port
/// `port` and checks the response's status code. Then, it performs a GET to
/// verify the field's value corresponds to `expected`.
#[macro_export]
macro_rules! check_put_field_json {
    ($port:expr, $name:expr, $body:expr, $status:expr, $expected:expr $( , $headers:expr )* ) => {
        let response = retry_await!(reqwest::Client::builder()
            .build()
            .unwrap()
            .put(&format!(
                "http://127.0.0.1:{}/{}",
                $port,
                concat!("config/", stringify!($name))
            ))
            .body($body)
            $(.header($headers.0, $headers.1)),*
            .send())
        .unwrap();
        assert_eq!(response.status(), $status);
        check_get_field_json!($port, $name, $expected);
    };
}

/// Performs a PUT for the field `name` on a server running on localhost on port
/// `port` and checks the response's status code. Then, it performs a GET to
/// verify the field's value corresponds to `expected`.
#[macro_export]
macro_rules! check_put_field_text {
    ($port:expr, $name:expr, $body:expr, $status:expr, $expected:expr $( , $headers:expr )* ) => {
        let response = retry_await!(reqwest::Client::builder()
            .build()
            .unwrap()
            .put(&format!(
                "http://127.0.0.1:{}/{}",
                $port,
                concat!("config/", stringify!($name))
            ))
            .body($body)
            $(.header($headers.0, $headers.1)),*
            .send())
        .unwrap();
        assert_eq!(response.status(), $status);
        check_get_field_text!($port, $name, $expected);
    };
}

pub mod portpicker {
    // Code modified from the portpicker crate.
    //
    // Dropped the check for ipv6 and udp.

    use rand::prelude::*;
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, ToSocketAddrs};

    pub type Port = u16;

    // Try to bind to a socket using TCP
    pub fn test_bind_tcp<A: ToSocketAddrs>(addr: A) -> Option<Port> {
        Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
    }

    /// Check if a port is free on TCP
    pub fn is_free_tcp(port: Port) -> bool {
        let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
        test_bind_tcp(ipv4).is_some()
    }

    /// Asks the OS for a free port
    pub fn ask_free_tcp_port() -> Option<Port> {
        let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        test_bind_tcp(ipv4)
    }

    /// Picks an available port that is available on TCP
    pub fn pick_unused_port() -> Option<Port> {
        let mut rng = rand::thread_rng();

        // Try random port first
        for _ in 0..10 {
            let port = rng.gen_range(15000..25000);
            if is_free_tcp(port) {
                return Some(port);
            }
        }

        // Ask the OS for a port
        for _ in 0..10 {
            if let Some(port) = ask_free_tcp_port() {
                return Some(port);
            }
        }

        // Give up
        None
    }
}
