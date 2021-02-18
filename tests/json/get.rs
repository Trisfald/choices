use choices::Choices;
use lazy_static::lazy_static;
use serde_json::json;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::json::*;
use util::*;

#[derive(Choices)]
#[choices(json)]
struct SimpleBoolConfig {
    debug: bool,
}

#[tokio::test]
async fn get_non_existing_field() {
    let port = get_free_port!();
    get_non_existing_field_impl(port, async move {
        SimpleBoolConfig { debug: true }
            .run((std::net::Ipv4Addr::LOCALHOST, port))
            .await
    })
    .await;
}

#[tokio::test]
async fn get_non_existing_field_mutable() {
    let port = get_free_port!();
    get_non_existing_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<SimpleBoolConfig>> =
                Arc::new(Mutex::new(SimpleBoolConfig { debug: true }));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

async fn get_scalar_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field_json!(port, b, "true");
    check_get_field_json!(port, c, json!("a").to_string());
    check_get_field_json!(port, int128, "-1");
    check_get_field_json!(port, int16, "-2");
    check_get_field_json!(port, int32, "-3");
    check_get_field_json!(port, int64, "-4");
    check_get_field_json!(port, int8, "-5");
    check_get_field_json!(port, intsize, "-6");
    check_get_field_json!(port, uint128, "1");
    check_get_field_json!(port, uint16, "2");
    check_get_field_json!(port, uint32, "3");
    check_get_field_json!(port, uint64, "4");
    check_get_field_json!(port, uint8, "5");
    check_get_field_json!(port, uintsize, "6");
    check_get_field_json!(port, float, "5.5");
    check_get_field_json!(port, double, "3.2");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_scalar_field() {
    let port = get_free_port!();
    get_scalar_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<ScalarConfig>> = Arc::new(Mutex::new(ScalarConfig::new()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

#[tokio::test]
async fn get_scalar_field_mutable() {
    let port = get_free_port!();
    get_scalar_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: ScalarConfig = ScalarConfig::new();
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

async fn get_string_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field_json!(port, string, json!("blabla").to_string());

    rt.shutdown_background();
}

#[tokio::test]
async fn get_string_field() {
    let port = get_free_port!();
    get_string_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: StringConfig = {
                StringConfig {
                    string: "blabla".to_string(),
                }
            };
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

#[tokio::test]
async fn get_string_field_mutable() {
    let port = get_free_port!();
    get_string_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<StringConfig>> = {
                Arc::new(Mutex::new(StringConfig {
                    string: "blabla".to_string(),
                }))
            };
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

async fn get_option_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field_json!(port, character, json!("a").to_string());
    check_get_field_json!(port, empty, "null");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_option_field() {
    let port = get_free_port!();
    get_option_field_impl(port, async move {
        OptionConfig {
            character: Some('a'),
            empty: None,
        }
        .run((std::net::Ipv4Addr::LOCALHOST, port))
        .await
    })
    .await;
}

#[tokio::test]
async fn get_option_field_mutable() {
    let port = get_free_port!();
    get_option_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<OptionConfig>> = {
                Arc::new(Mutex::new(OptionConfig {
                    character: Some('a'),
                    empty: None,
                }))
            };
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

async fn get_vec_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field_json!(port, vector, json!([1, 2, 3]).to_string());

    rt.shutdown_background();
}

#[tokio::test]
async fn get_vec_field() {
    let port = get_free_port!();
    get_vec_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: VecConfig = VecConfig::new();
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}

#[tokio::test]
async fn get_vec_field_mutable() {
    let port = get_free_port!();
    get_vec_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<VecConfig>> = Arc::new(Mutex::new(VecConfig::new()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await;
}
