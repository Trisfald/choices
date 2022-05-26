use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

#[derive(Choices)]
#[choices(message = "welcome")]
struct Config {}

async fn check_message_impl<F>(port: u16, server_future: F, expected: &str)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_text!(port, "config", expected);

    rt.shutdown_background();
}

#[tokio::test]
async fn check_message() {
    let port = get_free_port!();
    check_message_impl(
        port,
        async move { Config {}.run((std::net::Ipv4Addr::LOCALHOST, port)).await },
        "welcome\n",
    )
    .await;
}

#[tokio::test]
async fn check_message_mutable() {
    let port = get_free_port!();
    check_message_impl(
        port,
        async move {
            lazy_static! {
                static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {}));
            }
            CONFIG.run((std::net::Ipv4Addr::LOCALHOST, port)).await
        },
        "welcome\n",
    )
    .await;
}

#[derive(Choices)]
#[choices(message = "")]
struct ConfigMsgEmpty {}

#[tokio::test]
async fn check_message_empty() {
    let port = get_free_port!();
    check_message_impl(
        port,
        async move {
            ConfigMsgEmpty {}
                .run((std::net::Ipv4Addr::LOCALHOST, port))
                .await
        },
        "",
    )
    .await;
}
