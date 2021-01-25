//! Utilities for the tests.

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

/// Returns a port (u16) taken by reinterpreting the first two bytes
/// of the source file's name and the line number.
#[macro_export]
macro_rules! file_line_port {
    () => {{
        let file = file!().as_bytes();
        // Skip reserved ports.
        1024 as u16 +
                // There can be 25 different chars.
                // Reserve 2500 for the second char * 100 slots per file.
                (file[0] - 97) as u16 * 2500 as u16 +
                // Divide line number by 10 and support 100 slots per files (thus max length 1000).
                (file[1] - 97) as u16 * 100 as u16 + line!() as u16 / 10
    }};
}

/// Performs a GET on a server running on localhost on port
/// `port` and checks the body matches `expected`.
#[macro_export]
macro_rules! check_get {
    ($port:expr, $path:expr, $expected:expr) => {
        let response = retry_await!(reqwest::get(&format!(
            "http://127.0.0.1:{}/{}",
            $port, $path
        )))
        .unwrap();
        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, $expected);
    };
}

/// Performs a GET for the field `name` on a server running on localhost on port
/// `port` and checks the body matches `expected`.
#[macro_export]
macro_rules! check_get_field {
    ($port:expr, $name:expr, $expected:expr) => {
        check_get!($port, concat!("config/", stringify!($name)), $expected)
    };
}
