use choices::Choices;
use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
#[choices(json)]
struct Config {
    debug: bool,
    retries: u8,
    delay: f64,
    score: Option<i32>,
    map: HashMap<u8, i32>,
}

async fn get_list_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    let response =
        retry_await!(reqwest::get(&format!("http://127.0.0.1:{}/config", port))).unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers()[reqwest::header::CONTENT_TYPE],
        "application/json"
    );
    let body = response.text().await.unwrap();
    assert_eq!(
        body,
        json!([
            {"name": "debug", "type": "bool"},
            {"name": "retries", "type": "u8"},
            {"name": "delay", "type": "f64"},
            {"name": "score", "type": "Option<i32>"},
            {"name": "map", "type": "HashMap<u8, i32>"}
        ])
        .to_string()
    );

    rt.shutdown_background();
}

#[tokio::test]
async fn get_list() {
    let port = get_free_port!();
    get_list_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Config = Config::default();
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
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
