use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices, Default)]
pub struct SkipConfig {
    #[choices(skip)]
    pub debug: bool,
}

#[tokio::test]
async fn skip_field_in_list() {
    let port = get_free_port!();

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<SkipConfig>> = Arc::new(Mutex::new(SkipConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    });

    check_get!(
        port,
        "config",
        "Available configuration options:\n",
        util::CONTENT_TYPE_TEXT
    );

    rt.shutdown_background();
}

#[tokio::test]
async fn skip_field() {
    let port = get_free_port!();
    skip_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: SkipConfig = SkipConfig::default();
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await
}

#[tokio::test]
async fn skip_field_mutable() {
    let port = get_free_port!();
    skip_field_mutable_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<SkipConfig>> = Arc::new(Mutex::new(SkipConfig::default()));
        }
        CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
    })
    .await
}
