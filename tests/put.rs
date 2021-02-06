use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[derive(Choices)]
struct SimpleConfig {
    debug: bool,
}

#[tokio::test]
async fn put_non_existing_field_mutable() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<SimpleConfig>> =
                Arc::new(Mutex::new(SimpleConfig { debug: true }));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    let response = retry_await!(reqwest::Client::builder().build().unwrap().put(&format!(
        "http://127.0.0.1:{}/config/fake",
        port
    )).send())
    .unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}
