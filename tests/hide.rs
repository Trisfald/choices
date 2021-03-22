use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
struct Config {
    #[choices(hide_get)]
    a: i32,
    #[choices(hide_put)]
    b: i32,
    #[choices(hide_get, hide_put)]
    c: i32,
}

#[tokio::test]
async fn get_should_be_hidden() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    let response =
        retry_await!(reqwest::get(&format!("http://127.0.0.1:{}/config/a", port))).unwrap();
    assert_eq!(response.status(), 405);

    check_get_field_text!(port, b, "0");

    let response =
        retry_await!(reqwest::get(&format!("http://127.0.0.1:{}/config/c", port))).unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}

#[tokio::test]
async fn put_should_be_hidden() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    let response = retry_await!(reqwest::Client::builder()
        .build()
        .unwrap()
        .put(&format!("http://127.0.0.1:{}/config/a", port))
        .body("3")
        .send())
    .unwrap();
    assert_eq!(response.status(), 200);

    check_empty_put!(port, b, 405);

    check_empty_put!(port, c, 404);

    rt.shutdown_background();
}
