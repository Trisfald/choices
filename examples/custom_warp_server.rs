//! Example of using an existing server for configuration.

// Run the example with `cargo run --example custom_warp_server` then try the following commands:
// `curl localhost:8081/config/user`
// `curl localhost:8081/hello`

use choices::Choices;
use lazy_static::lazy_static;
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
}

#[tokio::main]
async fn main() {
    let routes = CONFIG.filter().or(warp::path("hello").map(|| "Hello!"));
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}
