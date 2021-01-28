use choices::Choices;
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[tokio::test]
async fn get_list() {
    let port = file_line_port!();

    #[derive(Choices)]
    #[choices(path = "myconfig")]
    struct Config {
        debug: bool,
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(async move { Config { debug: true }.run(([127, 0, 0, 1], port)).await });

    check_get!(port, "myconfig/debug", "true");

    rt.shutdown_background();
}
