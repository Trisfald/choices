use choices::Choices;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Default, Serialize, Deserialize)]
struct UserType<T: Default, U: Default> {
    _t: T,
    _u: U,
}

#[derive(Choices, Default)]
#[choices(json)]
struct Config {
    field: UserType<u8, u8>,
}

#[tokio::test]
async fn user_type() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    let response =
        retry_await!(reqwest::get(&format!("http://127.0.0.1:{}/config", port))).unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert_eq!(
        body,
        json!([{
            "name": "field",
            "type": "UserType<u8, u8>",
        }])
        .to_string()
    );

    check_get_field_json!(port, field, json!({"_t": 0, "_u": 0}).to_string());

    rt.shutdown_background();
}
