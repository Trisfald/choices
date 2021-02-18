//! Using json to represent the configuration's values.

// Run the example with `cargo run --example json --features="json"`

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
#[choices(json)]
struct Config {
    port: u16,
    files: Vec<String>,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        port: 10,
        files: vec![String::from("tmp.txt"), String::from("log.txt")]
    }));
}

#[tokio::main]
async fn main() {
    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // Get the configuration's index at: curl localhost:8081/config
    // To view the value of `files`: curl localhost:8081/config/files
    // To change `files`: curl -X PUT -H "Content-Type: application/json" localhost:8081/config/files -d '["tmp.txt","other.txt"]'
}
