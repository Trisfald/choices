//! Customizing the configuration's root path.

// Get the configuration's index at `curl localhost:8081/config`

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
struct Config {
    port: u16,
    name: String,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        port: 10,
        name: String::from("service")
    }));
}

#[tokio::main]
async fn main() {
    // Set a config field directly.
    CONFIG.lock().unwrap().port = 100;

    // Set a config field through its setter.
    CONFIG.lock().unwrap().set_name("another service").unwrap();

    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // To change port: curl -X PUT localhost:8081/config/port -d "42"
    // To view the new value: curl localhost:8081/config/port
}
