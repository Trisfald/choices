//! Example of using an existing server for configuration.

// Run the example with `cargo run --example custom_warp_server` then try the following commands:
// `curl localhost:8081/config/user`
// `curl localhost:8081/hello`
//
// `curl localhost:8082/config/user`
// `curl localhost:8082/hello`

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Choices)]
struct Config {
    user: String,
}

lazy_static! {
    static ref CONFIG: Config = {
        Config {
            user: "Matt".to_string(),
        }
    };
    static ref CONFIG2: Arc<Mutex<Config>> = {
        Arc::new(Mutex::new(Config {
            user: "David".to_string(),
        }))
    };
}

#[tokio::main]
async fn main() {
    // For immutable config call filter().
    let routes = CONFIG.filter().or(warp::path("hello").map(|| "Hello!"));
    let future = warp::serve(routes).run((std::net::Ipv4Addr::LOCALHOST, 8081));
    // For mutable config call filter_mutable().
    let routes2 = Config::filter_mutable(CONFIG2.clone()).or(warp::path("hello").map(|| "Hello!"));
    let future2 = warp::serve(routes2).run((std::net::Ipv4Addr::LOCALHOST, 8082));

    tokio::join!(future, future2);
}
