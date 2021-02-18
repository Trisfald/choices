use choices::Choices;
use lazy_static::lazy_static;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::json::*;
use util::*;

/// Performs a PUT for the field `name` on a server running on localhost on port
/// `port` and checks the response's status code. Then, it performs a GET to
/// verify the field's value corresponds to `expected`.
macro_rules! check_put_field {
    ($port:expr, $name:expr, $body:expr, $status:expr, $expected:expr $( , $headers:expr )* ) => {
        let response = retry_await!(reqwest::Client::builder()
            .build()
            .unwrap()
            .put(&format!(
                "http://127.0.0.1:{}/{}",
                $port,
                concat!("config/", stringify!($name))
            ))
            .body($body)
            $(.header($headers.0, $headers.1)),*
            .send())
        .unwrap();
        assert_eq!(response.status(), $status);
        check_get_field_json!($port, $name, $expected);
    };
}

#[derive(Choices)]
struct SimpleConfig {
    debug: bool,
}

#[tokio::test]
async fn put_non_existing_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<SimpleConfig>> =
                Arc::new(Mutex::new(SimpleConfig { debug: true }));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    let response = retry_await!(reqwest::Client::builder()
        .build()
        .unwrap()
        .put(&format!("http://127.0.0.1:{}/config/fake", port))
        .send())
    .unwrap();
    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}

#[tokio::test]
async fn put_scalar_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<ScalarConfig>> =
                Arc::new(Mutex::new(ScalarConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    macro_rules! check_numeric_field {
        ($name:expr, $body:expr) => {
            check_put_field!(port, $name, $body, 200, $body);
            check_put_field!(port, $name, "wrong", 400, $body);
        };
    }

    // bool
    check_put_field!(port, b, "true", 200, "true");
    check_put_field!(port, b, "wrong", 400, "true");
    // char
    check_put_field!(port, c, json!("a").to_string(), 200, json!("a").to_string());
    check_put_field!(port, c, "", 411, json!("a").to_string());
    // integers
    check_numeric_field!(int128, "-1");
    check_numeric_field!(int16, "-2");
    check_numeric_field!(int32, "-3");
    check_numeric_field!(int64, "-4");
    check_numeric_field!(int8, "-5");
    check_numeric_field!(intsize, "-6");
    check_numeric_field!(uint128, "1");
    check_numeric_field!(uint16, "2");
    check_numeric_field!(uint32, "3");
    check_numeric_field!(uint64, "4");
    check_numeric_field!(uint8, "5");
    check_numeric_field!(uintsize, "6");
    // floating points
    check_numeric_field!(float, "5.5");
    check_numeric_field!(float, "3.2");

    rt.shutdown_background();
}

#[tokio::test]
async fn put_string_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<StringConfig>> =
                Arc::new(Mutex::new(StringConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field!(
        port,
        string,
        json!("blabla").to_string(),
        200,
        json!("blabla").to_string()
    );
    check_put_field!(
        port,
        string,
        json!("").to_string(),
        200,
        json!("").to_string()
    );

    rt.shutdown_background();
}

#[tokio::test]
async fn put_option_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<OptionConfig>> =
                Arc::new(Mutex::new(OptionConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field!(
        port,
        character,
        json!("a").to_string(),
        200,
        json!("a").to_string()
    );
    check_put_field!(port, character, "null", 200, "null");
    check_put_field!(port, empty, "true", 200, "true");
    check_put_field!(port, empty, "wrong", 400, "true");

    rt.shutdown_background();
}

#[tokio::test]
async fn put_vec_field() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<VecConfig>> = Arc::new(Mutex::new(VecConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field!(
        port,
        vector,
        json!([0, 1, 2]).to_string(),
        200,
        json!([0, 1, 2]).to_string()
    );

    rt.shutdown_background();
}
