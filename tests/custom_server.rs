use choices::warp::Filter;
use lazy_static::lazy_static;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use util::*;

async fn get_all_impl<F>(port: u16, server_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.spawn(server_future);

    check_get_text!(port, "config/debug", "true");
    check_get_text!(port, "hello", "Hello!");

    rt.shutdown_background();
}

#[tokio::test]
async fn get_all() {
    let port = get_free_port!();
    get_all_impl(port, async move {
        let routes = text::SimpleBoolConfig { debug: true }
            .filter()
            .or(choices::warp::path("hello").map(|| "Hello!"));
        choices::warp::serve(routes)
            .run((std::net::Ipv4Addr::LOCALHOST, port))
            .await
    })
    .await;
}

#[tokio::test]
async fn get_all_mutable() {
    let port = get_free_port!();
    get_all_impl(port, async move {
        lazy_static! {
            static ref CONFIG: Arc<Mutex<text::SimpleBoolConfig>> =
                Arc::new(Mutex::new(text::SimpleBoolConfig { debug: true }));
        }
        let routes = text::SimpleBoolConfig::filter_mutable(CONFIG.clone())
            .or(choices::warp::path("hello").map(|| "Hello!"));
        choices::warp::serve(routes)
            .run((std::net::Ipv4Addr::LOCALHOST, port))
            .await
    })
    .await;
}
