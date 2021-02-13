//! Using json to represent the configuration's values.

// Run the example with `cargo run --example json --features="json"`

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
#[choices(json)]
struct Config {
    port: u16,
    file: String,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        port: 10,
        file: String::from("tmp.txt")
    }));
}

#[tokio::main]
async fn main() {
    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // Get the configuration's index at: curl localhost:8081/config
    // To change port: curl -X PUT localhost:8081/config/port -d "42"
    // To view the new value: curl localhost:8081/config/port
}
