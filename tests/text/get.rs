use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::text::*;
use util::*;

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

    check_get_field_text!(port, b, "true");
    check_get_field_text!(port, c, "a");
    check_get_field_text!(port, int128, "-1");
    check_get_field_text!(port, int16, "-2");
    check_get_field_text!(port, int32, "-3");
    check_get_field_text!(port, int64, "-4");
    check_get_field_text!(port, int8, "-5");
    check_get_field_text!(port, intsize, "-6");
    check_get_field_text!(port, uint128, "1");
    check_get_field_text!(port, uint16, "2");
    check_get_field_text!(port, uint32, "3");
    check_get_field_text!(port, uint64, "4");
    check_get_field_text!(port, uint8, "5");
    check_get_field_text!(port, uintsize, "6");
    check_get_field_text!(port, float, "5.5");
    check_get_field_text!(port, double, "3.2");

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

    check_get_field_text!(port, string, "blabla");

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

    check_get_field_text!(port, character, "a");
    check_get_field_text!(port, empty, "");

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
