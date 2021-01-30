use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[derive(Choices)]
#[choices(path = "myconfig")]
struct Config {
    debug: bool,
}

async fn get_field_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get!(port, "myconfig/debug", "true");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_field() {
    let port = file_line_port!();
    get_field_impl(port, async move {
        Config { debug: true }.run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[tokio::test]
async fn get_field_mutable() {
    let port = file_line_port!();
    get_field_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config { debug: true }));
        }
        CONFIG.run(([127, 0, 0, 1], port)).await
    })
    .await;
}
