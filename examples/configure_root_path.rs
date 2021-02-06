//! Customizing the configuration's root path.

// Get the configuration at `curl localhost:8081/myconfig`

use choices::Choices;
use lazy_static::lazy_static;

#[derive(Choices)]
#[choices(path = "myconfig")]
struct Config {
    debug: bool,
}

lazy_static! {
    static ref CONFIG: Config = Config { debug: false };
}

#[tokio::main]
async fn main() {
    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;
}
