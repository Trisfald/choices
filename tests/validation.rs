use choices::{Choices, ChoicesError, ChoicesResult};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
struct Config {
    #[choices(validator = check_value)]
    value: i32,
}

const GOOD_VALUE: i32 = 1;
const BAD_VALUE: i32 = -1;

fn check_value(v: &i32) -> ChoicesResult<()> {
    if *v >= 0 {
        Ok(())
    } else {
        Err(ChoicesError::ValidationError("".to_string()))
    }
}

#[test]
fn validation_for_setter() {
    let mut config = Config::default();
    assert!(config.set_value(BAD_VALUE).is_err());
    assert_eq!(config.value, 0);
    assert!(config.set_value(GOOD_VALUE).is_ok());
    assert_eq!(config.value, GOOD_VALUE);
}

#[tokio::test]
async fn validation_for_put() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field_text!(
        port,
        value,
        format!("{}", GOOD_VALUE),
        200,
        format!("{}", GOOD_VALUE)
    );
    check_put_field_text!(
        port,
        value,
        format!("{}", BAD_VALUE),
        400,
        format!("{}", GOOD_VALUE)
    );

    rt.shutdown_background();
}
