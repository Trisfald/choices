//! Basic example of `choices`' usage.

// Run the example with `cargo run --example simple_readonly` then try the following commands:
// `curl localhost:8081/config`
// `curl localhost:8081/config/debug`
// `curl localhost:8081/config/retries`
// `curl localhost:8081/config/delay`
// `curl localhost:8081/config/score`

use choices::Choices;

#[derive(Choices)]
struct Config {
    debug: bool,
    retries: u8,
    delay: f64,
    score: Option<i32>,
}

#[tokio::main]
async fn main() {
    Config {
        debug: true,
        retries: 3,
        delay: 0.1,
        score: Some(3),
    }
    .run((std::net::Ipv4Addr::LOCALHOST, 8081))
    .await;
}
