use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
struct Config {
    #[choices(on_set = (|v| self.total += v))]
    value: i32,
    total: i32,
}

#[test]
fn on_set_for_setter() {
    let mut config = Config::default();
    config.set_value(2);
    config.set_value(3);
    assert_eq!(config.value, 3);
    assert_eq!(config.total, 5);
}

#[tokio::test]
async fn on_set_for_put() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_put_field_text!(port, value, "2", 200, "2");
    check_put_field_text!(port, value, "3", 200, "3");
    check_get_field_text!(port, total, "5");

    rt.shutdown_background();
}
