use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices)]
#[choices(rw_lock)]
struct Config {
    debug: bool,
}

#[tokio::test]
async fn get_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<RwLock<Config>> = Arc::new(RwLock::new(Config { debug: true }));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_get_text!(port, "config/debug", "true");

    rt.shutdown_background();
}

#[tokio::test]
async fn put_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<RwLock<Config>> = Arc::new(RwLock::new(Config { debug: true }));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field_text!(port, debug, "false", 200, "false");

    rt.shutdown_background();
}
