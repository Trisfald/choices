//! Common test case scenarios.

use crate::retry_await;
use choices::warp::Future;
use tokio::runtime::Runtime;

pub async fn get_non_existing_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    let response = retry_await!(reqwest::get(&format!(
        "http://127.0.0.1:{}/config/fake",
        port
    )))
    .unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}

pub async fn skip_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    let response = retry_await!(reqwest::get(&format!(
        "http://127.0.0.1:{}/config/debug",
        port
    )))
    .unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}

pub async fn skip_field_mutable_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    let response = retry_await!(reqwest::get(&format!(
        "http://127.0.0.1:{}/config/debug",
        port
    )))
    .unwrap();
    assert_eq!(response.status(), 404);

    let response = retry_await!(reqwest::Client::builder()
        .build()
        .unwrap()
        .put(&format!("http://127.0.0.1:{}/config/debug", port))
        .body("true")
        .send())
    .unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}
