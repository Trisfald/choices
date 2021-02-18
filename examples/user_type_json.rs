//! Defining your own configuration types.

// Run the example with `cargo run --example user_type_json --features="json"`

use choices::Choices;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Default, Serialize, Deserialize)]
struct TimePoint {
    years: u32,
    seconds: u32,
}

#[derive(Choices, Default)]
#[choices(json)]
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
    // Set the value: curl -X PUT -H "Content-Type: application/json" localhost:8081/config/time_point -d '{"years":1,"seconds":10}'
}
