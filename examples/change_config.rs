//! Customizing the configuration's root path.

// Get the configuration at `curl localhost:8081/myconfig`

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
struct Config {
    debug: bool,
    name: String,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        debug: false,
        name: String::from("service")
    }));
}

#[tokio::main]
async fn main() {
    // Set a config field directly.
    CONFIG.lock().unwrap().debug = true;

    // Set a config field through its setter.
    // CONFIG.lock().unwrap().set_debug(true);

    CONFIG.run(([127, 0, 0, 1], 8081)).await;

    // TODO
}
