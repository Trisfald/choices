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
///
/// This macro guarantees an unique port number as long as:
/// - the file starts with two `a-Z` characters
/// - no other file using the macro starts with the same two characters
/// - no two macro usages in the same file occur in less than 6 lines
/// - the macro is not used on line number > 599 *
///
/// (*) checked statically
#[macro_export]
macro_rules! file_line_port {
    () => {{
        let file = file!().as_bytes();
        let chars_n: u16 = 25;
        let ports_per_file: u16 = 100;
        let ports_line_interval: u16 = 6;
        let max_line_number = ports_per_file * ports_line_interval - 1;
        let line = line!() as u16;
        if line > max_line_number {
            panic!(
                "can't use file_line_port on line number > {}",
                max_line_number
            );
        }
        // Skip reserved ports.
        1024 as u16 +
                                // There can be `chars_n` different chars.
                                // Reserve ports for the second char * slots per file.
                                (file[0] - 97) as u16 * chars_n * ports_per_file +
                                // Divide line number by `ports_line_interval`.
                                (file[1] - 97) as u16 * ports_per_file + line / ports_line_interval
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
