use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
struct Config {
    debug: bool,
    retries: u8,
    delay: f64,
    score: Option<i32>,
}

async fn get_list_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get!(
        port,
        "config",
        "Available configuration options:\n  - debug: bool\n  \
            - retries: u8\n  - delay: f64\n  - score: Option<i32>\n",
        util::CONTENT_TYPE_TEXT
    );

    rt.shutdown_background();
}

#[tokio::test]
async fn get_list() {
    let port = get_free_port!();
    get_list_impl(port, async move {
        Config {
            debug: true,
            retries: 3,
            delay: 0.1,
            score: Some(3),
        }
        .run((std::net::Ipv4Addr::LOCALHOST, port))
        .await
    })
    .await;
}

#[tokio::test]
async fn get_list_mutable() {
    let port = get_free_port!();
    get_list_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}
