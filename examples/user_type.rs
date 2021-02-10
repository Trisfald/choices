//! Defining your own configuration types.

// Run the example with `cargo run --example user_type`

use bytes::Bytes;
use choices::{Choices, ChoicesError, ChoicesInput, ChoicesOutput, ChoicesResult};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct TimePoint {
    years: u32,
    seconds: u32,
}

// You must implement the traits ChoicesInput and ChoicesOutput for your type.

impl ChoicesInput<'_> for TimePoint {
    fn from_chars(bytes: &Bytes) -> ChoicesResult<Self> {
        let chars = std::str::from_utf8(&bytes)?;
        let v: Vec<&str> = chars.split('.').collect();
        if v.len() == 2 {
            let years = v[0].parse::<u32>()?;
            let seconds = v[1].parse::<u32>()?;
            Ok(TimePoint { years, seconds })
        } else {
            Err(ChoicesError::ParseError(
                "failed to parse input".to_string(),
            ))
        }
    }
}

impl ChoicesOutput for TimePoint {
    fn body_string(&self) -> String {
        format!("{}.{}", self.years, self.seconds)
    }
}

#[derive(Choices, Default)]
struct Config {
    time_point: TimePoint,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

#[tokio::main]
async fn main() {
    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // View the value: curl localhost:8081/config/time_point
    // Set the value: curl -X PUT localhost:8081/config/time_point -d "42.1"
}
