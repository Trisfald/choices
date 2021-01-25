use choices::Choices;
use tokio::runtime::Runtime;
use warp::Filter;

#[macro_use]
mod util;

#[tokio::test]
async fn get_list() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {
        debug: bool,
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        let routes = Config { debug: true }
            .filter()
            .or(warp::path("hello").map(|| "Hello!"));
        warp::serve(routes).run(([127, 0, 0, 1], port)).await
    });

    check_get!(port, "config/debug", "true");
    check_get!(port, "hello", "Hello!");

    rt.shutdown_background();
}
