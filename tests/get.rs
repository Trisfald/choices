use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[derive(Choices)]
struct EmptyConfig {}

async fn get_non_existing_field_impl<F>(port: u16, server_future: F)
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

#[tokio::test]
async fn get_non_existing_field() {
    let port = file_line_port!();
    get_non_existing_field_impl(port, async move {
        EmptyConfig {}.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[tokio::test]
async fn get_non_existing_field_mutable() {
    let port = file_line_port!();
    get_non_existing_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<EmptyConfig>> = Arc::new(Mutex::new(EmptyConfig {}));
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[derive(Choices)]
struct ScalarConfig {
    b: bool,
    c: char,
    int128: i128,
    int16: i16,
    int32: i32,
    int64: i64,
    int8: i8,
    intsize: isize,
    uint128: u128,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint8: u8,
    uintsize: usize,
    float: f32,
    double: f64,
}

impl ScalarConfig {
    fn new() -> Self {
        ScalarConfig {
            b: true,
            c: 'a',
            int128: -1,
            int16: -2,
            int32: -3,
            int64: -4,
            int8: -5,
            intsize: -6,
            uint128: 1,
            uint16: 2,
            uint32: 3,
            uint64: 4,
            uint8: 5,
            uintsize: 6,
            float: 5.5,
            double: 3.2,
        }
    }
}

async fn get_scalar_type_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field!(port, b, "true");
    check_get_field!(port, c, "a");
    check_get_field!(port, int128, "-1");
    check_get_field!(port, int16, "-2");
    check_get_field!(port, int32, "-3");
    check_get_field!(port, int64, "-4");
    check_get_field!(port, int8, "-5");
    check_get_field!(port, intsize, "-6");
    check_get_field!(port, uint128, "1");
    check_get_field!(port, uint16, "2");
    check_get_field!(port, uint32, "3");
    check_get_field!(port, uint64, "4");
    check_get_field!(port, uint8, "5");
    check_get_field!(port, uintsize, "6");
    check_get_field!(port, float, "5.5");
    check_get_field!(port, double, "3.2");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_scalar_type() {
    let port = file_line_port!();
    get_scalar_type_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<ScalarConfig>> = Arc::new(Mutex::new(ScalarConfig::new()));
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[tokio::test]
async fn get_scalar_type_mutable() {
    let port = file_line_port!();
    get_scalar_type_impl(port, async move {
        lazy_static! {
            static ref CONFIG: ScalarConfig = ScalarConfig::new();
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[derive(Choices)]
struct StringConfig {
    string: String,
}

async fn get_string_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field!(port, string, "blabla");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_string_field() {
    let port = file_line_port!();
    get_string_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: StringConfig = {
                StringConfig {
                    string: "blabla".to_string(),
                }
            };
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[tokio::test]
async fn get_string_field_mutable() {
    let port = file_line_port!();
    get_string_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<StringConfig>> = {
                Arc::new(Mutex::new(StringConfig {
                    string: "blabla".to_string(),
                }))
            };
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[derive(Choices)]
struct OptionConfig {
    character: Option<char>,
    empty: Option<bool>,
}

async fn get_option_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_field!(port, character, "Some(a)");
    check_get_field!(port, empty, "None");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_option_field() {
    let port = file_line_port!();
    get_option_field_impl(port, async move {
        OptionConfig {
            character: Some('a'),
            empty: None,
        }
        .run(([127, 0, 0, 1], port))
        .await
    })
    .await;
}

#[tokio::test]
async fn get_option_field_mutable() {
    let port = file_line_port!();
    get_option_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<OptionConfig>> = {
                Arc::new(Mutex::new(OptionConfig {
                    character: Some('a'),
                    empty: None,
                }))
            };
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}
