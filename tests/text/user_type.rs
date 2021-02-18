use choices::bytes::Bytes;
use choices::{Choices, ChoicesInput, ChoicesOutput, ChoicesResult};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Default)]
struct UserType<T: Default, U: Default> {
    _t: T,
    _u: U,
}

impl<T: Default, U: Default> ChoicesInput<'_> for UserType<T, U> {
    fn from_chars(_: &Bytes) -> ChoicesResult<Self> {
        Ok(Self::default())
    }
}

impl<T: Default, U: Default> ChoicesOutput for UserType<T, U> {
    fn body_string(&self) -> String {
        "ok".to_string()
    }
}

#[derive(Choices, Default)]
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
        "Available configuration options:\n  - field: UserType<u8, u8>\n"
    );

    check_get_field_text!(port, field, "ok");

    rt.shutdown_background();
}
