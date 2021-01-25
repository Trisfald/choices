use choices::Choices;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[tokio::test]
async fn get_non_existing_field() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {}

    let rt = Runtime::new().unwrap();
    rt.spawn(async move { Config {}.run(([127, 0, 0, 1], port)).await });

    let response = retry_await!(reqwest::get(&format!(
        "http://127.0.0.1:{}/config/fake",
        port
    )))
    .unwrap();

    assert_eq!(response.status(), 404);

    rt.shutdown_background();
}

#[tokio::test]
async fn get_scalar_type() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {
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

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        Config {
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
        .run(([127, 0, 0, 1], port))
        .await
    });

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
async fn get_string_field() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {
        string: String,
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Config = {
                Config {
                    string: "blabla".to_string(),
                }
            };
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    });

    check_get_field!(port, string, "blabla");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_option_field() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {
        character: Option<char>,
        empty: Option<bool>,
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        Config {
            character: Some('a'),
            empty: None,
        }
        .run(([127, 0, 0, 1], port))
        .await
    });

    check_get_field!(port, character, "Some(a)");
    check_get_field!(port, empty, "None");

    rt.shutdown_background();
}
