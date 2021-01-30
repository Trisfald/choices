use choices::Choices;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::Filter;

#[macro_use]
mod util;

#[derive(Choices)]
struct Config {
    debug: bool,
}

async fn get_all_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get!(port, "config/debug", "true");
    check_get!(port, "hello", "Hello!");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_all() {
    let port = file_line_port!();
    get_all_impl(port, async move {
        let routes = Config { debug: true }
            .filter()
            .or(warp::path("hello").map(|| "Hello!"));
        warp::serve(routes).run(([127, 0, 0, 1], port)).await
    })
    .await;
}

#[tokio::test]
async fn get_all_mutable() {
    let port = file_line_port!();
    get_all_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config { debug: true }));
        }
        let routes =
            Config::filter_mutable(CONFIG.clone()).or(warp::path("hello").map(|| "Hello!"));
        warp::serve(routes).run(([127, 0, 0, 1], port)).await
    })
    .await;
}
